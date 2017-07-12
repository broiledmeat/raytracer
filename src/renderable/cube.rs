use std::f64;
use vector3::Vector3;
use ray::Ray;
use material::Material;
use renderable::{Renderable, HitResult};

pub struct Cube
{
    pub origin: Vector3,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub material: Box<Material>
}

impl Cube
{
    pub fn new<T: Material + 'static>(origin: Vector3, width: f64, height: f64, depth: f64, material: T) -> Cube
    {
        Cube { origin: origin, width: width, height: height, depth: depth, material: Box::new(material) }
    }
}

impl Renderable for Cube
{
    fn test_hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitResult>
    {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let half_depth = self.depth / 2.0;
        let mut t_min = -f64::INFINITY;
        let mut t_max = f64::INFINITY;

        if ray.direction.x != 0.0
        {
            let tx0 = (self.origin.x - half_width - ray.origin.x) / ray.direction.x;
            let tx1 = (self.origin.x + half_width - ray.origin.x) / ray.direction.x;

            t_min = t_min.max(tx0.min(tx1));
            t_max = t_max.min(tx0.max(tx1));
        }

        if ray.direction.y != 0.0
        {
            let tx0 = (self.origin.y - half_height - ray.origin.y) / ray.direction.y;
            let tx1 = (self.origin.y + half_height - ray.origin.y) / ray.direction.y;

            t_min = t_min.max(tx0.min(tx1));
            t_max = t_max.min(tx0.max(tx1));
        }

        if ray.direction.z != 0.0
        {
            let tx0 = (self.origin.z - half_depth - ray.origin.z) / ray.direction.z;
            let tx1 = (self.origin.z + half_depth - ray.origin.z) / ray.direction.z;

            t_min = t_min.max(tx0.min(tx1));
            t_max = t_max.min(tx0.max(tx1));
        }

        if t_min >= min_t && t_max <= max_t && t_max >= t_min
        {
            let point = ray.translate_to(t_min);
            let mut normal = point - self.origin;

            if normal.x == self.origin.x - half_width
            {
                normal = Vector3{x: -1.0, y: 0.0, z: 0.0};
            }
            else if normal.x == self.origin.x + half_width
            {
                normal = Vector3{x: 1.0, y: 0.0, z: 0.0};
            }
            else if normal.y == self.origin.x - half_height
            {
                normal = Vector3{x: 0.0, y: -1.0, z: 0.0};
            }
            else if normal.y == self.origin.x + half_height
            {
                normal = Vector3{x: 0.0, y: 1.0, z: 0.0};
            }
            else if normal.z == self.origin.x - half_depth
            {
                normal = Vector3{x: 0.0, y: 0.0, z: -1.0};
            }
            else if normal.z == self.origin.x + half_depth
            {
                normal = Vector3{x: 0.0, y: 0.0, z: 1.0};
            }

            return Some(HitResult
            {
                origin: point, //self.origin + t_min * ray.direction,
                normal: normal,
                t: t_min,
                material: &*self.material
            });
        }

        None
    }
}
