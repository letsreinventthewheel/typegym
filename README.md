# TypeGym

TypeGym is a terminal typing trainer built in Rust with Ratatui and Crossterm. It generates short practice sessions, tracks typing speed and accuracy, and keeps the interface intentionally minimal so the focus stays on the text.

## Watch the YouTube Series

[TypeGym playlist](https://www.youtube.com/playlist?list=PLI7p1zrAYQeVm6Mgux1k58CUAM7AV5yW8)

## What It Does

- Renders a centered terminal typing interface with live cursor positioning.
- Highlights incorrect characters while leaving completed text clean.
- Tracks words per minute and accuracy for each completed session.
- Supports restarting sessions from inside the app.
- Generates practice text from static text, random word lists, weighted word lists, files, or a simple Markov chain.
- Optionally reflows generated text to a target line width.

## What You Can Learn

This project is useful if you want to see how a small terminal application is built from first principles in Rust:

- Structuring an event loop around terminal input and redraws.
- Rendering styled text with Ratatui.
- Handling raw keyboard input with Crossterm.
- Modeling typing state, cursor position, mistakes, backspace, and word deletion.
- Computing WPM and accuracy from session timing and keystroke history.
- Building configurable CLI behavior with Clap.
- Generating practice material with random sampling and Markov chains.

## Requirements

- Rust toolchain.
- A terminal that supports ANSI color output.

## Build

```bash
cargo build --release
```

The optimized executable will be available at:

```bash
target/release/typegym
```

## Run

```bash
cargo run
```

By default, TypeGym generates text using the Markov source in `data/markov.txt`.

Common examples:

```bash
cargo run -- --text-source static
cargo run -- --text-source nonsense --max-words 50
cargo run -- --text-source weighted --max-words 80
cargo run -- --text-source file:data/markov.txt
cargo run -- --text-source markov:data/markov.txt --max-words 120
cargo run -- --reflow --width 72
```

Use `--help` to see all options:

```bash
cargo run -- --help
```

## Controls

- Type the displayed text to complete a session.
- `Backspace` removes the previous character.
- `Ctrl+W` removes the previous word.
- `Esc` starts a new session.
- `Ctrl+C` quits during a session.
- After completing a session, `Enter` quits and `Esc` starts another session.

## Text Sources

TypeGym accepts the following values for `--text-source`:

- `static`: use the built-in example paragraph.
- `nonsense`: generate random words from `data/words.txt`.
- `weighted`: generate random words from `data/words_weighted.txt`, weighted by frequency.
- `file:<path>`: choose a short excerpt from a text file.
- `markov:<path>`: build a Markov chain from a text file and generate practice text from it.

## Further Ideas

- Add a persistent history file for previous session results.
- Display live WPM and accuracy while typing.
- Add configurable themes beyond raw ANSI color indexes.
- Add difficulty presets for different word counts and text sources.
