extern crate image;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::fs::File;

use std::env;

const IMAGE_SIZE: u32 = 250;
const TILES_PER_ROW: usize = 5;

fn main() {
    for arg in env::args().skip(1) {
        generate_identicon(&arg);
    }
}

fn generate_identicon(value : &str) {
    let mut sha = Sha256::new();
    let mut digest = vec![0u8; sha.output_bytes()];
    
    sha.input_str(value);
    sha.result(digest.as_mut_slice());

    let tile_size = IMAGE_SIZE / TILES_PER_ROW as u32;


    let mut image_buffer = image::ImageBuffer::new(IMAGE_SIZE, IMAGE_SIZE);
    
    let color = [digest[0], digest[1], digest[2]];
    let white = [254, 254, 254];

    let mut tiles = vec![vec![[0; 3]; TILES_PER_ROW]; TILES_PER_ROW];

    for i in 0..5 {
        for j in 0..5 {
            let index = i * 4 + j;
            tiles[i][j] = if digest[index] % 2 == 0 { color} else { white }
        }
    }

    // mirror image vertically
    for i in 0..2 {
        for j in 0..5 {            
            tiles[4 - i][j] = tiles[i][j];
        }
    }

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let rgb = tiles[xx as usize][yy as usize];

        *pixel = image::Rgb(rgb);
    }

    let ref mut fout = File::create(format!("{}.png", value)).unwrap();
    image::ImageRgb8(image_buffer).save(fout, image::PNG).unwrap();
}
