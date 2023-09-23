use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::str::{FromStr, Lines, SplitWhitespace};
use std::sync::Arc;

use crate::collision::octree::Octree;
use crate::scene::engine::Vector3d;
use crate::scene::entities::{Color, Texture, Triangle};
use crate::scene::material::{Material, MaterialMap};
use crate::scene::scenedata::SceneData;

use image::io::Reader as ImageReader;

static DEFAULT_VERTICES: &Vector3d = &Vector3d {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
static MISSING_VERTEX_ERROR_MESSAGE: &str = "No vertex with this index";

pub fn parse_mtl_file_lines<'a>(material_map: &mut MaterialMap, lines: Lines) {
    // name: String,
    // /// The three below coefficients should be somewhere between { 0.0, 0.0, 0.0 } and { 1.0, 1.0, 1.0}
    // /// They are used to weight the R, G, B values sampled from the texture.
    // ambient_color_coefficient: Vector3d, // Ka
    // diffuse_color_coefficient: Vector3d,  // Kd
    // specular_color_coefficient: Vector3d, // Ks
    // specular_weight: f64,                 // Ns
    // texture: &'a Texture, // map_Ka, will also be used for map Kd and Ks for the time being
    // bump_map: &'a Texture, // map_bump not part of mtl standard but is used unofficially, apparently mtl predates bump/normal maps

    let mut name_texture_map: HashMap<String, Arc<Texture>> = HashMap::new();

    let mut name: Option<String> = None;
    let mut ambient_color_coefficient: Option<Vector3d> = None;
    let mut diffuse_color_coefficient: Option<Vector3d> = None;
    let mut specular_color_coefficient: Option<Vector3d> = None;
    let mut specular_weight: Option<f64> = None;
    let mut texture: Option<Arc<Texture>> = None;
    let mut bump_map: Option<Arc<Texture>> = None;

    // Add an END to the end of the iterator to make sure it adds the last material.
    for line in lines.chain(vec!["END"]) {
        let mut split_line = line.split_whitespace();
        let line_type = split_line.next();

        match line_type {
            Some("newmtl" | "END") => {
                if let Some(actual_name) = name {
                    let mat = Material {
                        name: actual_name.to_string(),
                        ambient_color_coefficient: ambient_color_coefficient
                            .unwrap_or(*DEFAULT_VERTICES),
                        diffuse_color_coefficient: diffuse_color_coefficient
                            .unwrap_or(*DEFAULT_VERTICES),
                        specular_color_coefficient: specular_color_coefficient
                            .unwrap_or(*DEFAULT_VERTICES),
                        specular_weight: specular_weight.unwrap_or(240.0),
                        texture: texture.clone().unwrap(),
                        bump_map: bump_map.clone(),
                    };

                    material_map
                        .materials
                        .insert(actual_name.to_string(), Arc::new(mat));
                }

                let next_name = parse_next_value_from_split::<String>(&mut split_line);
                name = next_name.clone();
            }
            Some("map_Ka") => {
                let texture_name: String =
                    parse_next_value_from_split(&mut split_line).expect("Expected a texture name");

                if let Some(tex) = name_texture_map.get(&texture_name.clone()) {
                    texture = Some(Arc::clone(tex));
                } else {
                    let tex = get_texture_from_file_name(texture_name.clone());
                    let t_arc = Arc::new(tex);
                    material_map.textures.push(Arc::clone(&t_arc));
                    name_texture_map.insert(texture_name.clone(), Arc::clone(&t_arc));
                    texture = Some(Arc::clone(&t_arc));
                }
            }
            Some("bump") => {
                let texture_name: String =
                    parse_next_value_from_split(&mut split_line).expect("Expected a texture name");

                if let Some(tex) = name_texture_map.get(&texture_name.clone()) {
                    bump_map = Some(Arc::clone(tex));
                } else {
                    let tex = get_texture_from_file_name(texture_name.clone());
                    let t_arc = Arc::new(tex);
                    material_map.textures.push(Arc::clone(&t_arc));
                    name_texture_map.insert(texture_name.clone(), Arc::clone(&t_arc));
                    bump_map = Some(Arc::clone(&t_arc));
                }
            }
            Some("Ka") => {
                let cc = get_color_coefficient_from_split_lines(&mut split_line);
                ambient_color_coefficient = Some(cc);
            }
            Some("Kd") => {
                let cc = get_color_coefficient_from_split_lines(&mut split_line);
                diffuse_color_coefficient = Some(cc);
            }
            Some("Ks") => {
                let cc = get_color_coefficient_from_split_lines(&mut split_line);
                specular_color_coefficient = Some(cc);
            }
            Some("Ns") => {
                let sw: f64 = parse_next_value_from_split(&mut split_line)
                    .expect("Expected a valid Ns float value");
                specular_weight = Some(sw);
            }
            Some(&_) => {}
            None => {}
        }
    }
}

