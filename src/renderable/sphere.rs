use vector3::Vector3;
use ray::Ray;
use material::Material;
use renderable::{Renderable, HitResult};

pub struct Sphere
{
    pub origin: Vector3,
    pub radius: f64,
    pub material: Box<Material>
}

impl Renderable for Sphere
{
    fn test_hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitResult>
    {
        let vec = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = vec.dot(ray.direction);
        let c = vec.dot(vec) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0
        {
            for t in [(-b - discriminant.sqrt()) / a, (-b + discriminant.sqrt()) / a].iter()
            {
                if *t > min_t && *t < max_t
                {
                    let point = ray.translate_to(*t);
                    return Some(HitResult
                    {
                        origin: point,
                        normal: (point - self.origin) / self.radius,
                        t: *t,
                        material: &*self.material
                    });
                }
            }
        }
        
        None
    }
}