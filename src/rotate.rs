use clap::ArgMatches;
use image::DynamicImage;

pub fn match_rotate(rotate_args: Option<&ArgMatches>) {
    if let Some(args) = rotate_args {
        // initialize required variables
        let int_input = args.get_one::<i32>("int-input").unwrap();
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(img) => {
                println!("Rotating and saving image...");
                // rotating image
                let img2: DynamicImage;
                if int_input == &180 {
                    img2 = img.rotate90()
                } else if int_input == &270 {
                    img2 = img.rotate270();
                } else {
                    img2 = img.rotate90();
                }

                // error handling for saving image
                match img2.save(outfile_input) {
                    Ok(_) => println!("Successfully saved rotated image to {}", outfile_input),
                    Err(e) => {
                        eprintln!("Failed to save rotated image to {}: {:?}", outfile_input, e)
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
