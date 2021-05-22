use colors_transform::{Color, Rgb};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::path::Path;
use std::vec::Vec;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "font2img",
    version = "v1.0.0",
    about = "A tool for converting TTF icon font to images.\n\n\
    example: \n\
    \tfont2png --charter $(printf '\\ue957') -s 80 -f a -o src/assets/on/user.png -c \"#d43c33\"\n\
    ",
    author = "Fanjiankong <kfanjian@gmail.com>"
)]
struct Options {
    #[structopt(short = "f", long = "font", required = true, help = "font file path")]
    fontpath: String,
    #[structopt(
        short = "o",
        long = "output",
        required = true,
        help = "output filename"
    )]
    output: String,
    #[structopt(
        // short = "i",
        long = "charter",
        required = true,
        // default_value = "\u{e966}",
        help = "icon charter"
    )]
    charter: String,
    #[structopt(short = "t", long = "transparent", help = "transparent background")]
    transparent: bool,
    #[structopt(short = "c", long = "color", help = "icon css style color")]
    color: String,
    #[structopt(
        short = "s",
        long = "size",
        default_value = "78",
        help = "output image's height and width(pixel)"
    )]
    size: u16,
    #[structopt(
        short = "S",
        long = "iconsize",
        default_value = "54",
        help = "icon's height and width(pixel)"
    )]
    iconsize: f32,
}
fn main() {
    let options = Options::from_args();
    println!("{:?}", options);

    let output_path = Path::new(&options.output);

    let mut image = RgbaImage::new(options.size as u32, options.size as u32);

    let fontbytes = Vec::from(include_bytes!("icon.ttf") as &[u8]);
    let size = fontbytes.len();
    let font = Font::try_from_vec(fontbytes).unwrap();
    println!("font file size: {}, count={}", size, font.glyph_count());

    let scale = Scale {
        x: options.iconsize,
        y: options.iconsize,
    };

    let offset: u32 = ((options.size - options.iconsize as u16) / 2) as u32;

    let color = Rgb::from_hex_str(&options.color).unwrap();

    println!(
        "color: {:?}, rgb=({},{},{})",
        color,
        color.get_red(),
        color.get_green(),
        color.get_blue(),
    );
    draw_text_mut(
        &mut image,
        Rgba([
            color.get_red() as u8,
            color.get_green() as u8,
            color.get_blue() as u8,
            255u8,
            // 0u8,
        ]),
        offset,
        offset,
        scale,
        &font,
        &options.charter,
    );
    // let (w, h) = text_size(scale, &font, text);
    // println!("Text size: {}x{}", w, h);

    println!("save to {}", options.output);
    let _ = image.save(output_path).unwrap();
}