pub fn parse_obj_file_lines<'a>(lines: Lines) -> SceneData {
    let vertices = Vec::new();
    let triangles = Vec::new();
    let vertex_texture_coords = Vec::new();
    let vertex_normal_coords = Vec::new();

    let octree: Octree = Octree::new(-20.0, 20.0, -20.0, 20.0, -20.0, 20.0);

    let material_map = MaterialMap {
        textures: vec![],
        materials: HashMap::new(),
    };

    let mut scene_data: SceneData = SceneData {
        vertices,
        triangles,
        vertex_texture_coords,
        vertex_normal_coords,
        material_map,
        octree,
    };

    let mut current_material: Option<Arc<Material>> = None;

    for line in lines {
        let mut split_line = line.split_whitespace();
        let line_type = split_line.next();

        match line_type {
            Some("mtllib") => {
                let mtllib_file_name: String =
                    parse_next_value_from_split(&mut split_line).expect("Invalid .mtl file name");
                let mtl_file = fs::read_to_string(mtllib_file_name).expect("Could not read file");
                let mtl_file_lines = mtl_file.lines();

                parse_mtl_file_lines(&mut scene_data.material_map, mtl_file_lines)
            }
            Some("usemtl") => {
                let material_name: String =
                    parse_next_value_from_split(&mut split_line).expect("Invalid material name");

                let m = scene_data
                    .material_map
                    .materials
                    .get(&material_name)
                    .expect("Material not found, is it in your mtl file?");

                current_material = Some(m.clone().clone());
            }
            Some("v") => {
                let v = get_vertex(&mut split_line);
                scene_data.vertices.push(v);
            }
            Some("f") => {
                let cm = Arc::clone(&current_material.clone().unwrap());

                let tri = get_triangle(&mut split_line, &scene_data, cm);
                scene_data.octree.push_triangle(tri.clone());
                scene_data.triangles.push(tri.clone());
            }
            Some("vt") => {
                let vt = get_vertex(&mut split_line);
                scene_data.vertex_texture_coords.push(vt);
            }
            Some("vn") => {
                let vn = get_vertex(&mut split_line);
                scene_data.vertex_normal_coords.push(vn);
            }
            Some(&_) => {}
            None => {}
        }
    }

    scene_data
}

fn parse_next_value_from_split<'a, T: FromStr>(
    line: &mut impl Iterator<Item = &'a str>,
) -> Option<T>
where
    <T as FromStr>::Err: Debug,
{
    if let Some(r) = line.next() {
        return Some(r.parse::<T>().expect("Could not parse value"));
    } else {
        return None;
    }
}

fn get_vertex(mut line: &mut SplitWhitespace<'_>) -> Vector3d {
    let x: f64 = parse_next_value_from_split(&mut line).expect("Cannot parse vertex");
    let y: f64 = parse_next_value_from_split(&mut line).expect("Cannot parse vertex");
    let z: f64 = parse_next_value_from_split(&mut line).unwrap_or(0.0);

    return Vector3d { x, y, z };
}

fn get_vertex_attributes<'a>(line: &str) -> (usize, Option<usize>, Option<usize>) {
    let mut line_split = line.split("/");

    let vertex_attribute_collection: String =
        parse_next_value_from_split(&mut line_split).expect("No attribute collection found");
    let mut vertex_attribute_split = vertex_attribute_collection.split("/");

    let index: usize = parse_next_value_from_split(&mut vertex_attribute_split)
        .expect("No index found in attribute collection");

    let tex_coord_index = parse_next_value_from_split::<usize>(&mut line_split);

    let normal_coord_index = parse_next_value_from_split::<usize>(&mut line_split);

    return (index, tex_coord_index, normal_coord_index);
}

