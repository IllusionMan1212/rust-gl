use crate::{mesh::{Mesh, Vertex, Texture}, shader::Shader};
use russimp;
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
}

fn process_node<'a>(
    node: &russimp::node::Node,
    scene: &'a russimp::scene::Scene,
    meshes: &mut Vec<Mesh>,
    dir: &std::path::PathBuf,
    loaded_textures: &mut Vec<Texture>
) {
    for i in 0..node.meshes.len() {
        let mesh = &scene.meshes[node.meshes[i] as usize];
        meshes.push(process_mesh(mesh, scene, dir, loaded_textures));
    }

    for child in node.children.borrow().clone().into_iter() {
        process_node(&child, scene, meshes, dir, loaded_textures);
    }
}

fn process_mesh(
    mesh: &russimp::mesh::Mesh,
    scene: &russimp::scene::Scene,
    dir: &std::path::PathBuf,
    loaded_textures: &mut Vec<Texture>
) -> Mesh {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut textures = vec![];

    for i in 0..mesh.vertices.len() {
        let pos = glm::vec3(mesh.vertices[i].x, mesh.vertices[i].y, mesh.vertices[i].z);

        let norm = match mesh.normals.len() {
            0 => glm::vec3(0.0, 0.0, 0.0),
            _ => glm::vec3(mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z),
        };

        let t = &mesh.texture_coords[0];
        let tex_coords = match t {
            Some(tex) => {
                glm::vec2(tex[i].x, tex[i].y)
            },
            None => glm::vec2(0.0, 0.0)
        };

        vertices.push(Vertex::new(pos, norm, tex_coords));
    }

    for i in 0..mesh.faces.len() {
        for j in 0..mesh.faces[i].0.len() {
            indices.push(mesh.faces[i].0[j]);
        }
    }

    println!("material count: {}", scene.materials.len());
    println!("material index: {}", mesh.material_index);
    let mat = &scene.materials[mesh.material_index as usize];
    println!("{:?}", mat);

    let mut diffuse_maps = load_material_textures(mat, russimp::material::TextureType::Diffuse, "texture_diffuse", dir, loaded_textures);
    textures.append(&mut diffuse_maps);
    let mut specular_maps = load_material_textures(mat, russimp::material::TextureType::Specular, "texture_specular", dir, loaded_textures);
    textures.append(&mut specular_maps);

    return Mesh::new(vertices, indices, textures);
}

fn load_material_textures(
    mat: &russimp::material::Material,
    tex_type: russimp::material::TextureType,
    type_name: &str,
    dir: &std::path::PathBuf,
    loaded_textures: &mut Vec<Texture>
) -> Vec<Texture> {
    let mut textures = vec![];

    println!("there exists '{}' textures in this material", mat.textures.len());

    for (typ, tex) in mat.textures.iter() {
        if *typ == tex_type {
            let texture = tex.borrow();
            let mut skip = false;
            let tex_filename = &texture.filename;
            println!("texture filename: {}", tex_filename);
            let path = dir.join(tex_filename);

            for loaded_tex in &mut *loaded_textures {
                if loaded_tex.path == path {
                    textures.push(loaded_tex.clone());
                    skip = true;
                    break;
                }
            }

            if !skip {
                let texture = Texture::new(path, type_name);
                loaded_textures.push(texture.clone());
                textures.push(texture);
            }
        }
    }

    return textures;
}

impl Model {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>>  {
        let scene = russimp::scene::Scene::from_file(path,
            vec![
            russimp::scene::PostProcess::Triangulate,
            russimp::scene::PostProcess::FlipUVs,
            ]).with_context(|| format!("Failed to load model at {}", path))?;

        let root_node = match &scene.root {
            Some(root) => root,
            None => return Err("Scene has no root node".into()),
        };

        if scene.flags & russimp::sys::AI_SCENE_FLAGS_INCOMPLETE == 1 {
            return Err("Scene is incomplete")?;
        }

        // TODO: don't just explode here
        let directory = std::path::Path::new(path).parent().unwrap().to_path_buf();

        let mut loaded_textures = vec![];
        let mut meshes = vec![];
        process_node(&root_node, &scene, &mut meshes, &directory, &mut loaded_textures);

        Ok(Model {
            meshes,
        })
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
