# 🧠 3BLD-Coach

A command-line training tool for **3x3 Blindfolded Solving (3BLD)**. Simulates
cube states and scrambles, and generates buffer-based (Old Pochmann/M2) or
freestyle 3-style memo for the corners and edges of a scramble, plus an
interactive memorize → execute → review practice session.

Built in Rust (binary `3bld-coach`, package `bld_coach`) on top of the
[`cubesim`](https://docs.rs/cubesim) crate.

---

## 🚀 Features

- 🧩 Accurate 3x3 cube simulation and WCA-style scramble generation.
- 🎯 Corner & edge memo generation for **Old Pochmann/M2** (buffer-routed
  cycle-breaking) and **freestyle 3-style** (commutator-based, with typed
  swap/twist/flip/parity targets).
- 📈 An interactive `memo` command: scramble → write down your memo →
  execution timer → correct/incorrect report per piece type. 3-style
  answers are checked by simulating whether they actually solve the
  scramble, so more than one correct decomposition is accepted, not just
  one canonical answer.
- ⚙️ Customizable corner/edge buffer pieces (any Speffz letter `A`-`X`) on
  every command.

---

## 📦 Installation

Requires a Rust toolchain (`cargo`). From the repo root:

```bash
cargo build --release
```

or run directly with `cargo run --`.

## 🕹️ Usage

```bash
# Generate a random WCA-style scramble
cargo run -- scramble --length 20

# Print the memo for one piece type (Old Pochmann/M2 by default)
cargo run -- solve corners
cargo run -- solve edges

# Chain both piece types on the same scramble
cargo run -- solve all

# Same, but with freestyle 3-style targets instead
cargo run -- solve corners --method three-style
cargo run -- solve all --method three-style

# Pick custom buffer pieces (default is UBL corner / UR edge, i.e. 'A'/'B')
cargo run -- solve corners --corner-buffer E

# Run an interactive memorize -> execute -> review session
cargo run -- memo
cargo run -- memo --method three-style
cargo run -- memo --group-size 3   # M2-style triples instead of pairs

cargo run -- --help                # full flag reference
```

---

## 🧠 How It Works

- `cube.rs` wraps `cubesim::FaceletCube` for scrambling and WCA-notation
  formatting.
- `lettering.rs` implements the Speffz A–X sticker-lettering scheme and the
  corner/edge buffer definitions.
- `cycle.rs` holds the shared permutation-query and cycle-walking
  primitives both memo methods are built on: `PieceCycles::next_letter`
  answers "which solved-cube sticker currently occupies this slot?" via
  color-set matching; `walk_cycles` walks a cycle from a starting letter
  and continues into further "floating" cycles (what Old Pochmann/M2
  need); `walk_one_cycle` walks a single cycle with no further
  continuation (what 3-style needs, since it drives its own multi-track
  discovery -- see below).
- `memo.rs` implements Old Pochmann/M2: `cycles` walks the buffer's
  permutation cycle, then continues into any remaining unsolved pieces as
  further floating cycles, the way a solver picks up a fresh letter pair
  once the buffer's own cycle closes.
- `threestyle.rs` implements freestyle, commutator-based 3-style, not as an
  extension of `memo::cycles` but as an independent generator. `generate`
  classifies every disjoint permutation cycle into typed targets:
  `CornerSwap`/`EdgeSwap` (ordinary commutator targets), `CornerTwist`/
  `EdgeFlip` (a single piece twisted/flipped in place -- not just the
  buffer's own piece, unlike `memo::cycles`, which only happens to handle
  that case when it's the buffer itself), and `CornerParity`/`EdgeParity`
  (a lone leftover letter after pairing up an odd cycle two at a time; a
  piece type can have zero, one, or several of these independently -- they
  don't need to come in matched corner/edge pairs to each be individually
  valid). A single physical corner swap couples all 3 of its stickers, but
  `next_letter`'s color-matching only follows one sticker's chain per step,
  so a full corner-to-corner swap generally spans up to 3 independent
  "parallel tracks" (one per sticker axis) that `generate` discovers and
  resolves individually -- Old Pochmann never needs this (its fixed buffer
  algorithm physically carries the other 2 stickers of each involved corner
  along for free), but 3-style's freestyle simulation does, to be able to
  validate an answer at all. `validate`/`validate_piece` check correctness
  by simulating a submitted answer (any semantically valid decomposition,
  not just the one `generate` produces) against an abstract per-letter
  permutation model seeded from the scramble -- this is what gives
  freestyle's "more than one valid answer" flexibility real meaning.
  `parse_tokens` parses a typed answer back into targets, and `display`
  renders targets with a label on anything that isn't an ordinary swap.

  **Known modeling caveat:** a real commutator always physically involves
  the buffer as one of its three slots; a freestyle `Swap` target naming
  two non-buffer letters is an abstract permutation step validated against
  `threestyle`'s own simulation model, not a guarantee that it corresponds
  to one single minimal real-world algorithm. This trade-off is accepted in
  exchange for genuine freestyle flexibility and a correctness check that
  doesn't just replay `generate`'s own walk order.
- `trainer.rs` ties it together into the interactive `memo` session.
  `run_session` dispatches on the chosen method to `session_old_pochmann`
  or `session_three_style`. Both print the scramble, prompt for the corner
  and edge memo *before* any execution happens, time the gap between two
  Enter presses as execution time, then report each piece type as correct
  or incorrect. Old Pochmann's expected memo is chunked into `group_size`
  letter groups and checked by exact string match; 3-style's is rendered
  one target per token and checked semantically via `validate_piece`, so a
  correct-but-different decomposition from the one displayed is accepted
  too.

---

## 🔭 Possible future work

Nothing is required for the current feature set. Possible refinements to
`threestyle`, if real-world practice surfaces a need:

1. Tighten the "known modeling caveat" above: validate (or constrain)
   freestyle `Swap`/`Parity` targets against an actual commutator-algorithm
   database, rather than an abstract permutation model, so every accepted
   answer is guaranteed to correspond to a real, executable algorithm.
2. Surface which specific algorithm family a target needs (e.g. which of
   the 4 standard insertion variants, or the interchange, per BLDDB's
   terminology) rather than just labeling `(twist)`/`(flip)`/`(parity)`.
3. Visual representation of the cube (CLI or TUI).
4. JSON import/export of memo sessions.
