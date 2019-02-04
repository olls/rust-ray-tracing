use vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
  pub diffuse_colour: Vec3,
  pub specular_exponent: f32,
  pub albedo: Vec3
}

impl Material
{
  pub fn new(diffuse_colour: Vec3, specular_exponent: f32, albedo: Vec3) -> Material
  {
    Material
    {
      diffuse_colour,
      specular_exponent,
      albedo
    }
  }
}
