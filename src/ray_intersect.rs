use crate::color::Color;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersect>;
}
