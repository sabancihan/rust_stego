use image::{ImageBuffer, Rgba};
use base64::{engine::general_purpose, Engine as _};
use std::io::Cursor;


pub(super) fn hide_secret(input: String,img: String) -> String {
    let bytes = general_purpose::STANDARD.decode(img).unwrap();
    let mut img = image::load_from_memory(&bytes).unwrap().to_rgba8();
    hide_secret_image(input, &mut img);

    let mut result = vec![];
    img.write_to(&mut Cursor::new(&mut result), image::ImageOutputFormat::Png).unwrap();

    let base64_image = general_purpose::STANDARD.encode(&result);
    base64_image

}

fn hide_secret_image(input: String, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)  {

    let dimensions = img.dimensions();

    let size = dimensions.0 * dimensions.1;

    let len = input.len();

    if len > ((size * 8/3) as usize) {
        panic!("Image is too small to hide the message");
    }

    let mut count = 0 as usize;

    let  input = input.chars().collect::<Vec<char>>();

    for (_,_, pixel) in img.enumerate_pixels_mut() {

        let char_pos = count / 8;

        if char_pos >= len {
            pixel[0] = pixel[0] & 0b11111110;
            break;
        }

        for i in 0..3 {

            let curr = input[char_pos] as u8;
          
            let bit = (curr >> (7 - count % 8)) & 1;

            pixel[i] = (pixel[i] & 0b11111110) | bit;

        }

        count += 3;
    }
    
   
}


fn bin_str_to_word(bin_str: &str) -> String {
    bin_str.split(" ")
    .map(|n| u32::from_str_radix(n, 2).unwrap())
    .map(|c| char::from_u32(c).unwrap())
    .collect()
}




fn find_secret_image(img:  ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {

    let mut hidden_binary = String::new();

    
    for (_,_, pixel) in img.enumerate_pixels() {
        for i in 0..3 {
            let bit = pixel[i] & 1;
            if bit == 0 {
                return bin_str_to_word(&hidden_binary);
            }
            hidden_binary.push_str(&format!("{}", bit));
            
        }
        
    }
    panic!("No message found");

}

pub(super) fn find_secret(base64_image: String) -> String {

    let bytes = general_purpose::STANDARD.decode(base64_image.as_bytes()).unwrap();
    
    let img = image::load_from_memory(&bytes).unwrap().to_rgba8();

    find_secret_image(img)


}