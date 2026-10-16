mod ssh;

use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    target: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("Monitoring da {}", cli.target);

    let (_connection, stdout) = ssh::SshConnection::connect(&cli.target)?;

    let reader = BufReader::new(stdout);

    for line in reader.lines().take(5) {
        println!("{}", line?);
    }
    //we gon keep tis runnin for 5 just for keeping testing simple n small
    Ok(())
}
