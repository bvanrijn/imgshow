use base64::encode;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct InlineImageBuilder {
    name: String,
    size: usize,
    width: String,
    height: String,
    preserve_aspect_ratio: bool,
    inline: bool,
    data: Vec<u8>,
}

impl InlineImageBuilder {
    pub fn new() -> Self {
        Self {
            preserve_aspect_ratio: true,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn set_name(&mut self, v: String) -> &mut Self {
        self.name = v;
        self
    }

    #[allow(dead_code)]
    pub fn set_size(&mut self, v: usize) -> &mut Self {
        self.size = v;
        self
    }

    #[allow(dead_code)]
    pub fn set_width(&mut self, v: String) -> &mut Self {
        self.width = v;
        self
    }

    #[allow(dead_code)]
    pub fn set_height(&mut self, v: String) -> &mut Self {
        self.height = v;
        self
    }

    #[allow(dead_code)]
    pub fn set_preserve_aspect_ratio(&mut self, v: bool) -> &mut Self {
        self.preserve_aspect_ratio = v;
        self
    }

    pub fn set_inline(&mut self, v: bool) -> &mut Self {
        self.inline = v;
        self
    }

    pub fn set_data(&mut self, v: Vec<u8>) -> &mut Self {
        self.data = v;
        self
    }
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
