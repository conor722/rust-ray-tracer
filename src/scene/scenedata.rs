use crate::collision::octree::Octree;

use super::{engine::Vector3d, entities::Triangle, material::MaterialMap};

#[derive(Debug, PartialEq)]
pub struct SceneData {
    pub triangles: Vec<Triangle>,
    pub vertices: Vec<Vector3d>,
    pub vertex_texture_coords: Vec<Vector3d>,
    pub vertex_normal_coords: Vec<Vector3d>,
    pub material_map: MaterialMap,
    pub octree: Octree,
}
