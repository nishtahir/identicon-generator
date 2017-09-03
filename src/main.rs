extern crate image;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::fs::File;
use std::path::Path;
use std::vec::Vec;

fn main() {
    let input = "nishtahir";
    let mut sha = Sha256::new();
    let mut digest = vec![0u8; sha.output_bytes()];
    
    sha.input_str(input);
    sha.result(digest.as_mut_slice());

    let image_size: u32 = 500;
    let tiles_per_row: usize = 5;
    let tile_size = image_size / tiles_per_row as u32;


    let mut image_buffer = image::ImageBuffer::new(image_size, image_size);
    
    let color = [digest[0], digest[1], digest[2]];
    let white = [254, 254, 254];

    let mut tiles = vec![vec![[0; 3]; tiles_per_row]; tiles_per_row];

    for i in 0..5 {
        for j in 0..5 {
            let index = i * 4 + j;
            tiles[i][j] = if digest[index] % 2 == 0 { color} else { white }
        }
    }
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let rgb = tiles[xx as usize][yy as usize];

        *pixel = image::Rgb(rgb);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    let _ = image::ImageRgb8(image_buffer).save(fout, image::PNG);
}
