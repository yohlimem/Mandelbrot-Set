use image::Rgb;
pub(crate) use image::{codecs::gif::GifEncoder, ImageBuffer, Frame, Rgba, Delay};
use num::complex::Complex;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::sync::{Mutex, Arc};
use std::thread;


fn main() {

    use std::time::Instant;
    let now = Instant::now();


    let width = 900;
    let height = 600;
    let zoom = 500.0;
    let x_offset = 900. / zoom;
    let y_offset = 0.0 / zoom;
    
    let image_path = "hello.gif";
    let frame_amount = 26;
    let thread_workers = 12;
    
    let file = BufWriter::new(File::create(image_path).unwrap());
    
    // Create a vector to store frames
    let frames = Arc::new(Mutex::new(Vec::new()));
    let i = Arc::new(Mutex::new(1 as i64));

    let mut handles = vec![];
    
    // Generate a series of frames (e.g., a simple animation)
    for _ in 0..thread_workers {
        let frames_clone = Arc::clone(&frames); // Clone the Arc to use in the new thread
        let counter_clone = Arc::clone(&i); // Clone the Arc to use in the new thread
        let handle = thread::spawn(move || {
            let mut counter = counter_clone.lock().unwrap();
            let mut frames = frames_clone.lock().unwrap(); // Lock the Mutex before accessing it
            for _ in 0..frame_amount/thread_workers{
                *counter += 4;
                let zoom = (*counter * 100) as f64;
                // println!("{}", zoom);
                let frame = ImageBuffer::from_fn(width, height, |x,y| {
                    let done = mandelbrot(Complex { re: ((width as f64 / 2.0) - x as f64) / zoom - x_offset, im: ((height as f64 / 2.0) - y as f64) / zoom + y_offset }, None) as u8;
                    if done == 255{
                        Rgba([0,0,0,255])

                    } else {
                        linear_color_interpolation(Rgba([3, 252, 15,255]), Rgba([252, 3, 3,255]), done)

                    }
                });
                let frame_duration = Delay::from_numer_denom_ms(1, 100); // 100 centiseconds = 1 second
                let gif_frame = Frame::from_parts(frame, width, height,  frame_duration);
                
                // Add the frame to the list of frames
                frames.push(gif_frame); // Push the frame into the frames vector
                println!("frame number: {}", frames.len());
            }
        });
        handles.push(handle);
        
        
        // Create a new frame and set its duration (in 1/100ths of a second)

        
    }

    for handle in handles{
        handle.join().unwrap();
    }


    // Create a GIF encoder
    let mut encoder = GifEncoder::new(file);
    encoder.set_repeat(image::gif::Repeat::Infinite);

    // Lock the Mutex and get a reference to the frames
    let frames = frames.lock().unwrap();

    // Write the frames to the GIF file
    for frame in frames.iter() {
        encoder.encode_frame(frame.clone()).unwrap();
    }

    // Open the GIF file in Windows
    // let gif_path = Path::new(&image_path);
    // if gif_path.exists() {
    //     Command::new("explorer.exe")
    //     .arg("/select,")
    //     .arg(gif_path.to_str().unwrap())
    //     .status()
    //     .expect("Failed to open the GIF file");
    // } else {
    //     println!("The GIF file does not exist at the specified path.");
    // }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}


fn mandelbrot(complex: Complex<f64>, max_iterations: Option<u32>) -> u32 {
    // println!("{:?}",complex);
    let max_iter: u32 = max_iterations.unwrap_or(255); // Default to 255 if max_iterations is not provided
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);

    for i in 0..max_iter {
        z = z * z + complex;
        if z.norm_sqr() > 4.0 {
            return i as u32;
        }
    }

    max_iter as u32
}

fn linear_color_interpolation(rgb1: Rgba<u8>, rgb2: Rgba<u8>, value: u8) -> Rgba<u8> {
    let r = rgb1[0] as f64 + (rgb2[0] as f64 - rgb1[0] as f64) * (value as f64 / 255.0);
    let g = rgb1[1] as f64 + (rgb2[1] as f64 - rgb1[1] as f64) * (value as f64 / 255.0);
    let b = rgb1[2] as f64 + (rgb2[2] as f64 - rgb1[2] as f64) * (value as f64 / 255.0);
    Rgba([r as u8, g as u8, b as u8, 255])
}
