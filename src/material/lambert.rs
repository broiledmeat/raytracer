use vector3::Vector3;
use ray::Ray;
use renderable::HitResult;
use material::{Material, ScatterResult, random_in_unit_sphere};

pub struct Lambert
{
    pub albedo: Vector3
}

impl Lambert
{
    pub fn new(albedo: Vector3) -> Lambert
    {
        Lambert { albedo: albedo }
    }
}

impl Material for Lambert
{
    #[allow(unused_variables)]
    fn scatter(&self, ray: Ray, hit_result: HitResult) -> Option<ScatterResult>
    {
        let target = hit_result.origin + hit_result.normal + random_in_unit_sphere();
        Some(ScatterResult{
            scattered: Ray{origin: hit_result.origin, direction: target - hit_result.origin},
            attenuation: self.albedo
        })
    }
}