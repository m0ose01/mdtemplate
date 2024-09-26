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
}


fn main() {
    let args = Args::parse();
    println!("{}", args.input);
    println!("{}", args.output);
}
