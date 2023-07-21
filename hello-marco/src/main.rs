use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Bach Nguyen", about = "A Marco Polo game")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Bach Nguyen")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            if name == "Marco" {
                let result = hello_marco::marco_polo(&name);
                println!("{}", result);
            } else {
                println!("What's your name?")
            }
        }
        None => println!("No command given"),
    }
}
