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
  shared `BufferArgs`, so solvers aren't stuck with the hardcoded defaults,
  and `--method old-pochmann|three-style` to pick the memo method (default
  `old-pochmann`).
- `cube.rs` — wraps `cubesim::FaceletCube` as `CubeState`; generates random
  WCA-notation scrambles and formats `Move` sequences back to notation.
- `lettering.rs` — Speffz-style A–X letter scheme and buffer definitions.
  `speffz_letter` maps a sticker to its letter; `corner_sticker`/
  `edge_sticker` invert it; `corner_piece`/`edge_piece` group the 3 (or 2)
  letters that name the same physical piece. The piece-grouping tables were
  derived empirically against `cubesim`'s facelet layout (see `memo.rs`'s
  module doc) and cross-checked against the pre-existing UFL/UF unit tests.
- `cycle.rs` — shared permutation-query and cycle-walking primitives used by
  both `memo` and `threestyle`. `PieceCycles::next_letter` answers "which
  solved-cube sticker currently occupies this slot?" via color-set
  matching; `walk_cycles` walks a starting letter's cycle and then
  continues into any further unvisited, non-fixed letters as additional
  "floating" cycles (the buffer-routed traversal Old Pochmann/M2 need);
  `walk_one_cycle` walks a single cycle from one starting letter with no
  further continuation (what `threestyle` needs, since it drives its own
  multi-track discovery -- see below).
- `memo.rs` — the Old Pochmann/M2 cycle-breaking generator, now a thin
  wrapper over `cycle::walk_cycles`. `cycles` walks the buffer's
  permutation cycle via `cycle`'s primitives, then continues into any
  further unsolved pieces as additional floating cycles, the way a solver
  picks up a fresh letter pair once the buffer's own cycle closes.
- `threestyle.rs` — **Implemented.** Freestyle, commutator-based 3-style
  memo generation, independent of (not an extension of) `memo::cycles`.
  `generate` classifies every disjoint cycle of a scramble's corner/edge
  permutation into typed `Target`s: `CornerSwap`/`EdgeSwap` (ordinary
  commutator targets), `CornerTwist`/`EdgeFlip` (a single piece
  twisted/flipped in place, needing a different algorithm family -- not
  just the buffer's own piece, unlike `memo::cycles`, which only happens to
  handle that case correctly when it's the buffer itself), and
  `CornerParity`/`EdgeParity` (a lone leftover letter after pairing up an
  odd cycle two at a time, needing a parity-style algorithm; a piece type
  can have zero, one, or several of these independently -- they don't need
  to come in matched corner/edge pairs to each be individually valid).
  A single physical corner swap couples all 3 of its stickers, but
  `next_letter`'s color-matching only ever follows one sticker's chain per
  step, so a full corner-to-corner swap generally spans up to 3 independent
  "parallel tracks" (one per sticker axis) that `generate` discovers and
  resolves individually -- Old Pochmann never needs this (its fixed buffer
  algorithm physically carries the other 2 stickers of each involved corner
  along for free), but 3-style's freestyle simulation does, to be able to
  validate an answer at all. `validate`/`validate_piece` check correctness
  by simulating a submitted answer (any semantically valid decomposition,
  not just the one `generate` produces) against an abstract per-letter
  permutation model, seeded from the scramble itself -- this is the
  mechanism that gives freestyle's "more than one valid answer" flexibility
  real meaning. `parse_tokens` parses a typed answer back into targets, and
  `display` renders targets for display with a label on anything that
  isn't an ordinary swap.

  **Known modeling caveat:** a real commutator always physically involves
  the buffer as one of its three slots; a freestyle `Swap` target naming
  two non-buffer letters is an abstract permutation step validated against
  `threestyle`'s own simulation model, not a guarantee that it corresponds
  to one single minimal real-world algorithm. This trade-off is accepted
  for v1 (see `threestyle.rs`'s module doc and commit history for the
  reasoning) in exchange for genuine freestyle flexibility and a
  correctness check that doesn't just replay `generate`'s own walk order.
- `trainer.rs` — interactive memorize → execute → review practice loop.
  **Implemented** for both methods. `run_session` dispatches on `Method`
  (`OldPochmann` or `ThreeStyle`) to `session_old_pochmann` or
  `session_three_style`. Both print the scramble, prompt for the corner and
  edge memo the solver came up with *before* any execution happens, time
  the gap between two Enter presses as execution time, then report each
  piece type as correct or incorrect. Old Pochmann's expected memo is
  chunked into `group_size`-letter groups (`chunk_letters`, e.g. "BR NE
  QI" for `group_size` 2) and checked by exact string match. 3-style's
  expected memo is rendered one target per token (`threestyle::display`,
  labeling twists/flips/parity targets) and checked by
  `threestyle::validate_piece`, so a correct-but-different decomposition
  from the one displayed is accepted too.

### What works

- `cargo run -- scramble --length 20` generates a valid WCA scramble (18
  basic face turns, no immediate face repeats).
- `cargo run -- solve corners` / `solve edges` scrambles a cube and prints
  the Old-Pochmann-style letter memo for that piece type (or "Already
  solved." if the buffer's piece and everything else already happens to be
  correct). `--method three-style` prints the 3-style target list instead,
  with a note when the other piece type also needs a parity fix.
- `cargo run -- memo` runs a full memorize → execute → review session:
  scramble, memo prompts, an execution timer, and a correct/incorrect
  report per piece type. `--method three-style` runs the same flow using
  freestyle 3-style targets and semantic (not string) correctness checking.
- `--corner-buffer`/`--edge-buffer` on `solve` and `memo` let solvers pick
  their own buffer pieces (any Speffz letter `A`-`X`) instead of the UBL/UB
  defaults, for both methods.
- `cargo run -- --help` shows a working CLI with all three subcommands
  fully wired up, including `--method` on `solve` and `memo`.

### What's missing

- Nothing blocking; see "Next steps" for possible follow-ups.

## Next steps

Nothing required for the current feature set. Possible future refinements
to `threestyle`, if real-world practice surfaces a need:

1. Tighten the "known modeling caveat" above: validate (or constrain)
   freestyle `Swap`/`Parity` targets against an actual commutator-algorithm
   database, rather than an abstract permutation model, so every accepted
   answer is guaranteed to correspond to a real, executable algorithm.
2. Surface which specific algorithm family a target needs (e.g. which of
   the 4 standard insertion variants, or the interchange, per BLDDB's
   terminology) rather than just labeling `(twist)`/`(flip)`/`(parity)`.
