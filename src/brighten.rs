use clap::ArgMatches;

pub fn match_brighten(brighten_args: Option<&ArgMatches>) {
    if let Some(args) = brighten_args {
        let mut value = 0;
        // initialize required variables
        if let Some(positive_input) = args.get_one::<i32>("positive-input") {
            value = *positive_input;
        } else if let Some(negative_input) = args.get_one::<i32>("negative-input") {
            value = 0 - negative_input;
        } else {
            println!("As no input was provided, value will default to 1");
        };
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(img) => {
                println!("Brightening and saving image...");
                // brightening image
                let img2 = img.brighten(value);
                // error handling for saving image
                match img2.save(outfile_input) {
                    Ok(_) => println!("Successfully saved brightened image to {}", outfile_input),
                    Err(e) => {
                        eprintln!(
                            "Failed to save brightened image to {}: {:?}",
                            outfile_input, e
                        )
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
