use std::{collections::HashMap, sync::Arc};

use super::{engine::Vector3d, entities::Texture};

/// Holds lighting characteristics of a surface which we can use
/// to determine the colour of a point on the surface.
/// Note this is a small subset of values found in .mtl and so some mtl values
/// Will be dropped if not in this struct.
#[derive(Debug, PartialEq)]

pub struct Material {
    pub name: String,
    /// The three below coefficients should be somewhere between { 0.0, 0.0, 0.0 } and { 1.0, 1.0, 1.0}
    /// They are used to weight the R, G, B values sampled from the texture.
    pub ambient_color_coefficient: Vector3d, // Ka
    pub diffuse_color_coefficient: Vector3d,  // Kd
    pub specular_color_coefficient: Vector3d, // Ks
    pub specular_weight: f64,                 // Ns
    pub texture: Arc<Texture>, // map_Kd, will also be used for map Ka and Ks for the time being
    pub bump_map: Option<Arc<Texture>>, // map_bump not part of mtl standard but is used unofficially, apparently mtl predates bump/normal maps
}

#[derive(Debug, PartialEq)]
pub struct MaterialMap {
    pub textures: Vec<Arc<Texture>>,
    pub materials: HashMap<String, Arc<Material>>,
}
