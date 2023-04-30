use glad_gl::gl;

use crate::shader::Shader;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,

    vao: u32,
    vbo: u32, 
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);
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
            vao,
            vbo,
            ebo,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        let mut diffuse = 1;
        let mut specular = 1;

        for i in 0..self.textures.len() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                let name = self.textures[i].typ.as_str();

                if name == "texture_diffuse" {
                    diffuse += 1;
                } else if name == "texture_specular" {
                    specular += 1;
                }

                shader.set_float(format!("material.{}{}", name, if name == "texture_diffuse" { diffuse } else { specular }).as_str(), i as f32);
                gl::BindTexture(gl::TEXTURE_2D, self.textures[i].id);
            }
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);

            // draw Mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, self.indices.as_ptr() as *const std::ffi::c_void);
            gl::BindVertexArray(0);
        }
    }
}

pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

pub struct Texture {
    pub id: u32,
    pub typ: String,
}
