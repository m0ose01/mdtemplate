use std::fs;
use std::io::Write;
use std::process::Command;

use clap::Parser;
use lopdf::Document;

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
            .arg(&args.input)
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

    if args.create_markdown {
        let document = Document::load(&args.input).expect("Could not open PDF file {args.input}.");
        let page_count = document.get_pages().len();

        let mut md_file = fs::File::create(&args.output).expect("Could not create markdown file.");
        let mut md_file_contents = "".to_string();

        for page in 1..page_count {
            md_file_contents.push_str(
                &slide_template(page)
            );
        }
        let _ = md_file.write(md_file_contents.as_bytes()).expect("Error writing to file.");
    }
}

fn slide_template(slide: usize) -> String {
    format!("\
# Slide {slide} {{.slide}}

![Slide {slide}](slides/img_{slide}.jpg)

* 

")
}
