use clap::Parser;

fn main() {
    let cli = Cli::parse();

    println!("two: {}", cli.two);
    println!("two: {}", cli.one);
}

#[derive(Parser)]
#[command(name = "RedisLight")]
#[command(author = "Gearhartlove")]
#[command(version = "1.0")]
#[command(about = "Mimics basic Redis commands in the console.", long_about = None)]
pub struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}
