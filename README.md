
Sudoku solver
=============

This is a very simple web app for solving Sudoku problems.
This is more just an experiment for me to try out the [Yew](https://yew.rs/) framework.

## Demo

* Normal Sudoku: https://rolandbernard.github.io/sudoku-solver/
* 16x16 "Sudoku": https://rolandbernard.github.io/sudoku-solver/16

## Development

You will have to install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
and [Trunk](https://trunkrs.dev/#install).

### Running

To start a development server run the following command in the root directory of this repository.
```sh
trunk serve
```

### Building

To build a release version of the application run the following command, replacing `<PUBLIC_URL>`
with the public URL will be served from (the default is `/`):
```sh
trunk build --release --public-url <PUBLIC_URL>
```

### Files

This web app is written in rust using the [Yew](https://yew.rs/) framework.
You will find all the styles in the `scss/` directory, with an entry at `scss/index.scss`.

The source for the app can be found in the `src/` directory. With `src/main.rs` being the entry
point into the web application.

The actual problem solver can be found in the `src/solver` subdirectory. It is a very simple
constraint solver, that can technically handle not just Sudoku but also some other simple
“Sudoku-like“ problems.

