use clap::ArgMatches;

pub fn match_blur(invert_args: Option<&ArgMatches>) {
    if let Some(args) = invert_args {
        // initialize required variables
        let fp_input = args.get_one::<f32>("fp-input").unwrap();
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(img) => {
                println!("Blurring and saving image...");
                // blurring image
                let img2 = img.blur(*fp_input);
                // error handling for saving image
                match img2.save(outfile_input) {
                    Ok(_) => println!("Successfully saved blurred image to {}", outfile_input),
                    Err(e) => {
                        eprintln!("Failed to save blurred image to {}: {:?}", outfile_input, e)
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
