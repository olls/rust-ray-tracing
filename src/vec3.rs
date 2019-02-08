use std::ops::{Add, Div, Mul, Neg, Sub, Rem};
use std::cmp::{PartialEq, PartialOrd, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct Vec3
{
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Add for Vec3
{
  type Output = Vec3;
  fn add(self, other: Vec3) -> Vec3
  {
    Vec3
    {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}

impl Sub for Vec3
{
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3
  {
    Vec3
    {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}

impl Mul for Vec3
{
  type Output = f32;
  fn mul(self, other: Vec3) -> f32
  {
    self.x * other.x +
    self.y * other.y +
    self.z * other.z
  }
}

impl Div for Vec3
{
  type Output = Vec3;
  fn div(self, other: Vec3) -> Vec3
  {
    Vec3
    {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}

impl Rem for Vec3
{
  type Output = Vec3;
  fn rem(self, other: Vec3) -> Vec3
  {
    Vec3
    {
      x: self.x % other.x,
      y: self.y % other.y,
      z: self.z % other.z
    }
  }
}

impl Neg for Vec3
{
  type Output = Vec3;
  fn neg(self) -> Vec3
  {
    Vec3
    {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}

impl PartialEq for Vec3
{
  fn eq(&self, other: &Vec3) -> bool
  {
    (self.x, self.y, self.z) == (other.x, other.y, other.z)
  }
}

impl PartialOrd for Vec3
{
  fn partial_cmp(&self, other: &Vec3) -> Option<Ordering>
  {
    (self.x, self.y, self.z).partial_cmp(&(other.x, other.y, other.z))
  }
}

impl Vec3
{
  pub fn new(x: f32, y: f32, z: f32) -> Vec3
  {
    Vec3 { x, y, z }
  }

  pub fn all(n: f32) -> Vec3
  {
    Vec3::new(n, n, n)
  }

  pub fn zero() -> Vec3
  {
    Vec3::all(0.0)
  }

  pub fn one() -> Vec3
  {
    Vec3::all(1.0)
  }

  pub fn norm(&self) -> f32
  {
    ((*self)*(*self)).sqrt()
  }

  pub fn normalise(&self) -> Vec3
  {
    self.scale(1.0/self.norm())
  }

  pub fn add(&self, other: f32) -> Vec3
  {
    Vec3
    {
      x: self.x + other,
      y: self.y + other,
      z: self.z + other
    }
  }

  pub fn scale(&self, other: f32) -> Vec3
  {
    Vec3
    {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }

  pub fn reflect(&self, n: Vec3) -> Vec3
  {
    *self - n.scale(2.0).scale(*self*n)
  }
}