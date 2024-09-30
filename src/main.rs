use image::{imageops::ColorMap, Pixel};
fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let value = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            blur_func(value, infile, outfile);
        }

        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let value = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            brighten_func(value, infile, outfile);
        }

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let x = args.remove(0);
            let y = args.remove(0);
            let width = args.remove(0);
            let height = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            crop_func(x, y, width, height, infile, outfile);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let option: i32 = args
                .remove(0)
                .parse()
                .expect("Failed to parse String to i32");
            let infile = args.remove(0);
            let outfile = args.remove(0);
            match option {
                1 => rotate_func90(infile, outfile),
                2 => rotate_func180(infile, outfile),
                3 => rotate_func270(infile, outfile),
                _ => print_usage_and_exit(),
            }
        }

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert_func(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale_func(infile, outfile);
        }
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }
        "generate" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let red = args.remove(0);
            let green = args.remove(0);
            let blue = args.remove(0);
            let outfile = args.remove(0);
            generate(outfile, red, green, blue);
        }
        _ => {
            print_usage_and_exit();
        }
        
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!(
        "blur <image> <output>: blur image 
brighten <image> <integer> <output>: brighten image (negative integers darken image, positive integers brighten image) 
crop <image> <x> <y> <width> <height> <output>: crop image 
rotate <image> <integer> <output>: rotate image (integer = 1 rotates by 90 degrees, integer = 2 rotates by 180 degrees and integer 270 degrees) 
invert <image> <output>: invert imag\
grayscale <image> <output>: grayscale image
generate <red> <green> <blue> <output>: generates a 800x800 solid image
fractal <output>: generates a simple 800x800 fractal image
        "
        
    );
    std::process::exit(-1);
}

fn blur_func(value: String, infile: String, outfile: String) {
    let value: f32 = value.parse().expect("Failed to parse String to f32");
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(value);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten_func(value: String, infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let value: i32 = value.parse().expect("Failed to parse String into i32");
    let img2 = img.brighten(value);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop_func(x: String, y: String, width: String, height: String, infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let x: u32 = x.parse().expect("Failed to parse String into u32");
    let y: u32 = y.parse().expect("Failed to parse String into u32");
    let width: u32 = width.parse().expect("Failed to parse String into u32");
    let height: u32 = height.parse().expect("Failed to parse String into u32");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate_func90(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.rotate90();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate_func180(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.rotate180();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate_func270(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.rotate270();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert_func(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale_func(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    img.grayscale();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, red: String, green: String, blue: String) {
    let width = 800;
    let height = 800;

    let red: u8 = red.parse().expect("Failed to parse String into u8");
    let green: u8 = green.parse().expect("Failed to parse String into u8");
    let blue: u8 = blue.parse().expect("Failed to parse String into u8");
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(outfile).unwrap();
}
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}


