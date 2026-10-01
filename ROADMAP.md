# Roadmap

## Current state

The project is a Rust CLI (binary `3bld-coach`, package `bld_coach`) for 3x3
blindfolded (3BLD) solve training, built on the [`cubesim`](https://docs.rs/cubesim)
crate. The original C prototype and its Nix flake have been fully removed and
replaced with a `devenv`-managed Rust setup.

### `src/` layout

- `main.rs` — wires up the module tree and calls `cli::run()`.
- `cli.rs` — clap-based CLI with three subcommands: `scramble`, `solve
  <corners|edges>`, `memo`. `solve` and `memo` both take `--corner-buffer`
  and `--edge-buffer` (Speffz letters, default `A`/`A` i.e. UBL/UB) via a
  shared `BufferArgs`, so solvers aren't stuck with the hardcoded defaults.
- `cube.rs` — wraps `cubesim::FaceletCube` as `CubeState`; generates random
  WCA-notation scrambles and formats `Move` sequences back to notation.
- `lettering.rs` — Speffz-style A–X letter scheme and buffer definitions.
  `speffz_letter` maps a sticker to its letter; `corner_sticker`/
  `edge_sticker` invert it; `corner_piece`/`edge_piece` group the 3 (or 2)
  letters that name the same physical piece. The piece-grouping tables were
  derived empirically against `cubesim`'s facelet layout (see `memo.rs`'s
  module doc) and cross-checked against the pre-existing UFL/UF unit tests.
- `memo.rs` — cycle-breaking over corner/edge permutations relative to a
  buffer. **Implemented.** `cycles` walks the buffer's permutation cycle via
  color-set matching (no reliance on `cubesim` internals beyond `state()`),
  then continues into any further unsolved pieces as additional
  Old-Pochmann-style "floating" cycles, the way a solver picks up a fresh
  letter pair once the buffer's own cycle closes.
- `trainer.rs` — interactive memorize → execute → review practice loop.
  **Implemented.** `session` (the testable core of `run_session`) prints the
  scramble, prompts for the corner and edge memo the solver came up with
  *before* any execution happens, times the gap between two Enter presses as
  execution time, then reports each piece type as correct or incorrect
  against `memo::cycles`'s answer. The report displays the expected memo
  chunked into `group_size`-letter groups (`chunk_letters`, e.g. "BR NE
  QI" for `group_size` 2), matching how letter-pair/triple methods are
  practiced; correctness checking always compares the raw, unchunked
  letters.

### What works

- `cargo run -- scramble --length 20` generates a valid WCA scramble (18
  basic face turns, no immediate face repeats).
- `cargo run -- solve corners` / `solve edges` scrambles a cube and prints
  the Old-Pochmann-style letter memo for that piece type (or "Already
  solved." if the buffer's piece and everything else already happens to be
  correct).
- `cargo run -- memo` runs a full memorize → execute → review session:
  scramble, memo prompts, an execution timer, and a correct/incorrect report
  per piece type, with the expected memo displayed in `--group-size`-letter
  groups (default 2).
- `--corner-buffer`/`--edge-buffer` on `solve` and `memo` let solvers pick
  their own buffer pieces (any Speffz letter `A`-`X`) instead of the UBL/UB
  defaults.
- `cargo run -- --help` shows a working CLI with all three subcommands
  fully wired up.

### What's missing

- Nothing blocking; see "Next steps" for what's left.

## Next steps

1. Consider 3-style memo generation once the Old Pochmann/M2 path works
   end to end. This is a separate algorithm (commutator-based 3-cycle
   resolution) with no existing scaffolding, not an extension of
   `memo::cycles`.
