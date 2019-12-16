mod inline_image_builder;

use crate::inline_image_builder::InlineImageBuilder;
use std::env;
use std::process;

enum UnsupportedReason {
    /// Running inside Terminal.app is not supported
    AppleTerminal,
    /// $TERM_PROGRAM is not set
    EnvNotSet,
    /// Support for this terminal is unknown
    Unknown,
}

fn get_terminal_support() -> Result<(), UnsupportedReason> {
    let term_program_env = env::var("TERM_PROGRAM");

    if term_program_env.is_err() {
        return Err(UnsupportedReason::EnvNotSet);
    }

    match term_program_env.unwrap().as_ref() {
        "iTerm.app" => Ok(()),
        "Apple_Terminal" => Err(UnsupportedReason::AppleTerminal),
        _ => Err(UnsupportedReason::Unknown),
    }
}

fn main() {
    match get_terminal_support() {
        Ok(_) => (),
        Err(UnsupportedReason::AppleTerminal) => {
            eprintln!("Apple's Terminal application is not supported. Please use iTerm2 (https://www.iterm2.com) and try again.");
            process::exit(1);
        }
        Err(UnsupportedReason::EnvNotSet) => {
            eprintln!("The TERM_PROGRAM environment variable is not set.");
            process::exit(2);
        }
        Err(UnsupportedReason::Unknown) => {
            eprintln!("Your terminal is unsupported for an unknown reason. If your terminal supports images, please let me know by opening an issue.");
            process::exit(3);
        }
    }

    let mut image = InlineImageBuilder::new();
    image
        .set_data(include_bytes!("../image.jpg").to_vec())
        .set_inline(true);
    println!("{}", image);
}
