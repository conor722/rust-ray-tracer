use std::fmt::Debug;
use std::str::{FromStr, Lines, SplitWhitespace};

use crate::collision::octree::Octree;
use crate::scene::engine::Vector3d;
use crate::scene::entities::{Color, Texture, Triangle};

use image::io::Reader as ImageReader;

#[derive(Debug, PartialEq)]
pub struct SceneData {
    pub triangles: Vec<Triangle>,
    pub vertices: Vec<Vector3d>,
    pub vertex_texture_coords: Vec<Vector3d>,
    pub vertex_normal_coords: Vec<Vector3d>,
    pub textures: Vec<Texture>,
    pub octree: Octree,
}

static DEFAULT_VERTEX_TEXTURE_COORDS: &Vector3d = &Vector3d {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
static MISSING_VERTEX_ERROR_MESSAGE: &str = "No vertex with this index";

pub fn parse_lines<'a>(lines: Lines) -> SceneData {
    let vertices = Vec::new();
    let triangles = Vec::new();
    let vertex_texture_coords = Vec::new();
    let vertex_normal_coords = Vec::new();

    let octree: Octree = Octree::new(-10.0, 10.0, -10.0, 10.0, -10.0, 10.0);

    let textures = vec![Texture {
        colours: vec![
            Color { r: 255, g: 0, b: 0 },
            Color { r: 255, g: 0, b: 0 },
            Color { r: 255, g: 0, b: 0 },
            Color { r: 255, g: 0, b: 0 },
        ],
        width: 2,
        height: 2,
    }];

    let mut scene_data: SceneData = SceneData {
        vertices,
        triangles,
        vertex_texture_coords,
        vertex_normal_coords,
        textures,
        octree,
    };

    for line in lines {
        let mut split_line = line.split_whitespace();
        let line_type = split_line.next();

        match line_type {
            Some("v") => {
                let v = get_vertex(&mut split_line);
                scene_data.vertices.push(v);
            }
            Some("f") => {
                let tri = get_triangle(&mut split_line, &scene_data);
                scene_data.triangles.push(tri);
            }
            Some("vt") => {
                let vt = get_vertex(&mut split_line);
                scene_data.vertex_texture_coords.push(vt);
            }
            Some("vn") => {
                let vn = get_vertex(&mut split_line);
                scene_data.vertex_normal_coords.push(vn);
            }
            Some("t") => {
                let file_name: String =
                    parse_next_value_from_split(&mut split_line).expect("Invalid texture name");
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

                let tex = Texture {
                    width: img.width() as usize,
                    height: img.height() as usize,
                    colours: cols,
                };

                scene_data.textures.push(tex);
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
    let z: f64 = parse_next_value_from_split(&mut line).expect("Cannot parse vertex");

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

fn get_triangle<'a>(line: &'a mut SplitWhitespace<'_>, scene_data: &SceneData) -> Triangle {
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

    let specular: f64 = 240.0;

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

    let mut v1_tex_coords = DEFAULT_VERTEX_TEXTURE_COORDS;
    let mut v2_tex_coords = DEFAULT_VERTEX_TEXTURE_COORDS;
    let mut v3_tex_coords = DEFAULT_VERTEX_TEXTURE_COORDS;

    if let Some(v1_tc_index) = v1_tex_coord_index {
        v1_tex_coords = scene_data
            .vertex_texture_coords
            .get(v1_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
    }
    if let Some(v2_tc_index) = v2_tex_coord_index {
        v2_tex_coords = scene_data
            .vertex_texture_coords
            .get(v2_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
    }
    if let Some(v3_tc_index) = v3_tex_coord_index {
        v3_tex_coords = scene_data
            .vertex_texture_coords
            .get(v3_tc_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
    }

    let mut v1_normal_coords = DEFAULT_VERTEX_TEXTURE_COORDS;
    let mut v2_normal_coords = DEFAULT_VERTEX_TEXTURE_COORDS;
    let mut v3_normal_coords = DEFAULT_VERTEX_TEXTURE_COORDS;

    if let Some(v1_normal_index) = v1_normal_coord_index {
        v1_normal_coords = scene_data
            .vertex_normal_coords
            .get(v1_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
    }
    if let Some(v2_normal_index) = v2_normal_coord_index {
        v2_normal_coords = scene_data
            .vertex_normal_coords
            .get(v2_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
    }
    if let Some(v3_normal_index) = v3_normal_coord_index {
        v3_normal_coords = scene_data
            .vertex_normal_coords
            .get(v3_normal_index - 1)
            .unwrap_or_else(|| DEFAULT_VERTEX_TEXTURE_COORDS);
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
        color: Color { r: 0, g: 255, b: 0 },
        specular,
        texture_index: scene_data.textures.len() - 1,
    }
}
