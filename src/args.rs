use anyhow::Result;
use pico_args::Arguments;

const HELP: &str = "\
ft 0.1.0

Usage:
  ft [OPTIONS] [SHELL]

Arguments:
  [SHELL]  Sets shell to run [default: /usr/bin/bash]

Options:
  -f, --font-path PATH       Sets font path [default: font.ttf]
  -s, --font-size-px NUMBER  Sets font size [default: 16]
  -a, --disable-subpixel-aa  Disables subpixel antialiasing
  -d, --fb-device-path PATH  Sets framebuffer device path [default: /dev/fb0]
  -h, --help                 Prints help information
";

#[derive(Debug)]
pub struct Args {
    pub font_path: String,
    pub font_size_px: u32,
    pub font_subpixel_antialiasing: bool,
    pub framebuffer_device_path: String,
    pub shell_path: String,
}

impl Args {
    pub fn parse() -> Result<Self> {
        let mut pico_args = Arguments::from_env();

        if pico_args.contains(["-h", "--help"]) {
            print!("{}", HELP);
            std::process::exit(0);
        }

        let args = Self {
            font_path: pico_args
                .opt_value_from_str(["-f", "--font-path"])?
                .unwrap_or_else(|| "font.ttf".to_owned()),
            font_size_px: pico_args
                .opt_value_from_str(["-s", "--font-size-px"])?
                .unwrap_or(16),
            font_subpixel_antialiasing: !pico_args.contains(["-a", "--disable-subpixel-aa"]),
            framebuffer_device_path: pico_args
                .opt_value_from_str(["-d", "--fb-device-path"])?
                .unwrap_or_else(|| "/dev/fb0".to_owned()),
            shell_path: pico_args
                .opt_free_from_str()?
                .unwrap_or_else(|| "/usr/bin/bash".to_owned()),
        };

        let remaining = pico_args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unused arguments left: {:?}.", remaining);
        }

        Ok(args)
    }
}
