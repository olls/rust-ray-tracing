use vec3::Vec3;
use material::Material;

#[derive(Debug)]
pub struct Sphere {
  pub centre: Vec3,
  pub radius: f32,
  pub material: Material
}

impl Sphere
{
  pub fn ray_intersect(&self, orig: Vec3, dir: Vec3) -> (bool, Option<f32>)
  {
    let l: Vec3 = self.centre - orig;
    let tca: f32 = l*dir;
    let d2: f32 = l*l - tca*tca;

    if d2 > self.radius*self.radius
    {
      (false, None)
    }
    else
    {
      let thc: f32 = (self.radius*self.radius - d2).sqrt();

      let t0: f32;
      if tca - thc < 0.0
      {
        t0 = tca + thc;
      }
      else {
        t0 = tca - thc;
      }

      if t0 < 0.0
      {
        (false, Some(t0))
      }
      else
      {
        (true, Some(t0))
      }
    }
  }
}
