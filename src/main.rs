use quad_rand::compat::QuadRand;
use quad_rand::rand;
use quad_rand::srand;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

fn main() -> std::io::Result<()> {
    let mut res_bytes = vec![];
    srand(12345);

    for _ in 0..104900000 / 4 {
        let x = rand();
        let x = transform_u32_to_array_of_u8(x);
        for b in x.iter() {
            res_bytes.push(*b);
        }
    }
    // let res_bytes: Vec<_> = res_bytes.into_iter().map(|x| x as u8).collect();
    let mut res = File::create("result_transform.bin")?;
    res.write_all(&res_bytes)?;
    let mut res = File::create("result_rng.bin")?;
    res_bytes.clear();
    dbg!("I'm here");
    for _ in 0..104900000 / 4 {
        let x: u8 = Rng::gen(&mut QuadRand);
        res_bytes.push(x);
    }
    res.write_all(&res_bytes)?;

    Ok(())
}
