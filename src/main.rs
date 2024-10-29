use clap::{command, value_parser, Arg, Command};
use image_processor::{
    blur::match_blur, brighten::match_brighten, crop::match_crop, fractal::match_fractal, generate::match_generate, grayscale::match_grayscale, invert::match_invert, rotate::match_rotate
};

fn main() {
    let match_result = command!()
        .about("Extremely simple image processor made in Rust with basic function like blur and invert, and with more complex functions like generate, which generates a solid color image and fractal, which generates a simple fractal.")
        .subcommand(Command::new("blur").about("blur [value] [infile] [outfile]: takes an floating point argument and blurs a image
")
        .arg(
                Arg::new("fp-input")
                    .required(true)
                    .value_parser(value_parser!(f32))
            )
        .arg(
            Arg::new("infile-input")
                .required(true)
        )
        .arg(
            Arg::new("outfile-input")
                .required(true)
        )
        )
        .subcommand(Command::new("brighten").about("brighten [options] [value] [infile] [outfile]: takes an integer argument and brightens a image
-p: Enter a positive value to brighten the image
-n: Enter a negative value to darken the image
                    ")
                .arg(
                        Arg::new("positive-input")
                            .short('p')
                            .long("positive")
                            .value_parser(value_parser!(i32))
                    )
                .arg(
                        Arg::new("negative-input")
                            .short('n')
                            .long("negative")
                            .value_parser(value_parser!(i32))
                    )
                .arg(
                    Arg::new("infile-input")
                        .required(true)
                )
                .arg(
                    Arg::new("outfile-input")
                        .required(true)
                )
                )
                .subcommand(Command::new("crop").about("brighten [x] [y] [width] [height] [infile] [outfile]: crops an image
")
                        .arg(
                                Arg::new("x-input")
                                    .value_parser(value_parser!(u32))
                                    .required(true)
                            )
                        .arg(
                                Arg::new("y-input")
                                    .value_parser(value_parser!(u32))
                                    .required(true)
                            )
                        .arg(
                                Arg::new("width-input")
                                    .value_parser(value_parser!(u32))
                                    .required(true)
                                )
                        .arg(
                                Arg::new("height-input")
                                    .value_parser(value_parser!(u32))
                                    .required(true)
                                )
                        .arg(
                            Arg::new("infile-input")
                                .required(true)
                        )
                        .arg(
                            Arg::new("outfile-input")
                                .required(true)
                        )
                        )
                .subcommand(Command::new("rotate").about("rotate [value] [infile] [outfile]: takes an integer value of 90, 180 or 270 and rotates a image respectively. If no valid value is given, image will be rotated 90 degrees.
                ")
                        .arg(
                                Arg::new("int-input")
                                    .required(true)
                                    .value_parser(value_parser!(i32))
                            )
                        .arg(
                            Arg::new("infile-input")
                                .required(true)
                        )
                        .arg(
                            Arg::new("outfile-input")
                                .required(true)
                        )
                        ) .subcommand(Command::new("invert").about("invert [infile] [outfile]: inverts the color of an image
")
                    .arg(
                        Arg::new("infile-input")
                            .required(true)
                    )
                    .arg(
                        Arg::new("outfile-input")
                            .required(true)
                    )
                )
                .subcommand(Command::new("grayscale").about("grayscale [infile] [outfile]: grayscales the color of an image
                    ")
                    .arg(
                        Arg::new("infile-input")
                            .required(true)
                    )
                    .arg(
                        Arg::new("outfile-input")
                            .required(true)
                    )
                    )
                .subcommand(Command::new("fractal").about("fractal [outfile]: generates a fractal
                    ")
                    .arg(
                        Arg::new("outfile-input")
                            .required(true)
                    )
                    )
                .subcommand(Command::new("generate").about("generate [red] [blue] [green] [outfile]: generates a solid color image from provided rgb values
                    ")
                    .arg(
                        Arg::new("red-input")
                            .required(true)
                            .value_parser(value_parser!(u8))
                    )
                    .arg(
                        Arg::new("blue-input")
                            .required(true)
                            .value_parser(value_parser!(u8))
                    )
                    .arg(
                        Arg::new("green-input")
                            .required(true)
                            .value_parser(value_parser!(u8))
                        )
                    .arg(
                        Arg::new("outfile-input")
                            .required(true)
                    )
                    )
        .get_matches();

    // matching commands
    let blur_args = match_result.subcommand_matches("blur");
    match_blur(blur_args);

    let brighten_args = match_result.subcommand_matches("brighten");
    match_brighten(brighten_args);

    let crop_args = match_result.subcommand_matches("crop");
    match_crop(crop_args);

    let rotate_args = match_result.subcommand_matches("rotate");
    match_rotate(rotate_args);

    let invert_args = match_result.subcommand_matches("invert");
    match_invert(invert_args);

    let grayscale_args = match_result.subcommand_matches("grayscale");
    match_grayscale(grayscale_args);

    let fractal_args = match_result.subcommand_matches("fractal");
    match_fractal(fractal_args);
    
    let generate_args = match_result.subcommand_matches("generate");
    match_generate(generate_args);
}
