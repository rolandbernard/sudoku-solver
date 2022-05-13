
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::solver::sudoku::empty_sudoku;
use crate::workers::SolvingWorker;
use crate::components::sudoku_input::SudokuInput;

#[function_component(SudokuSolver)]
pub fn sudoku_solver() -> Html {
    let working = use_state(|| false);
    let sudoku = use_state(|| empty_sudoku());
    let solver_bridge = use_bridge::<SolvingWorker, _>({
        let working = working.clone();
        let sudoku = sudoku.clone();
        move |sol| {
            sudoku.set(sol);
            working.set(false);
        }
    });
    let handle_solve = {
        let sudoku = sudoku.clone();
        let working = working.clone();
        Callback::from(move |_| {
            if !*working {
                working.set(true);
                solver_bridge.send(*sudoku);
            }
        })
    };
    let on_change = {
        let sudoku = sudoku.clone();
        Callback::from(move |new| sudoku.set(new))
    };
    html! {
        <div class="sudoku-solver">
            <SudokuInput sudoku={*sudoku} working={*working} {on_change} />
            <button onclick={handle_solve}>{"Solve"}</button>
        </div>
    }
}

