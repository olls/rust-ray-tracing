use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::f32;
use std::f32::consts::PI;

mod vec3;
use vec3::Vec3;

mod material;
use material::Material;

mod light;
use light::Light;

mod sphere;
use sphere::Sphere;

fn scene_intersect(orig: Vec3, dir: Vec3, spheres: &[Sphere]) -> Option<(Vec3, Vec3, Material)>
{
  let mut hit: Vec3 = Vec3::zero();
  let mut n: Vec3 = Vec3::zero();
  let mut material: Material = Material::new(Vec3::zero(), 0.0, Vec3::zero());

  let mut spheres_dist = f32::MAX;
  for sphere in spheres
  {
    if let (true, Some(dist)) = sphere.ray_intersect(orig, dir)
    {
      if dist < spheres_dist
      {
        spheres_dist = dist;
        hit = orig + dir.scale(dist);
        n = (hit - sphere.centre).normalise();
        material = sphere.material;
      }
    };
  }


  let mut checkerboard_dist = f32::MAX;
  if dir.y.abs() > 1e-3
  {
    let plane = Vec3::new(0.0, 10.0, -20.0);
    let plane_size = 10.0;

    let d = -(orig.y + plane.y) / dir.y;

    let pt = orig + dir.scale(d);
    let p = pt - plane;

    if d > 0.0 &&
       d < spheres_dist &&
       p.x.abs() < plane_size &&
       p.z.abs() < plane_size
    {
      checkerboard_dist = d;
      hit = pt;
      n = Vec3::new(0.0, 1.0, 0.0);

      let mods = p.add(plane_size).scale(1.5) % Vec3::new(2.0, 2.0, 2.0);
      material.diffuse_colour = if (mods.x as i32 == 0) == (mods.z as i32 == 0)
      {
        Vec3::new(1.0, 0.0, 0.0)
      }
      else
      {
        Vec3::new(0.0, 0.0, 1.0)
      };
      material.specular_exponent = 20.0;
      material.albedo = Vec3::new(0.1, 0.1, 0.1);
    }
  }

  if spheres_dist.min(checkerboard_dist) < f32::MAX
  {
    Some((hit, n, material))
  }
  else {
    None
  }
}

fn cast_ray(orig: Vec3, dir: Vec3, spheres: &[Sphere], lights: &[Light], depth: u32) -> Option<Vec3>
{
  if depth > 1
  {
    None
  }
  else
  if let Some((point, n, material)) = scene_intersect(orig, dir, spheres)
  {
    let reflect_dir = dir.reflect(n);
    let reflect_orig = point + n.scale((reflect_dir*n).signum() * 1e-3);
    let reflect_colour = cast_ray(reflect_orig, reflect_dir, spheres, lights, depth+1);

    let (diffuse_intensity, specular_intensity) = lights.iter().fold((0.0, 0.0), |(diffuse_intensity, specular_intensity), light|
    {
      let light_dir = (light.position - point).normalise();
      let light_dist = (light.position - point).norm();

      let shadow_orig = point + n.scale((light_dir*n).signum() * 1e-3);
      if let Some((shadow_pt, _, _)) = scene_intersect(shadow_orig, light_dir, spheres)
      {
        if (shadow_pt - shadow_orig).norm() < light_dist
        {
          return (diffuse_intensity, specular_intensity)
        }
      }

      (diffuse_intensity + light.intensity * (light_dir*n).max(0.0),
       specular_intensity + (-(-light_dir).reflect(n)*dir).max(0.0).powf(material.specular_exponent) * light.intensity)
    });

    let mut result = material.diffuse_colour.scale(diffuse_intensity).scale(material.albedo.x) + Vec3::all(specular_intensity * material.albedo.y);
    result = result + if let Some(rc) = reflect_colour { rc } else { Vec3::new(0.4, 0.3, 0.5) }.scale(material.albedo.z);
    Some(result)
  }
  else
  {
    None
  }
}

