use clap::{Parser, Subcommand};
use crossterm::{
    ExecutableCommand,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::io::{Write, stdout};
use std::{thread, time::Duration};

// cli structure
#[derive(Parser)]
#[command(name = "datalab")]
#[command(version = "0.1")]
#[command(about = "datalab-cli tester")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// update
    Update,

    /// make error
    Error {
        #[arg(long)]
        file: Option<String>,

        #[arg(long)]
        suggestion: Option<String>,

        #[arg(long)]
        pos: Option<u32>,
    },
}

// formats
fn dl_error(module: &str, file: &str, suggestion: &str, pos: u32) {
    println!("[ERROR]: From: '{}'", module);
    println!("\tAn unexpected error occurred. [dl.core.001]");
    println!("\t-> At: {}:{}", file, pos);
    println!("\t`<no snippet available>`");
    println!("\t-> Suggestion: {}", suggestion);
}

// progress bar
fn update_bar() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    for i in 0..=100 {
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();

        print!("==> Updating datalab\n");
        print!(
            "--> Progress: [{}{}] {}%\n",
            "—".repeat((i / 5) as usize),
            " ".repeat(20 - (i / 5) as usize),
            i
        );
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50));
    }

    println!("\n[INFO]: Completed in 5s");
    terminal::disable_raw_mode().unwrap();
}

// main
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update => {
            update_bar();
        }

        Commands::Error {
            file,
            suggestion,
            pos,
        } => {
            // defaults
            let rand_pos = rand::thread_rng().gen_range(1..200);

            let file = file.unwrap_or_else(|| "src/main.dtcm".into());
            let suggestion = suggestion.unwrap_or_else(|| "Check your datachemist syntax".into());
            let pos = pos.unwrap_or(rand_pos);

            dl_error("datalab-cli", &file, &suggestion, pos);
        }
    }
}
