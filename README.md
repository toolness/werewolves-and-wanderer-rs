This is an implementation of _Werewolves and Wanderer_, the first
project in Tim Hartnell's [Creating Adventure Games on Your Computer][cagyc],
in Rust.

You can [try it in your browser][play].

Alternatively, it can also be run as a command-line program.

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

If you want to try the web version, you can install [Emscripten][] and
[NodeJS][] and run:

```
npm install
bash build-emscripten.sh
cd static
python -m SimpleHTTPServer
```

Then visit http://localhost:8000/ in your browser.

You can also deploy the `static/` directory to any webserver that
hosts static files.

## Debug mode

On non-release builds, a debug menu is included to aid in debugging.
Press the backtick key in the game's primary exploration mode to
enter it.

## Implementation notes

* Because of the fact that the web version can't block the UI
  thread to wait for user input--well, at least not in a
  user-friendly way--the program's source code is actually capable
  of running with or without blocking I/O, depending on the
  context.

  While this made it possible to write a program that ran
  via the command-line and the web, it also made things a bit
  confusing and cumbersome.

  In the future I might try using something like [`futures-rs`][]
  instead.

* When I originally started this project, I wanted it to work
  on bare metal setups that lacked a heap. I'm not sure *why* I
  wanted to do that, but I did, and eventually it became too
  cumbersome so I changed my mind. As a result, though, parts
  of the codebase are a bit, um, idosyncratic.

[cagyc]: http://www.atariarchives.org/adventure/
[play]: https://toolness.github.io/werewolves-and-wanderer-rs/
[Rust]: https://www.rust-lang.org/
[Emscripten]: http://emscripten.org/
[NodeJS]: http://nodejs.org/
[`futures-rs`]: https://github.com/alexcrichton/futures-rs
