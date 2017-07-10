pub mod plane;
pub mod plane_bounded;
pub mod sphere;
pub mod cube;

use vector3::Vector3;
use ray::Ray;
use material::Material;

pub const EPSILON: f64 = 0.001;

#[derive(Clone, Copy)]
pub struct HitResult<'a>
{
    pub origin: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub material: &'a Material
}

pub trait Renderable
{
    fn test_hit(&self, ray: Ray, min_time: f64, max_time: f64) -> Option<HitResult>;
}