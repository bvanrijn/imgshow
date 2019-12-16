use base64::encode;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::process;

#[derive(Debug, Default)]
struct InlineImageBuilder {
    name: String,
    size: usize,
    width: String,
    height: String,
    preserve_aspect_ratio: bool,
    inline: bool,
    data: Vec<u8>,
}

impl Display for InlineImageBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let esc = "\x1b";
        let bel = "\x07";

        write!(f, "{}]1337;File=", esc).expect("Can't write header");

        if self.name != "" {
            write!(f, "name={};", encode(&self.name)).expect("Can't write `name`");
        }

        if self.size != 0 {
            write!(f, "size={};", self.size.to_string()).expect("Can't write `size`");
        }

        if self.width != "" {
            write!(f, "width={};", self.width).expect("Can't write `width`");
        }

        if self.height != "" {
            write!(f, "height={};", self.height).expect("Can't write `height`");
        }

        if self.preserve_aspect_ratio {
            write!(f, "preserveAspectRatio=1;").expect("Can't write `preserveAspectRatio`");
        } else {
            write!(f, "preserveAspectRatio=0;").expect("Can't write `preserveAspectRatio`");
        }

        if self.inline {
            write!(f, "inline=1;").expect("Can't write `inline`");
        } else {
            write!(f, "inline=0;").expect("Can't write `inline`");
        }

        write!(f, ":{}{}", encode(&self.data), bel)
    }
}

impl InlineImageBuilder {
    fn new() -> Self {
        Self {
            preserve_aspect_ratio: true,
            ..Default::default()
        }
    }

    fn set_data(&mut self, v: Vec<u8>) -> &mut Self {
        self.data = v;
        self
    }

    fn set_inline(&mut self, v: bool) -> &mut Self {
        self.inline = v;
        self
    }
}

enum UnsupportedReason {
    /// Running inside Terminal.app is not supported
    AppleTerminal,
    /// $TERM_PROGRAM is not set
    EnvNotSet,
    /// Support for this terminal is unknown
    Unknown,
}

fn is_supported_terminal() -> Result<(), UnsupportedReason> {
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
    match is_supported_terminal() {
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
