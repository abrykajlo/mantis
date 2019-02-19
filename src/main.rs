mod math;

use math::ray::Ray;
use math::vec::{Vec3, dot};

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(
                &origin,
                &lower_left_corner + &horizontal * u + &vertical * v,
            );
            let col = color(&r);
            let ir: i32 = (255.99 * col[0]) as i32;
            let ig: i32 = (255.99 * col[1]) as i32;
            let ib: i32 = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0f32);
    &Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + &Vec3::new(0.5, 0.7, 1.0) * t
}
