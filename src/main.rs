//use std::io::Cursor;
use image::io::Reader as ImageReader;
//use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::collections::HashMap;

//use colorsys::{ColorAlpha, Hsl, Rgb};
use image::GenericImageView;
use lab::Lab;
use owo_colors::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "./test.png";
    let mut cols: HashMap<i32, (Lab, u32)> = HashMap::new();
    //println!("{} and {}", file!(), module_path!());

    let img = ImageReader::open(file)?.decode()?;
    //let (rgbas, _) = match img.as_rgba8()  {
    //    Some(s) => s,
    //    None => panic!(),
    //};
    //let (a, b, image::Rgba(rgbas)) = img.pixels();
    //let lab = Lab::rgbs_to_lab(rgbas);
    //let mut prev: [u8; 4] = [0, 0, 0, 0];
    let mut prev = Lab::default();

    //Iterate over all pixels in the image.
    for (_, _, image::Rgba(a)) in img.pixels() {
        let lab = Lab::from_rgba(&a);
        //println!("This is: '{lab:?}'");
        let delta_e: i32 = (
            (
              ((prev.l - lab.l) as i32 ^ 2)
            + ((prev.a - lab.a) as i32 ^ 2)
            + ((prev.b - lab.b) as i32 ^ 2)
            )
            as f32).sqrt() as i32;
        println!("{}", delta_e);
        //let sum: u32 = (lab.l + lab.a + lab.b) as u32;
        match cols.get(&delta_e) {
            Some(s) => {
                let (_, count) = s;
                cols.insert(delta_e, (lab, count + 1))
            },
            None => cols.insert(delta_e, (lab, 1)),
        };
        prev = lab;
        //println!("R: {} | G: {} | B: {} | A: {}", a[0], a[1], a[2], a[3]);
        //     // Do something with pixel.
    }

    for (i, (colab, count)) in &cols {
        let rgb = colab.to_rgb();
        println!("{} and {:?} {} and hex: {:?}", i, colab, count, rgb.on_color(Rgb(rgb[0], rgb[1], rgb[2])));
    }

    Ok(())
}
