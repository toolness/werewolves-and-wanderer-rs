This is an implementation of _Werewolves and Wanderer_, the first
project in Tim Hartnell's [Creating Adventure Games on Your Computer][cagyc],
in Rust.

## Motivation

CAGYC is probably the oldest book I've continuously owned and taken
with me as I moved from place to place. I first bought it (or rather,
my parents bought it for me) at a bookstore at some point in the 1980s,
and I always found it inspirational to pore through its pages. But I
never actually _started_ any of the projects in the book, much less
completed them.

In 2017 I was learning [Rust][] and thought a text adventure would be
a nice way to learn the language, so I decided to actually start
the book's first project.

## Quick start

Install Rust, clone the project and run:

```
cargo run
```

If you want to try the web version, you can install [Emscripten][] and run:

```
bash build-emscripten.sh
python -m SimpleHTTPServer
```

Then visit http://localhost:8000/ in your browser.

[cagyc]: http://www.atariarchives.org/adventure/
[Rust]: https://www.rust-lang.org/
[Emscripten]: http://emscripten.org/
