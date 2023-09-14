use std::fmt::Debug;
use std::str::{FromStr, Lines, Split};

use crate::scene::engine::Vector3d;
use crate::scene::entities::{Color, Triangle};

static MISSING_VERTEX_ERROR_MESSAGE: &str = "No vertex with this index";

pub fn parse_lines(lines: Lines) -> Vec<Triangle> {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();

    for line in lines {
        let mut split_line = line.split(" ");
        let line_type = split_line.next();

        match line_type {
            Some("vertex") => {
                let v = get_vertex(&mut split_line);
                vertices.push(v);
            }
            Some("triangle") => {
                let tri = get_triangle(&mut split_line, &vertices);
                triangles.push(tri);
            }
            _ => panic!("Unknown line type"),
        }
    }

    triangles
}

fn parse_next_value_from_split<T: FromStr>(line: &mut Split<'_, &str>) -> T
where
    <T as FromStr>::Err: Debug,
{
    line.next()
        .expect("premature end of line")
        .parse()
        .expect("cannot parse to float")
}

fn get_vertex(mut line: &mut Split<'_, &str>) -> Vector3d {
    let x: f64 = parse_next_value_from_split(&mut line);
    let y: f64 = parse_next_value_from_split(&mut line);
    let z: f64 = parse_next_value_from_split(&mut line);

    return Vector3d { x, y, z };
}

fn get_triangle(line: &mut Split<'_, &str>, vertices: &Vec<Vector3d>) -> Triangle {
    let v1_index: usize = parse_next_value_from_split(line);
    let v2_index: usize = parse_next_value_from_split(line);
    let v3_index: usize = parse_next_value_from_split(line);

    let r: u8 = parse_next_value_from_split(line);
    let g: u8 = parse_next_value_from_split(line);
    let b: u8 = parse_next_value_from_split(line);

    let specular: f64 = parse_next_value_from_split(line);

    let v1 = vertices
        .get(v1_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);
    let v2 = vertices
        .get(v2_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);
    let v3 = vertices
        .get(v3_index - 1)
        .expect(MISSING_VERTEX_ERROR_MESSAGE);

    Triangle {
        v1: *v1,
        v2: *v2,
        v3: *v3,
        color: Color { r, g, b },
        specular,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let lines = "vertex -20.0 0.0 120.0\n\
            vertex 20.0 0.0 120.0\n\
            vertex 0.0 20.0 120.0\n\
            triangle 1 2 3 255 0 0 240.0\n\
            vertex 20.0 0.0 120.0\n\
            vertex 50.0 0.0 140.0\n\
            vertex 20.0 20.0 150.0\n\
            triangle 4 5 6 0 255 0 240.0\n";
        let triangles = vec![
            Triangle {
                v1: Vector3d {
                    x: -20.0,
                    y: 0.0,
                    z: 120.0,
                },
                v2: Vector3d {
                    x: 20.0,
                    y: 0.0,
                    z: 120.0,
                },
                v3: Vector3d {
                    x: 0.0,
                    y: 20.0,
                    z: 120.0,
                },
                color: Color { r: 255, g: 0, b: 0 },
                specular: 240.0,
            },
            Triangle {
                v1: Vector3d {
                    x: 20.0,
                    y: 0.0,
                    z: 120.0,
                },
                v2: Vector3d {
                    x: 50.0,
                    y: 0.0,
                    z: 140.0,
                },
                v3: Vector3d {
                    x: 20.0,
                    y: 20.0,
                    z: 150.0,
                },
                color: Color { r: 0, g: 255, b: 0 },
                specular: 240.0,
            },
        ];

        let result = parse_lines(lines.lines());

        assert_eq!(result, triangles);
    }
}
