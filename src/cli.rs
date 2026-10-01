//! Command-line interface: argument parsing and dispatch into the rest of
//! the crate. Mirrors the `--scramble` / `--solve corners` / `--memo
//! trainer` usage sketched out in the README.

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::threestyle::Target;
use crate::{cube, lettering, memo, threestyle, trainer};

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
        /// Memo method: Old Pochmann/M2's buffer-routed cycles, or
        /// freestyle 3-style.
        #[arg(long, value_enum, default_value_t = Method::OldPochmann)]
        method: Method,
    },
    /// Run an interactive memo and execution practice session.
    Memo {
        #[command(flatten)]
        buffers: BufferArgs,
        /// Letters per group when displaying an Old Pochmann memo (2 for
        /// letter pairs, 3 for M2 triples); unused for `--method
        /// three-style`, whose targets each display as their own token.
        #[arg(long, default_value_t = 2)]
        group_size: usize,
        /// Memo method: Old Pochmann/M2's buffer-routed cycles, or
        /// freestyle 3-style.
        #[arg(long, value_enum, default_value_t = Method::OldPochmann)]
        method: Method,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Piece {
    Corners,
    Edges,
    /// Both piece types, chained on the same scramble.
    All,
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum Method {
    #[default]
    OldPochmann,
    ThreeStyle,
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

/// Labels each piece type's output line when `solve all` chains both, so
/// the two memos aren't ambiguous; solving a single piece type keeps the
/// original unlabeled output.
fn solve_prefix(piece: memo::Piece, piece_count: usize) -> &'static str {
    if piece_count == 1 {
        return "";
    }
    match piece {
        memo::Piece::Corners => "Corners -- ",
        memo::Piece::Edges => "Edges -- ",
    }
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Command::Scramble { length } => {
            let scramble = cube::random_scramble(length);
            println!("{}", cube::to_wca_string(&scramble));
        }
        Command::Solve { piece, buffers, method } => {
            let pieces: &[memo::Piece] = match piece {
                Piece::Corners => &[memo::Piece::Corners],
                Piece::Edges => &[memo::Piece::Edges],
                Piece::All => &[memo::Piece::Corners, memo::Piece::Edges],
            };
            let scramble = cube::random_scramble(25);
            let cube = cube::scrambled(&scramble);
            println!("Scramble: {}", cube::to_wca_string(&scramble));

            match method {
                Method::OldPochmann => {
                    for &piece in pieces {
                        let steps = memo::cycles(&cube, piece, &buffers.buffers());
                        let prefix = solve_prefix(piece, pieces.len());
                        if steps.is_empty() {
                            println!("{prefix}Already solved.");
                        } else {
                            let letters: String = steps.iter().map(|s| s.letter).collect();
                            println!("{prefix}Memo: {letters}");
                        }
                    }
                }
                Method::ThreeStyle => {
                    let memo = threestyle::generate(&cube, &buffers.buffers());
                    for &piece in pieces {
                        let (targets, other_name, other_has_parity) = match piece {
                            memo::Piece::Corners => {
                                (&memo.corners, "edge", memo.edges.iter().any(|t| matches!(t, Target::EdgeParity { .. })))
                            }
                            memo::Piece::Edges => (
                                &memo.edges,
                                "corner",
                                memo.corners.iter().any(|t| matches!(t, Target::CornerParity { .. })),
                            ),
                        };
                        let prefix = solve_prefix(piece, pieces.len());

                        if targets.is_empty() {
                            println!("{prefix}Already solved.");
                        } else {
                            println!("{prefix}Memo: {}", threestyle::display(targets));
                        }
                        // When solving both piece types together, a lone
                        // parity target is paired with the other type's own
                        // output just below, so the cross-reference warning
                        // would be redundant.
                        if other_has_parity && pieces.len() == 1 {
                            let article = if other_name == "edge" { "an" } else { "a" };
                            println!(
                                "Parity required: pair this with {article} {other_name} parity target (see the other piece type)."
                            );
                        }
                    }
                }
            }
        }
        Command::Memo { buffers, group_size, method } => {
            let method = match method {
                Method::OldPochmann => trainer::Method::OldPochmann,
                Method::ThreeStyle => trainer::Method::ThreeStyle,
            };
            trainer::run_session(&buffers.buffers(), method, group_size)
        }
    }
}
