use super::AABB::AABB;

struct Octree<'a> {
    pub bounding_box: AABB<'a>,
    pub children: Vec<Option<Octree<'a>>>,
}
