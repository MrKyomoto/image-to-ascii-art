use std::path::PathBuf;

use image::{DynamicImage, GenericImageView};

fn main() {
    let mut path = std::env::current_dir().unwrap();
    path.push("avator.jpg");
    println!("{:?}",path);
    let img = get_image(path);
    let ascii_art = image_to_ascii_art(img, 80);
    println!("{}",ascii_art);
}

// DynamicImage 是将RGBA通道的图片转换成像素矩阵的形式
fn get_image(dir: PathBuf) -> DynamicImage{
    image::open(dir.to_str().unwrap_or("."))
        .expect("Faild to open image")
}

// width用于对图片进行等比例缩放
fn image_to_ascii_art(img: DynamicImage, width: u32) -> String {
    let (width_orign,height_orign) = img.dimensions();

    let scale = width as f32 / width_orign as f32;

    let height = (scale * height_orign as f32 * 0.45) as u32;

    let img_resized = img.resize_exact(width,height,image::imageops::FilterType::Nearest);

    // 获取灰度图
    let img_resized_gray = img_resized.to_luma8();

    // from 0.0 to 1.0 (darkest to brightest)
    let ascii_chars = [" ",".",",","~","-","+","*","&","%","$","@"];

    // 初始化一个带有指定容量的空字符串
    let mut ascii_art = String::with_capacity((width * height) as usize); 

    for y in 0..height{
        for x in 0..width{
            let pixel = img_resized_gray.get_pixel(x, y);
            let luma = pixel[0] as f32 / 255.0;
            let index = (luma * (ascii_chars.len() - 1) as f32) as usize;
            
            ascii_art.push_str(ascii_chars[index]);
        }
        ascii_art.push('\n');
    };
    ascii_art
}