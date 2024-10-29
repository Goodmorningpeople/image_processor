use clap::ArgMatches;

pub fn match_generate(generate_args: Option<&ArgMatches>) {
    if let Some(args) = generate_args {
        // initialize required variables
        let red_input = *args.get_one::<u8>("red-input").unwrap();
        let blue_input = *args.get_one::<u8>("blue-input").unwrap();
        let green_input = *args.get_one::<u8>("green-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();
        
        let width = 800;
        let height = 800;
    
        let mut imgbuf = image::ImageBuffer::new(width, height);
    
        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb([red_input, green_input, blue_input]);
        }
        imgbuf.save(outfile_input).unwrap();
    }
}
