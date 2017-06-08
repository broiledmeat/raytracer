extern crate rand;
use std::f64::consts::PI;
use ray::Ray;
use vector3::Vector3;

pub struct Camera
{
    lower_left: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
    u: Vector3,
    v: Vector3,
    lense_radius: f64,
}

impl Camera
{
    pub fn new(origin: Vector3, look_at: Vector3, v_up: Vector3, fov: f64, aspect: f64, aperture: f64, focus_distance: f64) -> Camera
    {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).normalized();
        let u = v_up.cross(w).normalized();
        let v = w.cross(u);

        Camera
        {
            lower_left: origin - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            origin: origin,
            u: u,
            v: v,
            lense_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray
    {
        let rd = self.lense_radius * self.random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray{origin: self.origin + offset, direction: self.lower_left + (u * self.horizontal) + (v * self.vertical) - self.origin - offset}
    }

    fn random_in_unit_disk(&self) -> Vector3
    {
        loop
        {
            let p = 2.0 * Vector3{x: rand::random::<f64>(), y: rand::random::<f64>(), z: 0.0} - Vector3{x: 1.0, y: 1.0, z: 0.0};
            if p.dot(p) < 1.0
            {
                return p;
            }
        }
    }
}