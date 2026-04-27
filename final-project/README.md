# Quoridor AI

A terminal Quoridor game with pluggable AI bots, built in Rust.

## Running

```
cargo run [--release] -- [white] [black]
```

`--release` enables compiler optimizations and is strongly recommended for AI vs AI matches.

## Player Specs

| Spec          | Description                          |
| ------------- | ------------------------------------ |
| `human`       | Interactive keyboard input           |
| `random`      | Plays a random legal move            |
| `mm<n>[eval]` | Minimax to depth `n`                 |
| `ab<n>[eval]` | Alpha-beta pruning to depth `n`      |
| `p<n>[eval]`  | Parallelized alpha-beta to depth `n` |

**Depth** (`n`) is a positive integer. Higher depth = stronger play, exponentially slower.
Odd depths are preferred so the search ends on the bot's own move.

## Evaluators

Append the evaluator name directly after the depth. Defaults to `simple` if omitted.

| Suffix   | Evaluator         | Description                                            |
| -------- | ----------------- | ------------------------------------------------------ |
| `base`   | `BaseEvaluator`   | Pure path-length difference: `black_dist − white_dist` |
| `simple` | `SimpleEvaluator` | Path diff × 10 + wall advantage + tempo bonus          |
| `urgent` | `UrgentEvaluator` | Like simple, but wall weight scales with game urgency  |

## Examples

```bash
# Human (white) vs alpha-beta depth 5, simple evaluator (default)
cargo run -- human ab5

# Human vs alpha-beta depth 7, urgent evaluator, optimized build
cargo run --release -- human ab7urgent

# Alpha-beta depth 5 (simple) vs alpha-beta depth 7 (urgent)
cargo run --release -- ab5 ab7urgent

# Minimax depth 3 (base) vs parallelized alpha-beta depth 5 (urgent)
cargo run --release -- mm3base p5urgent

# Bot vs bot, both using default evaluator
cargo run --release -- ab5 ab7
```

## Controls (Human Player)

Type commands for pawn moves (`e2`) or for wall moves (`e3h`, `d4v`). Press `Esc` to quit.

## Benchmarking

```bash
cargo test --release bench::compare -- --nocapture
```
