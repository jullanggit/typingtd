use crate::physics::{collides, intersects, MovingObject, Position, AABB};
use bevy::{prelude::*, utils::tracing::field::debug};

#[derive(Debug)]
pub struct Quadtree {
    boundary: AABB,
    center: Position,
    capacity: usize,
    objects: Vec<(Entity, Option<AABB>, Position)>,
    divided: bool,
    max_depth: u8,
    depth: u8,
    // Children
    nw: Option<Box<Self>>,
    ne: Option<Box<Self>>,
    sw: Option<Box<Self>>,
    se: Option<Box<Self>>,
}
impl Quadtree {
    pub const fn new(
        boundary: AABB,
        center: Vec2,
        capacity: usize,
        max_depth: u8,
        depth: u8,
    ) -> Self {
        Self {
            boundary,
            center: Position::new(center),
            capacity,
            objects: Vec::new(),
            divided: false,
            max_depth,
            depth,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    pub fn subdivide(&mut self) {
        let half_boundary = self.boundary.halfsize / 2.0;
        let center = self.center.value;
        let halfsize = Vec2::new(half_boundary.x, half_boundary.y);

        // Northwest
        let nw = AABB { halfsize };
        let nw_center = Vec2::new(center.x - half_boundary.x, center.y + half_boundary.y);
        self.nw = Some(Box::new(Self::new(
            nw,
            nw_center,
            self.capacity,
            self.max_depth,
            self.depth + 1,
        )));

        // Northeast
        let ne = AABB { halfsize };
        let ne_center = Vec2::new(center.x + half_boundary.x, center.y + half_boundary.y);
        self.ne = Some(Box::new(Self::new(
            ne,
            ne_center,
            self.capacity,
            self.max_depth,
            self.depth + 1,
        )));

        // Southwest
        let sw = AABB { halfsize };
        let sw_center = Vec2::new(center.x - half_boundary.x, center.y - half_boundary.y);
        self.sw = Some(Box::new(Self::new(
            sw,
            sw_center,
            self.capacity,
            self.max_depth,
            self.depth + 1,
        )));

        // Southeast
        let se = AABB { halfsize };
        let se_center = Vec2::new(center.x + half_boundary.x, center.y - half_boundary.y);
        self.se = Some(Box::new(Self::new(
            se,
            se_center,
            self.capacity,
            self.max_depth,
            self.depth + 1,
        )));

        self.divided = true;

        let objects = std::mem::take(&mut self.objects);

        for (entity, aabb, position) in objects {
            self.insert(entity, aabb, position);
        }
    }

    pub fn insert(&mut self, entity: Entity, aabb: Option<AABB>, position: Position) -> bool {
        match aabb {
            Some(ref aabb) => {
                // Check if the aabb intersects the nodes boundary
                if !self.boundary.contains(self.center, aabb, position) {
                    return false;
                }
            }
            None => {
                // Check if the point intersects the nodes boundary
                if !intersects(&self.boundary, self.center, position) {
                    return false;
                }
            }
        }
        // If the node hasnt been subdivided yet
        if !self.divided {
            // and it still has capacity or has reached max depth
            if self.objects.len() < self.capacity || self.depth >= self.max_depth {
                // add it to the objects and return
                self.objects.push((entity, aabb, position));
                return true;
            }
            // if it doesnt have capacity anymore, subdivide
            self.subdivide();
        }

        // Define an array of mutable references to each quadrant
        let quadrants = [&mut self.nw, &mut self.ne, &mut self.sw, &mut self.se];

        // Attempts to insert it into a child node
        for quadrant in quadrants {
            if let Some(quadrant_ref) = quadrant.as_mut() {
                if quadrant_ref.insert(entity, aabb.clone(), position) {
                    return true;
                };
            }
        }

        // If it wasnt inserted, add it to the current node's objects (has to fit because of check
        // at the start of the function)
        self.objects.push((entity, aabb, position));
        true
    }

    pub fn query(&self, range: &AABB, position: Position, found: &mut Vec<Entity>) {
        // dont do anything if the range doesnt intersect with the nodes boundary
        if !collides(&self.boundary, self.center, range, position) {
            return;
        }

        // add objects of the current node
        found.extend(self.objects.iter().map(|(entity, _, _)| entity));

        // query child nodes
        if self.divided {
            self.nw.as_ref().unwrap().query(range, position, found);
            self.ne.as_ref().unwrap().query(range, position, found);
            self.sw.as_ref().unwrap().query(range, position, found);
            self.se.as_ref().unwrap().query(range, position, found);
        }
    }
}

pub fn build_quadtree<'a, T, I, F>(
    items: I,
    aabb: &AABB,
    capacity: usize,
    max_depth: u8,
    transform: F,
) -> Quadtree
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> (Option<&'a AABB>, &'a MovingObject, Entity),
{
    let mut quadtree = Quadtree::new(aabb.clone(), Vec2::ZERO, capacity, max_depth, 0);
    items.into_iter().map(transform).for_each(|item| {
        quadtree.insert(item.2, item.0.cloned(), item.1.position);
    });
    quadtree
}
