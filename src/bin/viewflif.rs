
// based on    https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/image-demo.rs
//  cargo build --bin  viewflif --features="viewflif" 
//  RUST_BACKTRACE=1 ./target/debug/viewflif  ./resources/sea_snail_cutout.png

extern crate flif;
extern crate png;
extern crate structopt;
// #[macro_use]
//extern crate structopt_derive;
extern crate sdl2;
extern crate image;

use image::ImageBuffer;

//use std::io::{Read, Write};
//use std::io::BufWriter;

//use std::fs::File;
//use png::HasParameters;


//use flif::components::Channels;
//use flif::error::*;
//use flif::Decoder;
//use flif::*;

use sdl2::rwops::RWops;
//use structopt::StructOpt;



use std::env;
use std::path::Path;
use sdl2::image::INIT_PNG;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::rwops;
use sdl2::image::ImageRWops; // trait

pub fn run(png: &Path) {

    let file = std::fs::File::open("~/vcs/github/flif.rs/resources/sea_snail_cutout.flif").unwrap();
    let mut decoder = flif::Decoder::new(file);
    let flif = decoder.decode().unwrap();
    let flifdata = flif.get_raw_pixels();
//  image: 122x128
    let rawpixel_flif: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(122,128, flifdata).unwrap();
    // crash: thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: UnsupportedError("Unsupported image format")', /checkout/src/libcore/result.rs:916:5


    //println!("{:?}", rawpixel_flif);
    let image: image::DynamicImage = image::load_from_memory(&rawpixel_flif).unwrap();
    let imgrgb: image::RgbImage  = image.to_rgb();
// cargo build --features  viewflif

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

      // sdl:
      let rwops = RWops::from_bytes(&imgrgb).unwrap();


     let flifpng = rwops.load_png();

     let flif_surface = flifpng.unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();


    //@here load into sdl2
//    let foo = sdl2::image::LoadTextur
    //let texture = texture_creator.
//    let texture = texture_creator.load_texture(png).unwrap();
        let texture = texture_creator.create_texture_from_surface(&flif_surface).unwrap();


    canvas.copy(&texture, None, None).expect("Render failed");
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}


fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        run(Path::new(&args[1]));
    }
}
