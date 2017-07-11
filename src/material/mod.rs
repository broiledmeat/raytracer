pub mod lambert;
pub mod metal;
pub mod dielectric;

extern crate rand;
use vector3::{ONE, Vector3};
use ray::Ray;
use renderable::HitResult;


pub struct ScatterResult
{
    pub scattered: Ray,
    pub attenuation: Vector3
}

pub trait Material
{
    fn scatter(&self, ray: Ray, hit_result: HitResult) -> Option<ScatterResult>;
}

fn reflect(v: Vector3, n: Vector3) -> Vector3
{
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vector3, n: Vector3, ni_over_nt: f64) -> Option<Vector3>
{
    let vec = v.normalized();
    let dt = vec.dot(n);
    let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if disc > 0.0
    {
        Some(ni_over_nt * (vec - n * dt) - n * disc.sqrt())
    }
    else 
    {
        None
    }
}

fn schlick(cosine: f64, refraction: f64) -> f64
{
    let mut r0 = (1.0 - refraction) / (1.0 + refraction);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn random_in_unit_sphere() -> Vector3
{
    loop
    {
        let vec = 2.0 * Vector3{x: rand::random::<f64>(), y: rand::random::<f64>(), z: rand::random::<f64>()} - ONE;
        if vec.length_sqr() >= 1.0
        {
            return vec;
        }
    }
}