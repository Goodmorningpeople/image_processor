use clap::ArgMatches;

pub fn match_grayscale(grayscale_args: Option<&ArgMatches>) {
    if let Some(args) = grayscale_args {
        // initialize required variables
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(mut img) => {
                println!("Grayscaling and saving image...");
                // inverting image
                img.grayscale();
                // error handling for saving image
                match img.save(outfile_input) {
                    Ok(_) => println!("Successfully saved grayscaled image to {}", outfile_input),
                    Err(e) => {
                        eprintln!(
                            "Failed to save grayscaled image to {}: {:?}",
                            outfile_input, e
                        )
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
