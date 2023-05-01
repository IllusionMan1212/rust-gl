use glad_gl::gl;

use crate::{shader::Shader, utils};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub material: Material,

    vao: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>, material: Material) -> Mesh {
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

        Mesh {
            vertices,
            indices,
            textures,
            material,
            vao,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        let mut diffuse = 1;
        let mut specular = 1;

        shader.use_shader();
        shader.set_3fv("material.ambient", self.material.ambient);
        shader.set_3fv("material.diffuse", self.material.diffuse);
        shader.set_3fv("material.specular", self.material.specular);
        // TODO: normalize the shininess value (32.0 as default for now because broken values cause
        // the model to be black or rather cause the light to just be absorbed and barely reflected)
        shader.set_float("material.shininess", 32.0);

        for i in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                let name = self.textures[i].typ.as_str();

                if name == "texture_diffuse" {
                    diffuse += 1;
                } else if name == "texture_specular" {
                    specular += 1;
                }

                shader.set_int(format!("material.{}{}", name, if name == "texture_diffuse" { diffuse } else { specular }).as_str(), i as i32);
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
    pub typ: String,
    pub path: std::path::PathBuf,
}

impl Texture {
    pub fn new(path: std::path::PathBuf, type_name: &str) -> Self {
        // TODO: don't just explode here
        let id = utils::load_texture(path.to_str().unwrap()).unwrap();

        Texture {
            id,
            typ: type_name.to_string(),
            path,
        }
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
