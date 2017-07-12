extern crate rand;
use vector3::Vector3;
use ray::Ray;
use renderable::HitResult;
use material::{Material, ScatterResult, reflect, refract, schlick};

pub struct Dielectric
{
    pub refraction: f64
}

impl Dielectric
{
    pub fn new(refraction: f64) -> Dielectric
    {
        Dielectric { refraction: refraction }
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray: Ray, hit_result: HitResult) -> Option<ScatterResult>
    {
        let reflected = reflect(ray.direction.normalized(), hit_result.normal);
        let outward_normal: Vector3;
        let ni_over_nt: f64;
        let cosine: f64;

        if ray.direction.dot(hit_result.normal) > 0.0
        {
            outward_normal = -hit_result.normal;
            ni_over_nt = self.refraction;
            cosine = self.refraction * ray.direction.dot(hit_result.normal) / ray.direction.length();
        }
        else
        {
            outward_normal = hit_result.normal;
            ni_over_nt = 1.0 / self.refraction;
            cosine = -ray.direction.dot(hit_result.normal) / ray.direction.length();
        }

        let refracted = refract(ray.direction, outward_normal, ni_over_nt);
        match refracted
        {
            None => {},
            Some(v) =>
            {
                if rand::random::<f64>() > schlick(cosine, self.refraction)
                {
                    return Some(ScatterResult
                    {
                        scattered: Ray{origin: hit_result.origin, direction: v},
                        attenuation: Vector3{x: 1.0, y: 1.0, z: 1.0}
                    });
                }
            }
        }

        Some(ScatterResult
        {
            scattered: Ray{origin: hit_result.origin, direction: reflected},
            attenuation: Vector3{x: 1.0, y: 1.0, z: 1.0}
        })
    }
}