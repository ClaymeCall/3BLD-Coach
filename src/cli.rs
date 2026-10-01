//! Command-line interface: argument parsing and dispatch into the rest of
//! the crate. Mirrors the `--scramble` / `--solve corners` / `--memo
//! trainer` usage sketched out in the README.

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{cube, lettering, memo, trainer};

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
        #[command(flatten)]
        buffers: BufferArgs,
    },
    /// Run an interactive memo and execution practice session.
    Memo {
        #[command(flatten)]
        buffers: BufferArgs,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Piece {
    Corners,
    Edges,
}

/// Buffer piece letters, shared by `solve` and `memo`, so solvers aren't
/// stuck with the UBL corner / UB edge defaults.
#[derive(Args)]
struct BufferArgs {
    /// Corner buffer letter (Speffz 'A'..='X'; default 'A', UBL).
    #[arg(long, default_value = "A", value_parser = parse_speffz_letter)]
    corner_buffer: char,
    /// Edge buffer letter (Speffz 'A'..='X'; default 'A', UB).
    #[arg(long, default_value = "A", value_parser = parse_speffz_letter)]
    edge_buffer: char,
}

impl BufferArgs {
    fn buffers(&self) -> lettering::Buffers {
        lettering::Buffers::new(self.corner_buffer, self.edge_buffer)
            .expect("already validated by parse_speffz_letter")
    }
}

/// Parses a single Speffz letter from a CLI argument, case-insensitively.
fn parse_speffz_letter(s: &str) -> Result<char, String> {
    match s.to_ascii_uppercase().chars().collect::<Vec<_>>()[..] {
        [c] if ('A'..='X').contains(&c) => Ok(c),
        _ => Err(format!("`{s}` is not a Speffz letter ('a'..='x')")),
    }
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Command::Scramble { length } => {
            let scramble = cube::random_scramble(length);
            println!("{}", cube::to_wca_string(&scramble));
        }
        Command::Solve { piece, buffers } => {
            let piece = match piece {
                Piece::Corners => memo::Piece::Corners,
                Piece::Edges => memo::Piece::Edges,
            };
            let scramble = cube::random_scramble(25);
            let cube = cube::scrambled(&scramble);
            let steps = memo::cycles(&cube, piece, &buffers.buffers());

            println!("Scramble: {}", cube::to_wca_string(&scramble));
            if steps.is_empty() {
                println!("Already solved.");
            } else {
                let letters: String = steps.iter().map(|s| s.letter).collect();
                println!("Memo: {letters}");
            }
        }
        Command::Memo { buffers } => trainer::run_session(&buffers.buffers()),
    }
}
