use clap::ArgMatches;

pub fn match_invert(blur_args: Option<&ArgMatches>) {
    if let Some(args) = blur_args {
        // initialize required variables
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(mut img) => {
                println!("Inverting and saving image...");
                // grayscale image
                img.invert();
                // error handling for saving image
                match img.save(outfile_input) {
                    Ok(_) => println!("Successfully saved inverted image to {}", outfile_input),
                    Err(e) => {
                        eprintln!(
                            "Failed to save inverted image to {}: {:?}",
                            outfile_input, e
                        )
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
