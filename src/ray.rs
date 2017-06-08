use vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Ray
{
    pub origin: Vector3,
    pub direction: Vector3
}

impl Ray
{
    pub fn translate_to(&self, distance: f64) -> Vector3
    {
        self.origin + (self.direction * distance)
    }
}