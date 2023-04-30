use crate::{mesh::Mesh, shader::Shader};

pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(path: &str) -> Model {

        // TODO: 
        Model {

        }
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
