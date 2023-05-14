use glad_gl::gl;
use anyhow::Result;

use crate::{shader::Shader, utils};

fn decompose_mat(matrix: &mut glm::Mat4) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
    let pos = glm::vec3(matrix.c0.w, matrix.c1.w, matrix.c2.w);
    matrix.c3.x = 0.0;
    matrix.c3.y = 0.0;
    matrix.c3.z = 0.0;

    let scale_x = glm::length(glm::vec3(matrix.c0.x, matrix.c1.x, matrix.c2.x));
    let scale_y = glm::length(glm::vec3(matrix.c0.y, matrix.c1.y, matrix.c2.y));
    let scale_z = glm::length(glm::vec3(matrix.c0.z, matrix.c1.z, matrix.c2.z));
    let scale = glm::vec3(scale_x, scale_y, scale_z);

    let matrix = glm::mat3(
        matrix[0][0] / scale_x, matrix[0][1] / scale_y, matrix[0][2] / scale_z,
        matrix[1][0] / scale_x, matrix[1][1] / scale_y, matrix[1][2] / scale_z,
        matrix[2][0] / scale_x, matrix[2][1] / scale_y, matrix[2][2] / scale_z,
    );

    // println!("original rot: {:#?}", matrix);

    let roll = matrix[1][0].atan2(matrix[0][0]).to_degrees();
    let yaw = (-matrix[2][0]).atan2((matrix[2][1].powi(2) + matrix[2][2].powi(2)).sqrt()).to_degrees();
    let pitch = matrix[2][1].atan2(matrix[2][2]).to_degrees();

    let rotation = glm::vec3(pitch, yaw, roll);

    (pos, rotation, scale)
}

fn create_rotation_matrix(pitch: f32, yaw: f32, roll: f32) -> glm::Mat3 {
    let pitch = pitch.to_radians();
    let yaw = yaw.to_radians();
    let roll = roll.to_radians();

    let pitch_matrix = glm::mat3(
        1.0, 0.0, 0.0,
        0.0, pitch.cos(), -pitch.sin(),
        0.0, pitch.sin(), pitch.cos()
    );

    let yaw_matrix = glm::mat3(
        yaw.cos(), 0.0, yaw.sin(),
        0.0, 1.0, 0.0,
        -yaw.sin(), 0.0, yaw.cos()
    );

    let roll_matrix = glm::mat3(
        roll.cos(), -roll.sin(), 0.0,
        roll.sin(), roll.cos(), 0.0,
        0.0, 0.0, 1.0
    );

    pitch_matrix * yaw_matrix * roll_matrix
}

fn apply_rotation(matrix: &glm::Mat4, rot: glm::Vec3) -> glm::Mat4 {
    let mut temp = utils::mat_ident();
    let rot = create_rotation_matrix(rot.x, rot.y, rot.z);
    // println!("created rot: {:#?}", rot);
    // panic!("used to stop print spam lol")

    for i in 0..3 {
        for j in 0..3 {
            temp[i][j] = rot[j][i];
        }
    }
    return temp * *matrix;
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub material: Material,

    vao: u32,
}

impl Mesh {
    pub fn new(name: &str, vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>, material: Material, transformation: &mut glm::Mat4) -> Mesh {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<Vertex>() * vertices.len() as usize) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<u32>() * indices.len() as usize) as isize, indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as i32, std::ptr::null());

            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as i32, (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void);

            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as i32, (6 * std::mem::size_of::<f32>()) as *const std::ffi::c_void);

            gl::BindVertexArray(0);
        }

        let (position, rotation, scale) = decompose_mat(transformation);

        Mesh {
            name: name.to_string(),
            vertices,
            indices,
            textures,
            material,
            vao,
            position,
            rotation,
            scale,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        let mut diffuse = 1;
        let mut specular = 1;

        shader.use_shader();

        let model_mat = glm::ext::scale(&utils::mat_ident(), self.scale);
        let model_mat = apply_rotation(&model_mat, self.rotation);
        let model_mat = glm::ext::translate(&model_mat, self.position);
        shader.set_mat4fv("model", &model_mat);

        shader.set_3fv("material.ambient", self.material.ambient);
        shader.set_3fv("material.diffuse", self.material.diffuse);
        shader.set_3fv("material.specular", self.material.specular);
        // TODO: normalize the shininess value (32.0 as default for now because broken values cause
        // the model to be black or rather cause the light to just be absorbed and barely reflected)
        shader.set_float("material.shininess", 32.0);

        for i in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                match self.textures[i].typ {
                    russimp::material::TextureType::Diffuse => {
                        diffuse += 1;
                        shader.set_int(format!("material.texture_diffuse{}", diffuse).as_str(), i as i32);
                    },
                    russimp::material::TextureType::Specular => {
                        specular += 1;
                        shader.set_int(format!("material.texture_specular{}", specular).as_str(), i as i32);
                    },
                    _ => {}, // don't do anything because unsupported texture types are logged
                             // when the model is loaded
                }

                gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id);
            }
        }

        unsafe {
            // draw Mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());

            // reset stuff to default
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Clone, Debug)]
#[repr(packed(2))]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

impl Vertex {
    pub fn new(position: glm::Vec3, normal: glm::Vec3, tex_coords: glm::Vec2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub typ: russimp::material::TextureType,
    pub path: std::path::PathBuf,
}

impl Texture {
    pub fn new(path: std::path::PathBuf, typ: russimp::material::TextureType) -> Result<Self, Box<dyn std::error::Error>> {
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return Err("Failed to convert texture path to string".into()),
        };
        let id = utils::load_texture(path_str)?;

        Ok(Texture {
            id,
            typ,
            path,
        })
    }
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
    pub shininess: f32,
}

impl Material {
    pub fn new(name: String, ambient: glm::Vec3, diffuse: glm::Vec3, specular: glm::Vec3, shininess: f32) -> Self {
        Material {
            name,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ambient: {:?}\nDiffuse: {:?}\nSpecular: {:?}\nShininess: {}", self.ambient, self.diffuse, self.specular, self.shininess)
    }
}
