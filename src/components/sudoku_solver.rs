
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
    let handle_clear = {
        let sudoku = sudoku.clone();
        Callback::from(move |_| {
            sudoku.set(empty_sudoku());
        })
    };
    let on_change = {
        let sudoku = sudoku.clone();
        Callback::from(move |new| sudoku.set(new))
    };
    html! {
        <div class="sudoku-solver">
            <SudokuInput sudoku={*sudoku} working={*working} {on_change} />
            <div class="button-row">
                <button onclick={handle_solve}>{"Solve"}</button>
                <button onclick={handle_clear}>{"Clear"}</button>
            </div>
        </div>
    }
}

