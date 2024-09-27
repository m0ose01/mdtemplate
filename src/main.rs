use std::fs;
use std::process::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(help = "Input file (must be a PDF).")]
    input: String,

    #[arg(short = 'o', long = "output", default_value_t = ("template.md").to_string(), help = "Output file.")]
    output: String,

    #[arg(long = "md", help = "Create markdown template.")]
    create_markdown: bool,

    #[arg(long = "img", help = "Create images from PDF.")]
    create_images: bool,

    #[arg(short = 'v', long = "verbose", help = "Verbose output.")]
    verbose: bool,
}


fn main() {
    let args = Args::parse();

    let _ = fs::create_dir("slides");

    if args.create_images {
        let magick = Command::new("magick")
            .arg(args.input)
            .args(["-quality", "100"])
            .arg("slides/img_%d.jpg")
            .output()
            .expect("Failed to launch imagemagick process.");

        match magick.status.code() {
            Some(code) => println!("Magick exited with code {code}."),
            None => println!("Magick process was terminated by a signal."),
        }

        if args.verbose {
            println!("magick stdout: {:?}", magick.stdout);
            println!("magick stderr: {:?}", magick.stderr);
        }
    }
}
