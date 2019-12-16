use base64::encode;
use std::fmt;
use std::fmt::Display;

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

fn main() {
    let mut image = InlineImageBuilder::new();
    image
        .set_data(include_bytes!("../image.jpg").to_vec())
        .set_inline(true);
    println!("{}", image);
}
