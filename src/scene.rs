use std::f64;
use ray::Ray;
use renderable::{Renderable, HitResult};

pub struct Scene
{
    pub renderables: Vec<Box<Renderable>>
}

impl Scene
{
    pub fn test_hit(&self, ray: Ray) -> Option<HitResult>
    {
        let mut result: Option<HitResult> = None;
        let mut distance = f64::MAX;

        for renderable in self.renderables.iter()
        {
            let child_result = renderable.test_hit(ray, 0.001, f64::MAX);
            match child_result
            {
                None => {},
                Some(v) =>
                {
                    let child_distance = (v.origin - ray.origin).length();
                    if child_distance < distance
                    {
                        result = child_result;
                        distance = child_distance;
                    }
                }
            }
        }

        result
    }
}