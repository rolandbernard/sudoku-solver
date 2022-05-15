
use yew::prelude::*;
use yew_agent::use_bridge;

use crate::solver::sudoku::{empty_sudoku, default_domains};
use crate::workers::{SolvingWorker, ReducingWorker};
use crate::components::sudoku_input::SudokuInput;

#[function_component(SudokuSolver)]
pub fn sudoku_solver() -> Html {
    let working = use_state(|| false);
    let reducing = use_state(|| 0);
    let sudoku = use_state(|| empty_sudoku());
    let domains = use_state(|| default_domains());
    let reduce_bridge = use_bridge::<ReducingWorker, _>({
        let domains = domains.clone();
        let reducing = reducing.clone();
        move |sol| {
            domains.set(sol);
            reducing.set(*reducing - 1);
        }
    });
    let on_change = {
        let sudoku = sudoku.clone();
        let reducing = reducing.clone();
        Callback::from(move |new| {
            sudoku.set(new);
            reducing.set(*reducing + 1);
            reduce_bridge.send(new);
        })
    };
    let solver_bridge = use_bridge::<SolvingWorker, _>({
        let on_change = on_change.clone();
        let working = working.clone();
        move |res| {
            if let Some(sol) = res {
                on_change.emit(sol);
            }
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
        let on_change = on_change.clone();
        Callback::from(move |_| {
            on_change.emit(empty_sudoku());
        })
    };
    html! {
        <div class="sudoku-solver">
            <SudokuInput
                sudoku={*sudoku}
                domains={*domains}
                working={*working}
                reducing={*reducing != 0}
                {on_change}
            />
            <div class="button-row">
                <button onclick={handle_solve} disabled={*working}>{"Solve"}</button>
                <button onclick={handle_clear}>{"Clear"}</button>
            </div>
        </div>
    }
}

