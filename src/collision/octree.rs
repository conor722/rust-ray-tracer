use std::collections::HashMap;

use crate::scene::{engine::Vector3d, entities::Triangle};

use super::AABB::AABB;

#[derive(Clone, Debug, PartialEq)]
pub struct Octree {
    pub octant_AABB_map: HashMap<usize, usize>,
    pub octant_triangle_map: HashMap<usize, usize>,
    pub octant_child_map: HashMap<usize, Vec<usize>>,
    pub AABBs: Vec<AABB>,
    pub triangles: Vec<Triangle>,
    pub triangle_aabb_map: HashMap<usize, usize>,
    pub octant_count: usize,
}

impl Octree {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, min_z: f64, max_z: f64) -> Octree {
        let aabb = AABB::new(min_x, max_x, min_y, max_y, min_z, max_z);

        Octree {
            AABBs: vec![aabb],
            octant_AABB_map: HashMap::from([(0, 0)]),
            octant_triangle_map: HashMap::new(),
            octant_child_map: HashMap::new(),
            triangle_aabb_map: HashMap::new(),
            triangles: vec![],
            octant_count: 1,
        }
    }

    pub fn push_triangle(&mut self, triangle: Triangle) {
        let triangle_aabb = AABB::from_triangle(&triangle);

        let aabb_index = self.AABBs.len();
        let triangle_index = self.triangles.len();

        self.triangles.push(triangle);
        self.AABBs.push(triangle_aabb);

        self.triangle_aabb_map.insert(triangle_index, aabb_index);

        self.push_at_octant(triangle_index, aabb_index, 0);
    }

    fn push_at_octant(&mut self, triangle_index: usize, aabb_index: usize, octant_index: usize) {
        let intersects: bool;
        let current_octant_has_triangle: bool;
        let is_leaf_octant: bool;
        let children: Vec<usize>;

        {
            let aabb = self.AABBs.get(aabb_index).unwrap();
            let octant_aabb_index = self.octant_AABB_map.get(&octant_index).unwrap();
            let octant_aabb = self.AABBs.get(*octant_aabb_index).unwrap();

            intersects = aabb.clone().intersects(octant_aabb);
            current_octant_has_triangle = self.octant_triangle_map.contains_key(&octant_index);
            children = self
                .octant_child_map
                .get(&octant_index)
                .unwrap_or(&vec![])
                .clone();
            is_leaf_octant = children.len() == 0;

            println!(
                "intersects={}, children={:?}, current_octant_has_triangle={}",
                intersects, children, current_octant_has_triangle
            );
        }

        if !intersects {
            return;
        }

        if is_leaf_octant && !current_octant_has_triangle {
            self.octant_triangle_map
                .insert(octant_index, triangle_index);
        } else if is_leaf_octant {
            let child_indices = self.subdivide(octant_index);

            let old_triangle_index = self
                .octant_triangle_map
                .remove_entry(&octant_index)
                .unwrap()
                .1;
            let old_triangle_aabb_index = *self.triangle_aabb_map.get(&old_triangle_index).unwrap();

            for ci in &child_indices {
                self.push_at_octant(triangle_index, aabb_index, *ci);
                self.push_at_octant(old_triangle_index, old_triangle_aabb_index, *ci);
            }
        } else if intersects && !is_leaf_octant {
            for ci in children.iter() {
                self.push_at_octant(triangle_index, aabb_index, *ci);
            }
        } else {
            unreachable!()
        }
    }

    fn subdivide(&mut self, octant_index: usize) -> Vec<usize> {
        let octant_aabb: AABB;

        {
            let aabb_index = self.octant_AABB_map.get(&octant_index).unwrap();
            octant_aabb = self.AABBs.get(*aabb_index).unwrap().clone();
        }

        let Vector3d {
            x: x_min,
            y: y_min,
            z: z_min,
        } = octant_aabb.min_coords;

        let Vector3d {
            x: x_max,
            y: y_max,
            z: z_max,
        } = octant_aabb.max_coords;

        let half_x_distance = (x_max - x_min) / 2.0;
        let half_y_distance = (y_max - y_min) / 2.0;
        let half_z_distance = (z_max - z_min) / 2.0;

        let bottom_back_left = AABB::new(
            x_min,
            x_min + half_x_distance,
            y_min,
            y_min + half_y_distance,
            z_min,
            z_min + half_z_distance,
        );

        let bottom_front_left = AABB::new(
            x_min,
            x_min + half_x_distance,
            y_min,
            y_min + half_y_distance,
            z_min + half_z_distance,
            z_max,
        );

        let bottom_front_right = AABB::new(
            x_min + half_x_distance,
            x_max,
            y_min,
            y_min + half_y_distance,
            z_min + half_z_distance,
            z_max,
        );

        let bottom_back_right = AABB::new(
            x_min + half_x_distance,
            x_max,
            y_min,
            y_min + half_y_distance,
            z_min,
            z_min + half_z_distance,
        );

        let top_back_left = AABB::new(
            x_min,
            x_min + half_x_distance,
            y_min + half_z_distance,
            y_max,
            z_min,
            z_min + half_z_distance,
        );

        let top_front_left = AABB::new(
            x_min,
            x_min + half_x_distance,
            y_min + half_z_distance,
            y_max,
            z_min + half_z_distance,
            z_max,
        );

        let top_front_right = AABB::new(
            x_min + half_x_distance,
            x_max,
            y_min + half_z_distance,
            y_max,
            z_min + half_z_distance,
            z_max,
        );

        let top_back_right = AABB::new(
            x_min + half_x_distance,
            x_max,
            y_min + half_z_distance,
            y_max,
            z_min,
            z_min + half_z_distance,
        );

        let mut child_indices = vec![];

        self.octant_child_map.insert(octant_index, vec![]);

        for abb in [
            bottom_back_left,
            bottom_front_left,
            bottom_front_right,
            bottom_back_right,
            top_back_left,
            top_front_left,
            top_front_right,
            top_back_right,
        ] {
            self.AABBs.push(abb);
            self.octant_AABB_map
                .insert(self.octant_count, self.AABBs.len() - 1);
            self.octant_child_map
                .get_mut(&octant_index)
                .unwrap()
                .push(self.octant_count);
            child_indices.push(self.octant_count);
            self.octant_count += 1;
        }

        child_indices
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::scene::entities::Color;

    use super::*;

    #[test]
    fn test_assigns_first_triangle_to_root() {
        let mut octree = Octree::new(-10.0, 10.0, -10.0, 10.0, -10.0, 10.0);

        let default_tex_coords = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let triangle = Triangle {
            v1: Vector3d {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
            v2: Vector3d {
                x: 2.0,
                y: 5.0,
                z: 2.0,
            },
            v3: Vector3d {
                x: 5.0,
                y: 2.0,
                z: 2.0,
            },
            v1_tex_coords: default_tex_coords,
            v2_tex_coords: default_tex_coords,
            v3_tex_coords: default_tex_coords,
            v1_normal_coords: default_tex_coords,
            v2_normal_coords: default_tex_coords,
            v3_normal_coords: default_tex_coords,
            color: Color { r: 255, g: 0, b: 0 },
            specular: 240.0,
            texture_index: 0,
        };

        octree.push_triangle(triangle.clone());

        assert_eq!(octree.octant_count, 1);
        assert_eq!(octree.triangles, vec![triangle.clone()]);
        assert_eq!(octree.octant_triangle_map, HashMap::from([(0, 0)]));
        assert_eq!(
            octree.AABBs,
            [
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: -10.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 2.0,
                        y: 2.0,
                        z: 2.0
                    },
                    max_coords: Vector3d {
                        x: 5.0,
                        y: 5.0,
                        z: 2.0
                    }
                }
            ]
        );
        assert_eq!(octree.octant_AABB_map, HashMap::from([(0, 0)]));
        assert_eq!(octree.triangle_aabb_map, HashMap::from([(0, 1)]));
    }

    #[test]
    fn test_pushes_all_triangles_to_leaves_when_multiple_triangles_added() {
        let mut octree = Octree::new(-10.0, 10.0, -10.0, 10.0, -10.0, 10.0);

        let default_tex_coords = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let triangle1 = Triangle {
            v1: Vector3d {
                x: 5.2,
                y: 5.2,
                z: 5.2,
            },
            v2: Vector3d {
                x: 5.2,
                y: 5.5,
                z: 5.2,
            },
            v3: Vector3d {
                x: 5.5,
                y: 5.2,
                z: 5.2,
            },
            v1_tex_coords: default_tex_coords,
            v2_tex_coords: default_tex_coords,
            v3_tex_coords: default_tex_coords,
            v1_normal_coords: default_tex_coords,
            v2_normal_coords: default_tex_coords,
            v3_normal_coords: default_tex_coords,
            color: Color { r: 255, g: 0, b: 0 },
            specular: 240.0,
            texture_index: 0,
        };

        let triangle2 = Triangle {
            v1: Vector3d {
                x: 0.6,
                y: 0.6,
                z: 0.6,
            },
            v2: Vector3d {
                x: 0.6,
                y: 0.8,
                z: 0.6,
            },
            v3: Vector3d {
                x: 0.8,
                y: 0.6,
                z: 0.6,
            },
            v1_tex_coords: default_tex_coords,
            v2_tex_coords: default_tex_coords,
            v3_tex_coords: default_tex_coords,
            v1_normal_coords: default_tex_coords,
            v2_normal_coords: default_tex_coords,
            v3_normal_coords: default_tex_coords,
            color: Color { r: 255, g: 0, b: 0 },
            specular: 240.0,
            texture_index: 0,
        };

        octree.push_triangle(triangle1.clone());
        octree.push_triangle(triangle2.clone());

        assert_eq!(octree.octant_count, 17);
        assert_eq!(octree.triangles, vec![triangle1.clone(), triangle2.clone()]);
        assert_eq!(octree.octant_triangle_map, HashMap::from([(9, 1), (15, 0)]));
        assert_eq!(
            octree.AABBs,
            [
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: -10.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 5.2,
                        y: 5.2,
                        z: 5.2
                    },
                    max_coords: Vector3d {
                        x: 5.5,
                        y: 5.5,
                        z: 5.2
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.6,
                        y: 0.6,
                        z: 0.6
                    },
                    max_coords: Vector3d {
                        x: 0.8,
                        y: 0.8,
                        z: 0.6
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: -10.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: -10.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: -10.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 0.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: -10.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 0.0,
                        z: 0.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: 0.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 0.0,
                        y: 10.0,
                        z: 0.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: -10.0,
                        y: 0.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 0.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: -10.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 0.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 5.0,
                        y: 5.0,
                        z: 5.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 0.0,
                        z: 5.0
                    },
                    max_coords: Vector3d {
                        x: 5.0,
                        y: 5.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 5.0,
                        y: 0.0,
                        z: 5.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 5.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 5.0,
                        y: 0.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 5.0,
                        z: 5.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 5.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 5.0,
                        y: 10.0,
                        z: 5.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 0.0,
                        y: 5.0,
                        z: 5.0
                    },
                    max_coords: Vector3d {
                        x: 5.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 5.0,
                        y: 5.0,
                        z: 5.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0
                    }
                },
                AABB {
                    min_coords: Vector3d {
                        x: 5.0,
                        y: 5.0,
                        z: 0.0
                    },
                    max_coords: Vector3d {
                        x: 10.0,
                        y: 10.0,
                        z: 5.0
                    }
                }
            ]
        );
        assert_eq!(
            octree.octant_AABB_map,
            HashMap::from([
                (2, 4),
                (10, 12),
                (5, 7),
                (0, 0),
                (9, 11),
                (12, 14),
                (13, 15),
                (15, 17),
                (14, 16),
                (16, 18),
                (4, 6),
                (6, 8),
                (1, 3),
                (7, 9),
                (8, 10),
                (11, 13),
                (3, 5)
            ])
        );
        assert_eq!(octree.triangle_aabb_map, HashMap::from([(0, 1), (1, 2)]));
    }
}
