use clap::Parser;

#[derive(Parser)]
struct Cli {
    target: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Monitorin laptoz {}", cli.target);
}
