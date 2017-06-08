use vector3::Vector3;
use ray::Ray;
use material::Material;

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
        if denom.abs() > 0.0001
        {
            let t = (self.origin - ray.origin).dot(self.normal) / denom;
            let normal = if t >= 0.0 { self.normal } else { -self.normal };
            if t >= 0.0001
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