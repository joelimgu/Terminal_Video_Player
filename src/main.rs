
fn main() {
    play();
}


use termion;
use termion::{color, cursor};
use ffmpeg_frame_grabber::{FFMpegVideo, FFMpegVideoOptions};
use image_visualizer::{visualizer::view, VisualizableImage};
use std::{path::Path, time::Duration};
use std::thread;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};

//https://crates.io/crates/blockish-caca
//https://www.youtube.com/watch?v=KurlpZsmDTQ

use std::path::PathBuf;

// fn play_file(input: PathBuf) -> Result<(), Error> {
//     let decoder = ffmpeg_decoder::Decoder::open(&input)?;

//     let device = rodio::default_output_device().unwrap();
//     let sink = Sink::new(&device);

//     sink.append(decoder);
//     sink.play();
//     sink.sleep_until_end();

//     Ok(())
// }

fn gray_to_ascii(color : u8) -> char {
    //gray scale in ascii
    let gray_scale_ref = ['@','B','%','8','&','M','#','*','o','a','h','k','b','d','p','q','w','m','Z','O','Q','L','C','J','U','Y','X','z','c','v','u','n','x','r','j','f','t','/','\\','|','(',')','1','{','}','[',']','?','-','_','+','~','i','!','l','I',';',':',',','"','^','`','.',' '];

    //get the index of the ascii caracter
    let gray_scale_index = gray_scale_ref.len() as u32 - ((color as u32*gray_scale_ref.len() as u32)/256) -1;

    //return the char
    gray_scale_ref[gray_scale_index as usize]
}


fn print_ascii_img(screen_size : (u16,u16), scale : (i32,i32), img: image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>) {
    for i in 0..screen_size.1 {
        for n in 0..(screen_size.0){
            //test if the coord is in the img adn dont' dyspalyanything if it gets out
            let is_in_img = {
                (n as u32 * scale.0 as u32) < (img.dimensions().0 as u32)
                    &&
                    (i as u32 * scale.1 as u32) < (img.dimensions().1 as u32)
            };

            //go to the
            print!("{}{}", cursor::Hide, cursor::Goto(n + 1, i + 1));
            if is_in_img {
                // print!("{}", if img.get_pixel(n as u32 * scale.0 as u32, i as u32 * scale.1 as u32).0[0] == 0 { "_" } else {"@"} );
                print!("{}", gray_to_ascii(img.get_pixel(n as u32 * scale.0 as u32, i as u32 * scale.1 as u32).0[0]));
            } else {
                print!(" ");
            }
        };
    };
}


fn get_scale(img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) -> (i32,i32) {
    let screen_size = termion::terminal_size().expect("Failed to read screen size");

    //let's find how much the img is scaled % to the terminal in each direction
    let scale = (
        (img.dimensions().0 / screen_size.0 as u32) as i32,
        (img.dimensions().1 / screen_size.1 as u32) as i32
    );

    //now we take the smallest scate to avoid distorting teh img
    (
        if 2*scale.0 > scale.1 { scale.0 } else {scale.1},
        if 2*scale.0 > scale.1 { scale.0 } else {scale.1}
    )
}


fn print_img(img: image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>) {
    let screen_size = termion::terminal_size().expect("Failed to read screen size");
    let scale = get_scale(&img);

    for line in 0..screen_size.1 {
        // thread::spawn(|| { /* code to execute in the thread */});
        for col in 0..(screen_size.0){
            //test if the coord is in the img adn dont' dyspalyanything if it gets out
            let is_in_img = {
                (col as u32 * scale.0 as u32) < (img.dimensions().0 as u32)
                    &&
                    (line as u32 * scale.1 as u32) < (img.dimensions().1 as u32)
            };

            //go to the top left, the x2 is bc the unicode char is 2 times as tall than wide
            //so we need the print 2 char to create a pixel
            print!("{}{}", cursor::Hide, cursor::Goto(2*col + 1, line + 1));

            if is_in_img {
                let r = img.get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32).0[0];
                let g = img.get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32).0[1];
                let b = img.get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32).0[2];

                // print!("{}", if img.get_pixel(n as u32 * scale.0 as u32, i as u32 * scale.1 as u32).0[0] == 0 { "_" } else {"@"} );
                print!("{}\u{2588}",color::Fg(color::Rgb(r,g,b)));
                print!("{}\u{2588}",color::Fg(color::Rgb(r,g,b)));
            } else {
                print!(" ");
            }
        };

    };
}


fn play(){

    let path = "src/lake.jpeg";

    let screen_size = termion::terminal_size().expect("Failed to read line");

    //let img = image::open(path).expect("a problem opening the image").to_luma8();
    //let img = image::open(path).expect("a problem opening the image").to_rgb8();

    //println!("screensize: {:?}", screen_size);
    // println!("img size:{:?}", img.dimensions());


    let video = FFMpegVideo::open(
        Path::new(&"./src/Nyan_Cat.mp4"),
        //extract a frame every X duration ( why not working with less than a sec? 😭)
        //FFMpegVideoOptions::default().with_sampling_interval(Duration::from_millis(1000)),
        FFMpegVideoOptions::default()
    )
        .unwrap();


    let mut count = 0;
    println!("{}",termion::clear::All);
    for frame in video {
        let f = frame.unwrap();
        // println!("offset: {:?}", f.time_offset);
        // view!(&f.image.visualize());
        let a = f.image;
        let handle = thread::spawn(|| {print_img(a);});
        thread::sleep(Duration::from_millis(1/30*1000 as u64));
        handle.join().unwrap();
        count += 1; //count the number of frames rendered in total
    }
    println!("{}",count);


    //let's find how much the img is scaled % to the terminal in each direction
    // let scale = (
    //     (img.dimensions().0 / screen_size.0 as u32) as i32,
    //     (img.dimensions().1/ screen_size.1 as u32) as i32
    // );


    //now we take the smallest scale to avoid distorting the img
    // let scale = (
    //     if scale.0 > scale.1 { scale.0 } else {scale.1},
    //     if scale.0 > scale.1 { scale.0 } else {scale.1}
    // );

    // println!("scale: {:?}", img.get_pixel(1, 1));

    //print_ascii_img(screen_size,scale,img);
    //print_img(img);
    // print!("{}\u{2588}", color::Fg(color::Red));
    // print!("{}\u{2588}", color::Fg(color::Rgb(0,0,0)));
}

