extern crate rand;
use ray::Ray;
use renderable::HitResult;
use vector3::{ONE, Vector3};


pub struct ScatterResult
{
    pub scattered: Ray,
    pub attenuation: Vector3
}

pub trait Material
{
    fn scatter(&self, ray: Ray, hit_result: HitResult) -> Option<ScatterResult>;
}

pub struct Lambert
{
    pub albedo: Vector3
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

pub struct Metal
{
    pub albedo: Vector3,
    pub fuzz: f64
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
            Some(result)
        }
        else
        {
            None
        }
    }
}

pub struct Dielectric
{
    pub refraction: f64
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