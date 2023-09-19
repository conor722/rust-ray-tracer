use std::cell::RefCell;

use crate::scene::engine::Vector3d;

use super::AABB::AABB;

struct Octree<'a> {
    pub bounding_box: AABB<'a>,
    pub children: RefCell<Vec<RefCell<Octree<'a>>>>,
    pub leaf_geometry: Option<&'a AABB<'a>>,
}

impl<'a> Octree<'a> {
    fn new(
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        min_z: f64,
        max_z: f64,
    ) -> Octree<'static> {
        let aabb = AABB::new(min_x, max_x, min_y, max_y, min_z, max_z);
        return Octree {
            bounding_box: aabb,
            children: RefCell::new(vec![]),
            leaf_geometry: None,
        };
    }

    fn push(mut self, aabb: &'a AABB) {
        if self.children.borrow().len() == 0 {
            self.subdivide(self.leaf_geometry);
        } else {
            self.leaf_geometry = Some(aabb);
        }
    }

    fn subdivide(&self, geometry: Option<&'a AABB>) {
        let Vector3d {
            x: x_min,
            y: y_min,
            z: z_min,
        } = self.bounding_box.min_coords;

        let Vector3d {
            x: x_max,
            y: y_max,
            z: z_max,
        } = self.bounding_box.max_coords;

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
            let mut lg: Option<&AABB> = None;

            if let Some(l) = geometry {
                if l.intersects(&abb) {
                    lg = Some(l);
                }
            }

            self.children.borrow_mut().push(RefCell::new(Octree {
                bounding_box: abb,
                children: RefCell::new(vec![]),
                leaf_geometry: lg,
            }));
        }
    }
}