fn render(spheres: &[Sphere], lights: &[Light]) -> ()
{
  const WIDTH: usize    = 1024;
  const HEIGHT: usize   = 768;

  let mut framebuffer = vec![vec![Vec3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

  for j in 0..HEIGHT
  {
    for i in 0..WIDTH
    {
      framebuffer[j][i] = Vec3::new(
        (j as f32) / (HEIGHT as f32),
        (i as f32) / (WIDTH as f32),
        1.0 - (i as f32) / (WIDTH as f32)
      );
    }
  }

  const FOV: f32 = PI/3.;

  for j in 0..HEIGHT
  {
    for i in 0..WIDTH
    {
      let dir = Vec3
      {
        x:  ((i as f32) + 0.5) - (WIDTH  as f32)*0.5,
        y: -((j as f32) + 0.5) + (HEIGHT as f32)*0.5,
        z: -(HEIGHT as f32) / (2. * f32::tan(FOV/2.))
      }.normalise();

      if let Some(colour) = cast_ray(Vec3::zero(), dir, spheres, lights, 0)
      {
        framebuffer[j][i] = colour;
      }
    }
  }

  let path = Path::new("out.ppm");
  let display = path.display();

  let mut file = match File::create(&path)
  {
    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
    Ok(file) => file,
  };

  const N_COLOURS: usize = 255;

  match write!(file, "P6\n{} {}\n{}\n", WIDTH, HEIGHT, N_COLOURS)
  {
    Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    Ok(_)    => {
      let mut buff = vec![0 as u8; WIDTH*HEIGHT*3];
      for (y, row) in framebuffer.iter_mut().enumerate()
      {
        for (x, pixel) in row.iter_mut().enumerate()
        {
          // Rescale if any channel is greater than 1
          pixel.scale(1.0/(1.0 as f32).max(pixel.x).max(pixel.y).max(pixel.z));

          buff[3 * (y * WIDTH + x) + 0] = (pixel.x.max(0.0).min(1.0) * (N_COLOURS as f32)) as u8;
          buff[3 * (y * WIDTH + x) + 1] = (pixel.y.max(0.0).min(1.0) * (N_COLOURS as f32)) as u8;
          buff[3 * (y * WIDTH + x) + 2] = (pixel.z.max(0.0).min(1.0) * (N_COLOURS as f32)) as u8;
        }
      }

      match file.write_all(&buff)
      {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_)    => println!("successfully wrote to {}", display),
      };
    }
  };
}

fn main() {
  let grey   = Material::new(Vec3::new(0.4, 0.4, 0.3), 50.0, Vec3::new(0.6,  0.3, 0.0));
  let red    = Material::new(Vec3::new(0.3, 0.1, 0.1), 10.0, Vec3::new(0.9,  0.1, 0.0));
  let mirror = Material::new(Vec3::one(), 1425.0, Vec3::new(0.0, 10.0, 0.8));

  let spheres = [
    Sphere
    {
      centre: Vec3::new(-3., 0., -16.),
      radius: 2.,
      material: grey
    },
    Sphere
    {
      centre: Vec3::new(-1., -1.5, -12.),
      radius: 2.,
      material: mirror
    },
    Sphere
    {
      centre: Vec3::new( 1.5, -0.5, -18.),
      radius: 3.,
      material: red
    },
    Sphere
    {
      centre: Vec3::new( 7., 5., -18.),
      radius: 4.,
      material: mirror
    },
    Sphere
    {
      centre: Vec3::new( -20.0, 0.0, -50.),
      radius: 30.,
      material: mirror
    }
  ];

  let lights = [
    Light
    {
      position: Vec3::new(-20., 20.,  20.),
      intensity: 1.5
    },
    Light
    {
      position: Vec3::new( 30., 50., -25.),
      intensity: 1.8
    },
    Light
    {
      position: Vec3::new( 30., 20.,  30.),
      intensity: 1.7
    }
  ];

  render(&spheres, &lights);
}
