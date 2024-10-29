use clap::ArgMatches;

pub fn match_fractal(fractal_args: Option<&ArgMatches>) {
    if let Some(args) = fractal_args {
        // initialize required variables
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        println!("Generating fractal...");
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
        match imgbuf.save(outfile_input) {
            Ok(_) => println!("Successfully generated fractal"),
            Err(e) => eprintln!("Failed to generate fractal at {}: {:?}", outfile_input, e),
        }
    }
}
