use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

// #[derive(Clone, Copy, Eq)]
#[derive(Clone, Copy)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub const ZERO: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
pub const ONE: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};

impl Vector3
{
    pub fn length(&self) -> f64
    {
        self.length_sqr().sqrt()
    }

    pub fn length_sqr(&self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: Self) -> f64
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self
    {
        Vector3{
            x: (self.y * other.z) - (self.z * other.y),
            y: -((self.x * other.z) - (self.z * other.x)),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }

    pub fn normalized(&self) -> Self
    {
        *self / self.length()
    }
}

impl Neg for Vector3
{
    type Output = Self;

    fn neg(self) -> Self
    {
        Vector3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Add for Vector3
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Vector3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl AddAssign for Vector3
{
    fn add_assign(&mut self, other: Self)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        Vector3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl SubAssign for Vector3
{
    fn sub_assign(&mut self, other: Self)
    {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vector3
{
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Vector3{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Mul<f64> for Vector3
{
    type Output = Self;

    fn mul(self, other: f64) -> Self
    {
        Vector3{x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Mul<Vector3> for f64
{
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3
    {
        Vector3{x: self * other.x, y: self * other.y, z: self * other.z}
    }
}

impl MulAssign for Vector3
{
    fn mul_assign(&mut self, other: Self)
    {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f64> for Vector3
{
    fn mul_assign(&mut self, other: f64)
    {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div for Vector3
{
    type Output = Self;

    fn div(self, other: Self) -> Self
    {
        Vector3{x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }
}

impl Div<f64> for Vector3
{
    type Output = Self;

    fn div(self, other: f64) -> Self
    {
        Vector3{x: self.x / other, y: self.y / other, z: self.z / other}
    }
}

impl DivAssign for Vector3
{
    fn div_assign(&mut self, other: Self)
    {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f64> for Vector3
{
    fn div_assign(&mut self, other: f64)
    {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

// impl Ord for Vector3
// {
//     fn cmp(&self, other: &Self) -> Ordering
//     {
//         self.length().partial_cmp(&other.length())
//     }
// }

// impl PartialOrd for Vector3 {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for Vector3 {
//     fn eq(&self, other: &Self) -> bool {
//         self.length() == other.length()
//     }
// }