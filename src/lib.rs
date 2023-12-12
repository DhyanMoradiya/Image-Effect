use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use image::{load_from_memory};
use image::ImageOutputFormat::Png;


#[wasm_bindgen]
pub fn grayScale(encoded_file : &str) -> String {
    log(&"GreyScale method is called".into());

    let string_to_vector_image = STANDARD.decode(&encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&string_to_vector_image).unwrap();
    log(&"Image genrated".into());

    img = img.grayscale();
    log(&"gray scale effect applied".into());

    let mut img_buffer= vec![];
    img.write_to(&mut img_buffer, Png).unwrap();
    log(&"Image stored in to buffer".into());

    let  encoded_img = STANDARD.encode(img_buffer);
    log(&"image encoded".into());

    let data_url = format!("data:image/png;base64,{}", encoded_img);
    return data_url;

}