
Sudoku solver
=============

This is a very simple web app for solving Sudoku problems.
This is more just an experiment for me to try out the [Yew](https://yew.rs/) framework

## Development

This web app is written in rust using the [Yew](https://yew.rs/) framework.
You will find all the styles in the `index.css` file.

The source for the app can be found in the `src/` directory. With `src/main.rs` being the entry
point into the web application.

The actual problem solver can be found in the `src/solver` subdirectory. It is a very simple
constraint solver, that can technically handle not just Sudoku but also some other simple
“Sudoku-like“ problems.