fn get_triangle<'a>(
    line: &'a mut SplitWhitespace<'_>,
    scene_data: &SceneData,
    material: Arc<Material>,
) -> Triangle {
    let v1_attribute_string: String =
        parse_next_value_from_split(line).expect("No data for vertex 1");
    let v2_attribute_string: String =
        parse_next_value_from_split(line).expect("No data for vertex 2");
    let v3_attribute_string: String =
        parse_next_value_from_split(line).expect("No data for vertex 3");

    let (v1_index, v1_tex_coord_index, v1_normal_coord_index) =
        get_vertex_attributes(&v1_attribute_string);
    let (v2_index, v2_tex_coord_index, v2_normal_coord_index) =
        get_vertex_attributes(&v2_attribute_string);
    let (v3_index, v3_tex_coord_index, v3_normal_coord_index) =
        get_vertex_attributes(&v3_attribute_string);

    let v1 = scene_data
        .vertices
        .get(v1_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);
    let v2 = scene_data
        .vertices
        .get(v2_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);
    let v3 = scene_data
        .vertices
        .get(v3_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);

    let mut v1_tex_coords = DEFAULT_VERTICES;
    let mut v2_tex_coords = DEFAULT_VERTICES;
    let mut v3_tex_coords = DEFAULT_VERTICES;

    if let Some(v1_tc_index) = v1_tex_coord_index {
        v1_tex_coords = scene_data
            .vertex_texture_coords
            .get(v1_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }
    if let Some(v2_tc_index) = v2_tex_coord_index {
        v2_tex_coords = scene_data
            .vertex_texture_coords
            .get(v2_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }
    if let Some(v3_tc_index) = v3_tex_coord_index {
        v3_tex_coords = scene_data
            .vertex_texture_coords
            .get(v3_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }

    let mut v1_normal_coords = DEFAULT_VERTICES;
    let mut v2_normal_coords = DEFAULT_VERTICES;
    let mut v3_normal_coords = DEFAULT_VERTICES;

    if let Some(v1_normal_index) = v1_normal_coord_index {
        v1_normal_coords = scene_data
            .vertex_normal_coords
            .get(v1_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }
    if let Some(v2_normal_index) = v2_normal_coord_index {
        v2_normal_coords = scene_data
            .vertex_normal_coords
            .get(v2_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }
    if let Some(v3_normal_index) = v3_normal_coord_index {
        v3_normal_coords = scene_data
            .vertex_normal_coords
            .get(v3_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTICES);
    }

    Triangle {
        v1: *v1,
        v2: *v2,
        v3: *v3,
        v1_tex_coords: *v1_tex_coords,
        v2_tex_coords: *v2_tex_coords,
        v3_tex_coords: *v3_tex_coords,
        v1_normal_coords: *v1_normal_coords,
        v2_normal_coords: *v2_normal_coords,
        v3_normal_coords: *v3_normal_coords,
        material: Arc::clone(&material),
    }
}

fn get_texture_from_file_name(file_name: String) -> Texture {
    let img = ImageReader::open(file_name)
        .expect("Cannot read texture file")
        .decode()
        .expect("Cannot decode texture file");

    let mut cols = vec![];

    for chunk in img.as_bytes().chunks(3) {
        let new_col = Color {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
        };

        cols.push(new_col)
    }

    return Texture {
        width: img.width() as usize,
        height: img.height() as usize,
        colours: cols,
    };
}

fn get_color_coefficient_from_split_lines(line: &mut SplitWhitespace<'_>) -> Vector3d {
    let Vector3d { x: r, y: g, z: b } = get_vertex(line);

    assert!(
        r <= 1.0 && g <= 1.0 && b <= 1.0 && r >= 0.0 && g >= 0.0 && b >= 0.0,
        "All lighting intensity coefficients must be between 0.0 and 1.0"
    );

    return Vector3d { x: r, y: g, z: b };
}
