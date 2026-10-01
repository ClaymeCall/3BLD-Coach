//! Command-line interface: argument parsing and dispatch into the rest of
//! the crate. Mirrors the `--scramble` / `--solve corners` / `--memo
//! trainer` usage sketched out in the README.

use clap::{Parser, Subcommand, ValueEnum};

use crate::{cube, memo, trainer};

#[derive(Parser)]
#[command(name = "3bld-coach", version, about = "A trainer for 3x3 blindfolded solving")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a random WCA-style scramble.
    Scramble {
        /// Number of face turns in the scramble.
        #[arg(short, long, default_value_t = 25)]
        length: usize,
    },
    /// Generate a buffer-based memo solution for one piece type.
    Solve {
        #[arg(value_enum)]
        piece: Piece,
    },
    /// Run an interactive memo and execution practice session.
    Memo,
}

#[derive(Clone, Copy, ValueEnum)]
enum Piece {
    Corners,
    Edges,
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Command::Scramble { length } => {
            let scramble = cube::random_scramble(length);
            println!("{}", cube::to_wca_string(&scramble));
        }
        Command::Solve { piece } => {
            let piece = match piece {
                Piece::Corners => memo::Piece::Corners,
                Piece::Edges => memo::Piece::Edges,
            };
            let scramble = cube::random_scramble(25);
            let cube = cube::scrambled(&scramble);
            let steps = memo::cycles(&cube, piece, &Default::default());

            println!("Scramble: {}", cube::to_wca_string(&scramble));
            if steps.is_empty() {
                println!("Already solved.");
            } else {
                let letters: String = steps.iter().map(|s| s.letter).collect();
                println!("Memo: {letters}");
            }
        }
        Command::Memo => trainer::run_session(),
    }
}
