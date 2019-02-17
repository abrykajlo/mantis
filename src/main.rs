mod math;

use math::vec;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = vec::Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            
            let ir: i32 = (255.99 * col[0]) as i32;
            let ig: i32 = (255.99 * col[1]) as i32;
            let ib: i32 = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
