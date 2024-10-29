use clap::ArgMatches;

pub fn match_crop(crop_args: Option<&ArgMatches>) {
    if let Some(args) = crop_args {
        // initialize required variables
        let height_input = args.get_one::<u32>("height-input").unwrap();
        let width_input = args.get_one::<u32>("width-input").unwrap();
        let x_input = args.get_one::<u32>("x-input").unwrap();
        let y_input = args.get_one::<u32>("y-input").unwrap();
        let infile_input = args.get_one::<String>("infile-input").unwrap();
        let outfile_input = args.get_one::<String>("outfile-input").unwrap();

        // error handling for opening image
        match image::open(infile_input) {
            Ok(mut img) => {
                println!("Cropping and saving image...");
                // cropping image
                let img2 = img.crop(*x_input, *y_input, *width_input, *height_input);
                // error handling for saving image
                match img2.save(outfile_input) {
                    Ok(_) => println!("Successfully saved cropped image to {}", outfile_input),
                    Err(e) => {
                        eprintln!("Failed to save blurred image to {}: {:?}", outfile_input, e)
                    }
                }
            }
            Err(e) => eprintln!("Failed to open file for {}: {:?}", infile_input, e),
        }
    }
}
