//! Interactive practice sessions: scramble, memorize, execute, review. Ties
//! together `cube`, `lettering`, and `memo` into the "memo trainer" and
//! "execution practice" modes sketched out in the README.
//!
//! A session asks the solver to write down the letter memo they came up
//! with for each piece type *before* they execute anything (so they can't
//! peek at the correct answer mid-solve), times the gap between two Enter
//! presses as their execution time, then reports whether what they wrote
//! down matches the memo `memo::cycles` derives from the scramble.

use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

use crate::cube::{self, CubeState};
use crate::lettering::Buffers;
use crate::memo::{self, CycleStep, Piece};

/// Runs one interactive memo + execution practice session against
/// stdin/stdout.
pub fn run_session() {
    let scramble = cube::random_scramble(25);
    let cube = cube::scrambled(&scramble);
    let stdin = io::stdin();
    let stdout = io::stdout();

    session(
        &cube,
        &cube::to_wca_string(&scramble),
        &Buffers::default(),
        stdin.lock(),
        stdout.lock(),
    );
}

/// The testable core of [`run_session`]: takes the scramble already applied
/// (so tests can set up specific cube states) and generic input/output, so
/// it never has to touch real stdin/stdout.
fn session(cube: &CubeState, scramble: &str, buffers: &Buffers, mut input: impl BufRead, mut output: impl Write) {
    let corner_memo = letters_of(&memo::cycles(cube, Piece::Corners, buffers));
    let edge_memo = letters_of(&memo::cycles(cube, Piece::Edges, buffers));

    writeln!(output, "Scramble: {scramble}").unwrap();

    let corner_answer = prompt(&mut input, &mut output, "Corner memo (blank if already solved): ");
    let edge_answer = prompt(&mut input, &mut output, "Edge memo (blank if already solved): ");

    writeln!(output, "Press Enter to start executing your solve...").unwrap();
    wait_for_enter(&mut input);
    let start = Instant::now();
    writeln!(output, "Press Enter once you've finished executing.").unwrap();
    wait_for_enter(&mut input);
    let elapsed = start.elapsed();

    writeln!(output, "{}", report(&corner_memo, &corner_answer, &edge_memo, &edge_answer, elapsed)).unwrap();
}

/// The letters of a memo sequence, in order, as a single string (e.g. the
/// pair `"ER"`), the way a solver would write it down.
fn letters_of(steps: &[CycleStep]) -> String {
    steps.iter().map(|s| s.letter).collect()
}

fn prompt(input: &mut impl BufRead, output: &mut impl Write, label: &str) -> String {
    write!(output, "{label}").unwrap();
    output.flush().unwrap();
    read_line(input)
}

fn wait_for_enter(input: &mut impl BufRead) {
    read_line(input);
}

fn read_line(input: &mut impl BufRead) -> String {
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    line.trim().to_ascii_uppercase()
}

/// Builds the end-of-session report: execution time plus a correct/
/// incorrect line for each piece type.
fn report(corner_memo: &str, corner_answer: &str, edge_memo: &str, edge_answer: &str, elapsed: Duration) -> String {
    [
        format!("Execution time: {:.2}s", elapsed.as_secs_f64()),
        piece_report("Corners", corner_memo, corner_answer),
        piece_report("Edges", edge_memo, edge_answer),
    ]
    .join("\n")
}

fn piece_report(name: &str, expected: &str, given: &str) -> String {
    let display = |memo: &str| if memo.is_empty() { "already solved".to_string() } else { memo.to_string() };

    if given == expected {
        format!("{name}: correct ({})", display(expected))
    } else {
        format!("{name}: incorrect -- you wrote \"{given}\", correct memo was \"{}\"", display(expected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubesim::{sticker_index, Face, FaceletCube};
    use std::io::Cursor;

    /// Runs `session` with scripted stdin lines and returns everything it
    /// printed, for assertions against the transcript.
    fn run(cube: &CubeState, input_lines: &[&str]) -> String {
        let input = Cursor::new(input_lines.join("\n").into_bytes());
        let mut output = Vec::new();
        session(cube, "R U F", &Buffers::default(), input, &mut output);
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn solved_cube_expects_blank_memo_for_both_piece_types() {
        let transcript = run(&cube::solved(), &["", "", "", ""]);
        assert!(transcript.contains("Corners: correct (already solved)"));
        assert!(transcript.contains("Edges: correct (already solved)"));
    }

    #[test]
    fn matching_memo_is_reported_correct() {
        // Same buffer-corner twist as memo.rs's unit test: expects "ER".
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::U, 1) as usize] = Face::L;
        state[sticker_index(3, Face::L, 1) as usize] = Face::B;
        state[sticker_index(3, Face::B, 3) as usize] = Face::U;
        let cube = FaceletCube::from(state);

        let transcript = run(&cube, &["ER", "", "", ""]);
        assert!(transcript.contains("Corners: correct (ER)"));
        assert!(transcript.contains("Edges: correct (already solved)"));
    }

    #[test]
    fn mismatched_memo_is_reported_incorrect_with_the_right_answer() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::U, 1) as usize] = Face::L;
        state[sticker_index(3, Face::L, 1) as usize] = Face::B;
        state[sticker_index(3, Face::B, 3) as usize] = Face::U;
        let cube = FaceletCube::from(state);

        let transcript = run(&cube, &["XY", "", "", ""]);
        assert!(transcript.contains("Corners: incorrect -- you wrote \"XY\", correct memo was \"ER\""));
    }

    #[test]
    fn memo_answers_are_case_insensitive() {
        let mut state = cubesim::solved_state(3);
        state[sticker_index(3, Face::U, 1) as usize] = Face::L;
        state[sticker_index(3, Face::L, 1) as usize] = Face::B;
        state[sticker_index(3, Face::B, 3) as usize] = Face::U;
        let cube = FaceletCube::from(state);

        let transcript = run(&cube, &["er", "", "", ""]);
        assert!(transcript.contains("Corners: correct (ER)"));
    }

    #[test]
    fn transcript_prompts_for_both_piece_memos_and_execution_timing() {
        let transcript = run(&cube::solved(), &["", "", "", ""]);
        assert!(transcript.contains("Scramble: R U F"));
        assert!(transcript.contains("Corner memo"));
        assert!(transcript.contains("Edge memo"));
        assert!(transcript.contains("Press Enter to start executing"));
        assert!(transcript.contains("Execution time:"));
    }
}
