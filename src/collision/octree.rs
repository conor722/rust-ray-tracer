use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::scene::{engine::Vector3d, entities::Triangle};

use super::AABB::AABB;

#[derive(Clone, Debug, PartialEq)]
pub struct Octree {
    pub octant_AABB_map: HashMap<usize, usize>,
    pub octant_triangle_map: HashMap<usize, usize>,
    pub octant_child_map: HashMap<usize, Vec<usize>>,
    pub AABBs: Vec<AABB>,
    pub octants: Vec<Octant>,
    pub triangles: Vec<Triangle>,
    pub triangle_aabb_map: HashMap<usize, usize>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Octant {
    pub children: Vec<usize>,
}

impl Octant {
    fn new() -> Octant {
        Octant { children: vec![] }
    }
}

impl Octree {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, min_z: f64, max_z: f64) -> Octree {
        let aabb = AABB::new(min_x, max_x, min_y, max_y, min_z, max_z);
        let octant = { Octant { children: vec![] } };

        Octree {
            AABBs: vec![aabb],
            octant_AABB_map: HashMap::from([(0, 0)]),
            octant_triangle_map: HashMap::new(),
            octant_child_map: HashMap::new(),
            triangle_aabb_map: HashMap::new(),
            octants: vec![octant],
            triangles: vec![],
        }
    }

    fn root(&self) -> &Octant {
        &self.octants.get(0).unwrap()
    }

    pub fn push_triangle(mut self, triangle: Triangle) {
        let triangle_aabb = AABB::from_triangle(&triangle);

        let aabb_index = self.AABBs.len();
        let triangle_index = self.AABBs.len();

        self.triangles.push(triangle);
        self.AABBs.push(triangle_aabb);

        self.triangle_aabb_map
            .borrow_mut()
            .insert(self.triangles.len() - 1, self.AABBs.len() - 1);

        self.push_at_octant(triangle_index, aabb_index, 0);
    }

    fn push_at_octant(&mut self, triangle_index: usize, aabb_index: usize, octant_index: usize) {
        let intersects: bool;
        let current_octant_has_triangle: bool;
        let is_leaf_octant: bool;
        let children: Vec<usize>;

        {
            let octant = self.octants.get(octant_index).unwrap();
            let aabb = self.AABBs.get(aabb_index).unwrap();
            let octant_aabb_index = self.octant_AABB_map.get(&octant_index).unwrap();
            let octant_aabb = self.AABBs.get(*octant_aabb_index).unwrap();

            intersects = aabb.clone().intersects(octant_aabb);
            current_octant_has_triangle = self.octant_triangle_map.contains_key(&octant_index);
            is_leaf_octant = octant.children.len() == 0;
            children = octant.children.clone();
        }

        if intersects && is_leaf_octant && !current_octant_has_triangle {
            self.octant_triangle_map
                .insert(octant_index, triangle_index);
        } else if intersects && is_leaf_octant {
            let child_indices = self.subdivide(octant_index);

            for ci in &child_indices {
                let old_triangle_index = self
                    .octant_triangle_map
                    .remove_entry(&octant_index)
                    .unwrap()
                    .1;
                let old_triangle_aabb_index =
                    *self.triangle_aabb_map.get(&old_triangle_index).unwrap();

                self.push_at_octant(triangle_index, aabb_index, *ci);
                self.push_at_octant(old_triangle_index, old_triangle_aabb_index, *ci);
            }
        } else if intersects && !is_leaf_octant {
            for ci in children.iter() {
                self.push_at_octant(triangle_index, aabb_index, *ci);
            }
        }
    }

    fn subdivide(&mut self, octant_index: usize) -> Vec<usize> {
        let octant_aabb: AABB;
        let octant_children: &Vec<usize>;
        let octant: &mut Octant;
        let octants: &mut Vec<Octant>;

        {
            octants = &mut self.octants;
            octant = octants.get_mut(octant_index).unwrap();
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
            let child_octant = Octant::new();
            self.octants.push(child_octant);
            self.octant_child_map
                .get_mut(&octant_index)
                .unwrap()
                .push(self.octants.len());
            child_indices.push(self.octants.len() - 1);
        }

        child_indices
    }
}
