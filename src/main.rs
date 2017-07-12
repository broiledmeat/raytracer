extern crate rgb;
extern crate lodepng;
extern crate rand;
mod camera;
mod material;
mod ray;
mod renderable;
mod scene;
mod vector3;
use rgb::{RGBA, ByteSlice};
use camera::Camera;
use ray::Ray;
use renderable::plane::Plane;
use renderable::plane_bounded::PlaneBounded;
use renderable::sphere::Sphere;
use renderable::cube::Cube;
use material::lambert::Lambert;
use material::metal::Metal;
use material::dielectric::Dielectric;
use scene::Scene;
use vector3::Vector3;


fn main()
{
    const WIDTH: usize = 400;
    const HEIGHT: usize = 200;
    const RAY_COUNT: usize = 200;
    const BOUNCE_MAX: i32 = 100;

    let mut scene = Scene::new();
    scene.add(Plane::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Lambert::new(Vector3::new(0.4, 0.8, 0.4))));
    scene.add(PlaneBounded::new(Vector3::new(0.0, 0.25, 0.5), Vector3::new(-0.25, 0.5, 0.0), 0.5, 0.25, Lambert::new(Vector3::new(0.4, 0.4, 0.8))));
    scene.add(Sphere::new(Vector3::new(-1.0, 0.5, -1.0), 0.5, Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.2)));
    scene.add(Sphere::new(Vector3::new(1.0, 0.5, 0.2), 0.35, Metal::new(Vector3::new(0.8, 0.2, 0.6), 0.05)));
    scene.add(Cube::new(Vector3::new(-0.25, 0.5, -0.2), 0.5, 0.5, 0.5, Lambert::new(Vector3::new(0.8, 0.0, 0.0))));
    scene.add(Cube::new(Vector3::new(0.5, 0.5, 0.5), 1.0, 0.5, 0.5, Dielectric::new(0.5)));

    let camera_origin = Vector3{x: -0.75, y: 1.2, z: 1.0};
    let camera_look_at = Vector3{x: 0.0, y: 0.5, z: 0.0};
    let camera_up = Vector3{x: 0.0, y: 1.0, z:0.0};
    let camera = Camera::new(
        camera_origin, camera_look_at,
        camera_up, 72.0, (WIDTH as f64) / (HEIGHT as f64), 0.0, (camera_origin - camera_look_at).length());

    let mut pixel_data = [RGBA{r: 0, g: 0, b: 0, a: 255}; WIDTH * HEIGHT];

    for y in 0..HEIGHT
    {
        for x in 0..WIDTH
        {
            let mut color = vector3::ZERO;
            for _ in 0..RAY_COUNT
            {
                let u = ((x as f64) + rand::random::<f64>()) / (WIDTH as f64);
                let v = ((y as f64) + rand::random::<f64>()) / (HEIGHT as f64);
                let ray = camera.get_ray(u, v);
                color += get_color(ray, &scene, BOUNCE_MAX);
            }
            color /= RAY_COUNT as f64;
            color = Vector3{x: color.x.sqrt(), y: color.y.sqrt(), z: color.z.sqrt()};

            pixel_data[(HEIGHT - 1 - y) * WIDTH + x] = RGBA
            {
                r: (color.x * 255.0) as u8, 
                g: (color.y * 255.0) as u8,
                b: (color.z * 255.0) as u8, 
                a: 255
            };
        }
    }

    lodepng::encode32_file("out.png", &pixel_data.as_bytes(), WIDTH, HEIGHT).unwrap();
}

fn get_color(ray: Ray, scene: &Scene, bounce_max: i32) -> Vector3
{
    let hit_result = scene.test_hit(ray);
    match hit_result
    {
        None => {},
        Some(h) =>
        {
            if bounce_max < 0
            {
                return vector3::ZERO;
            }

            let scatter_result = h.material.scatter(ray, h);
            match scatter_result
            {
                None => {},
                Some(s) =>
                {
                    return s.attenuation * get_color(s.scattered, scene, bounce_max - 1);
                }
            }
        }
    }

    let direction = ray.direction.normalized();
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t) * vector3::ONE + t * Vector3{x: 0.5, y: 0.7, z: 1.0}
}
