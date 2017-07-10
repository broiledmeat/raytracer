use vector3::Vector3;
use ray::Ray;
use material::Material;
use renderable::{Renderable, HitResult, EPSILON};

pub struct Plane
{
    pub origin: Vector3,
    pub normal: Vector3,
    pub material: Box<Material>
}

impl Renderable for Plane
{
    fn test_hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitResult>
    {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() > EPSILON
        {
            let t = (self.origin - ray.origin).dot(self.normal) / denom;
            let normal = if t >= 0.0 { self.normal } else { -self.normal };
            if t > min_t && t < max_t
            {
                let point = ray.translate_to(t);
                return Some(HitResult
                {
                    origin: point,
                    normal: normal,
                    t: t,
                    material: &*self.material
                });
            }
        }

        None
    }
}