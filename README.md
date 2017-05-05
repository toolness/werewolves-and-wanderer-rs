[![Build Status](https://travis-ci.org/toolness/werewolves-and-wanderer-rs.svg?branch=master)](https://travis-ci.org/toolness/werewolves-and-wanderer-rs)

This is an implementation of _Werewolves and Wanderer_, the first
project in Tim Hartnell's [Creating Adventure Games on Your Computer][cagyc],
in Rust.

You can [try it in your browser][play]. It should work on browsers
at least as old as IE11, as well as a variety of screen readers.

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

* The architecture of the program (very) loosely follows that
  of the original BASIC program from CAGYC. In particular, this
  means that the game world isn't particularly object-oriented.
  It feels a bit baroque in some ways. I don't think I used
  dynamic dispatch at all.

  That said, I did go a bit overboard with some abstractions,
  so there's a bit of variety.

* Some of the "bugs" in the program are intentional features
  from the original BASIC implementation. For example, the
  chapter on [Interpreting Your Commands][chapter 9] states:

  > You can enter the full word (such as NORTH) or just the
  > first letter. As you can see, the last part of line 460
  > cuts the input down to a single letter anyway, so you may
  > as well just enter the first letter of your command.

  This actually results in lots of unexpected behavior, as
  e.g. "eat" causes the player to go east rather than consume
  a unit of food.

  I found this funny so I left it in.

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

  **Update:** While `futures-rs` was a bit overwhelming for me,
  I did manage to refactor things to make them (hopefully)
  less confusing and cumbersome in [#1][].

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
[chapter 9]: http://www.atariarchives.org/adventure/chapter9.php
[#1]: https://github.com/toolness/werewolves-and-wanderer-rs/pull/1
