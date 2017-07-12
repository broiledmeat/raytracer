use vector3::Vector3;
use ray::Ray;
use renderable::HitResult;
use material::{Material, ScatterResult, reflect, random_in_unit_sphere};

pub struct Metal
{
    pub albedo: Vector3,
    pub fuzz: f64
}

impl Metal
{
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal
    {
        Metal { albedo: albedo, fuzz: fuzz }
    }
}

impl Material for Metal
{
    fn scatter(&self, ray: Ray, hit_result: HitResult) -> Option<ScatterResult>
    {

        let reflected = reflect(ray.direction.normalized(), hit_result.normal);
        let result = ScatterResult{
            scattered: Ray{origin: hit_result.origin, direction: reflected + self.fuzz * random_in_unit_sphere()},
            attenuation: self.albedo
        };

        if result.scattered.direction.dot(hit_result.normal) > 0.0
        {
            return Some(result);
        }

        None
    }
}