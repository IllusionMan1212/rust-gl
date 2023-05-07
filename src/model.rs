use crate::{mesh::{Mesh, Vertex, Texture, Material}, shader::Shader, utils};
use russimp;
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
}

fn process_node<'a>(
    node: &russimp::node::Node,
    scene: &'a russimp::scene::Scene,
    meshes: &mut Vec<Mesh>,
    dir: &std::path::PathBuf,
    loaded_textures: &mut Vec<Texture>,
    init_trans: &glm::Mat4,
) {
    let node_trans = glm::mat4(
        node.transformation.a1, node.transformation.a2, node.transformation.a3, node.transformation.a4,
        node.transformation.b1, node.transformation.b2, node.transformation.b3, node.transformation.b4,
        node.transformation.c1, node.transformation.c2, node.transformation.c3, node.transformation.c4,
        node.transformation.d1, node.transformation.d2, node.transformation.d3, node.transformation.d4,
    );
    let mut new_trans = *init_trans * node_trans;

    // println!("node: {}", node.name);
    // println!("{:#?}", node_trans);
    // println!("{:#?}", node.metadata);
    // println!("{:#?}", node.transformation);

    for i in 0..node.meshes.len() {
        let mesh = &scene.meshes[node.meshes[i] as usize];
        meshes.push(process_mesh(mesh, scene, dir, loaded_textures, &mut new_trans));
    }

    for child in node.children.borrow().clone().into_iter() {
        process_node(&child, scene, meshes, dir, loaded_textures, &node_trans);
    }
}

fn process_mesh(
    mesh: &russimp::mesh::Mesh,
    scene: &russimp::scene::Scene,
    dir: &std::path::PathBuf,
    loaded_textures: &mut Vec<Texture>,
    transformation: &mut glm::Mat4,
) -> Mesh {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut textures = vec![];

    for i in 0..mesh.vertices.len() {
        let pos = glm::vec4(mesh.vertices[i].x, mesh.vertices[i].y, mesh.vertices[i].z, 1.0);
        // let pos = *transformation * pos;

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

        vertices.push(Vertex::new(pos.truncate(3), norm, tex_coords));
    }

    for i in 0..mesh.faces.len() {
        for j in 0..mesh.faces[i].0.len() {
            indices.push(mesh.faces[i].0[j]);
        }
    }

    // println!("material count: {}", scene.materials.len());
    // println!("material index: {}", mesh.material_index);
    let mat = &scene.materials[mesh.material_index as usize];
    // println!("{:?}", mat);

    let material = process_material(mat);

    let mut diffuse_maps = load_material_textures(mat, russimp::material::TextureType::Diffuse, "texture_diffuse", dir, loaded_textures);
    textures.append(&mut diffuse_maps);
    let mut specular_maps = load_material_textures(mat, russimp::material::TextureType::Specular, "texture_specular", dir, loaded_textures);
    textures.append(&mut specular_maps);


    return Mesh::new(mesh.name.as_str(), vertices, indices, textures, material, transformation);
}

fn process_material(mat: &russimp::material::Material) -> Material {
    let mut mat_name = String::from("Default_Mat");
    let mut ambient = glm::vec3(0.2, 0.2, 0.2);
    let mut diffuse = glm::vec3(0.7, 0.7, 0.7);
    let mut specular = glm::vec3(0.1, 0.1, 0.1);
    let mut shininess = 32.0;

    // TODO: better way of mapping properties
    for property in mat.properties.iter() {
        match property.key.as_str() {
            "$clr.ambient" => {
                ambient = match &property.data {
                    russimp::material::PropertyTypeInfo::FloatArray(a) => {
                        glm::vec3(a[0], a[1], a[2])
                    },
                    _ => panic!("Property should not be this type: {}", property.key)
                };
            },
            "$clr.diffuse" => {
                diffuse = match &property.data {
                    russimp::material::PropertyTypeInfo::FloatArray(a) => {
                        glm::vec3(a[0], a[1], a[2])
                    },
                    _ => panic!("Property should not be this type: {}", property.key)
                };
            },
            "$clr.specular" => {
                specular = match &property.data {
                    russimp::material::PropertyTypeInfo::FloatArray(a) => {
                        glm::vec3(a[0], a[1], a[2])
                    },
                    _ => panic!("Property should not be this type: {}", property.key)
                }
            }
            "?mat.name" => {
                mat_name = match &property.data {
                    russimp::material::PropertyTypeInfo::String(s) => {
                        s.to_string()
                    },
                    _ => panic!("Property should not be this type: {}", property.key)
                };
            }
            "$mat.shininess" => {
                shininess = match &property.data {
                    russimp::material::PropertyTypeInfo::FloatArray(a) => {
                        a[0]
                    },
                    _ => panic!("Property should not be this type: {}", property.key)
                };
            }
            _ => {},
        }
    }

    Material::new(mat_name, ambient, diffuse, specular, shininess)
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
            // HACK: fix this
            if tex_filename.is_empty() {
                continue;
            }
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

        // TODO: handle fbx metadata that contains the indices and values of the proper axes
        // reference: https://github.com/assimp/assimp/issues/849#issuecomment-538982013
        println!("scene metadata {:#?}", scene.metadata);
        println!("root metadata {:#?}", root_node.metadata);

        let mut loaded_textures = vec![];
        let mut meshes = vec![];
        let init_trans_mat = utils::mat_ident();
        process_node(&root_node, &scene, &mut meshes, &directory, &mut loaded_textures, &init_trans_mat);

        Ok(Model {
            name: root_node.name.to_owned(),
            meshes,
        })
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}
