# Roadmap

## Current state

The project is a Rust CLI (binary `3bld-coach`, package `bld_coach`) for 3x3
blindfolded (3BLD) solve training, built on the [`cubesim`](https://docs.rs/cubesim)
crate. The original C prototype and its Nix flake have been fully removed and
replaced with a `devenv`-managed Rust setup.

### `src/` layout

- `main.rs` — wires up the module tree and calls `cli::run()`.
- `cli.rs` — clap-based CLI with three subcommands: `scramble`, `solve
  <corners|edges>`, `memo`.
- `cube.rs` — wraps `cubesim::FaceletCube` as `CubeState`; generates random
  WCA-notation scrambles and formats `Move` sequences back to notation.
- `lettering.rs` — Speffz-style A–X letter scheme and buffer definitions.
  `speffz_letter` is implemented and unit-tested; not yet wired into
  `memo.rs`.
- `memo.rs` — cycle-breaking over corner/edge permutations relative to a
  buffer. **Stub.**
- `trainer.rs` — interactive memorize → execute → review practice loop.
  **Stub.**

### What works

- `cargo run -- scramble --length 20` generates a valid WCA scramble (18
  basic face turns, no immediate face repeats). This is the only fully
  implemented feature.
- `cargo run -- --help` shows a working CLI with all three subcommands
  wired up (though two of them panic with `todo!()` when invoked).

### What's missing

- `solve corners` / `solve edges` — not implemented; blocked on `memo.rs`.
- `memo.rs` — cycle-breaking logic; the key missing piece, now that
  `lettering.rs` is implemented.
- `memo` subcommand / `trainer.rs` — the interactive practice session;
  blocked on both of the above.

## Next steps

1. Implement `memo::cycles`, walking the corner/edge permutation cycles from
   a buffer piece to produce the letter-pair memo sequence a solver would
   learn (Old Pochmann / M2 style to start).
2. Wire `cli::run`'s `solve` branch to print that memo sequence instead of
   calling `todo!()`.
3. Implement `trainer::run_session`: scramble, prompt for memorized letters,
   time execution, and report results.
4. Longer-term, revisit the buffer/lettering defaults (currently hardcoded
   to UBL corner / UB edge) as configurable CLI options, and consider
   3-style memo generation once the Old Pochmann/M2 path works end to end.
