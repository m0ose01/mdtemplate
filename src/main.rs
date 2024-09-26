use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    #[arg(short = 'o', long = "output", default_value_t = ("template.md").to_string())]
    output: String,
}


fn main() {
    let args = Args::parse();
    println!("{}", args.input);
    println!("{}", args.output);
}
