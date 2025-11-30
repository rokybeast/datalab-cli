use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::{thread, time::Duration};

// cli struct
#[derive(Parser)]
#[command(name = "datalab")]
#[command(version = "1.0")]
#[command(about = "datalab-cli tester - A modern CLI tool for datalab operations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update {
        #[arg(long, default_value = "1.0")]
        speed: f32,
    },

    Error {
        #[arg(long, default_value = "src/main.dtcm")]
        file: String,

        #[arg(long, default_value = "Check your datachemist syntax")]
        suggestion: String,

        #[arg(long)]
        pos: Option<u32>,

        #[arg(long, default_value = "datalab-cli")]
        module: String,

        #[arg(long, default_value = "dl.core.001")]
        code: String,
    },
}

// errors
fn dl_error(module: &str, file: &str, suggestion: &str, pos: u32, code: &str) {
    println!("{} From: '{}'", "[ERROR]:".red().bold(), module.yellow());
    println!(
        "\t{} [{}]",
        "An unexpected error occurred.".white(),
        code.bright_black()
    );
    println!(
        "\t{} {}:{}",
        "->".cyan(),
        file.bright_white(),
        pos.to_string().bright_white()
    );
    println!("\t{}", "`<no snippet available>`".bright_black().italic());
    println!(
        "\t{} {}",
        "-> Suggestion:".green().bold(),
        suggestion.white()
    );
}

// progress bar
fn update_bar(speed: f32) {
    let pb = ProgressBar::new(100);

    // emdash '— ' style
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("— ")
    );

    pb.set_message("==> Updating datalab");

    let delay = Duration::from_millis((50.0 / speed) as u64);

    for i in 0..=100 {
        pb.set_position(i);
        thread::sleep(delay);
    }

    pb.finish_with_message("==> Update completed");

    let total_time = (5.0 / speed).round() as u32;
    println!(
        "\n{} Completed in {}s",
        "[INFO]:".green().bold(),
        total_time
    );
}

// main()
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Update { speed } => {
            if speed <= 0.0 {
                eprintln!("{} Speed must be greater than 0", "[ERROR]:".red().bold());
                std::process::exit(1);
            }
            update_bar(speed);
        }

        Commands::Error {
            file,
            suggestion,
            pos,
            module,
            code,
        } => {
            // @generate rand values if not given (args)
            let pos = pos.unwrap_or_else(|| rand::thread_rng().gen_range(1..200));

            dl_error(&module, &file, &suggestion, pos, &code);
        }
    }
}
