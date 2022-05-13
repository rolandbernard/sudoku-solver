
use yew::prelude::*;
use yew_agent::use_bridge;
use web_sys::HtmlElement;

use crate::solver::sudoku::{Sudoku, empty_sudoku};
use crate::agent::{Worker, WorkerInput};

fn sudoku_change(mut sudoku: Sudoku, row: usize, col: usize, event: &KeyboardEvent) -> Sudoku {
    let key = event.key_code();
    if key >= ('1' as u32) && key <= ('9' as u32) {
        sudoku[row][col] = Some(key - ('0' as u32));
    } else if key == (' ' as u32) || key == 8 {
        sudoku[row][col] = None;
    }
    return sudoku;
}

fn focus_change(cells: &Vec<Vec<NodeRef>>, row: usize, col: usize, event: &KeyboardEvent) {
    let key = event.key_code();
    if key == 38 {
        if row != 0 {
            if let Some(elem) = cells[row - 1][col].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    } else if key == 40 || key == 13 {
        if row + 1 < cells.len() {
            if let Some(elem) = cells[row + 1][col].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    } else if key == 37 {
        if col != 0 {
            if let Some(elem) = cells[row][col - 1].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    } else if key == 39 {
        if col + 1 < cells[row].len() {
            if let Some(elem) = cells[row][col + 1].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    } else if key == 8 {
        if col != 0 {
            if let Some(elem) = cells[row][col - 1].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        } else if row != 0 {
            if let Some(elem) = cells[row - 1][cells[row - 1].len() - 1].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    } else if (key >= ('1' as u32) && key <= ('9' as u32)) || key == (' ' as u32) {
        if col + 1 < cells[row].len() {
            if let Some(elem) = cells[row][col + 1].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        } else if row + 1 < cells.len() {
            if let Some(elem) = cells[row + 1][0].cast::<HtmlElement>() {
                _ = elem.focus();
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| "sudoku-idle");
    let sudoku = use_state(|| empty_sudoku());
    let solver_bridge = use_bridge::<Worker, _>({
        let state = state.clone();
        let sudoku = sudoku.clone();
        move |sol| {
            sudoku.set(sol.sudoku);
            state.set("sudoku-idle");
        }
    });
    let handle_solve = {
        let sudoku = sudoku.clone();
        let state = state.clone();
        Callback::from(move |_| {
            if *state == "sudoku-idle" {
                state.set("sudoku-solving");
                solver_bridge.send(WorkerInput { sudoku: *sudoku });
            }
        })
    };
    let mut cells = Vec::new();
    for _ in 0..9 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(use_node_ref());
        }
        cells.push(row);
    }
    let handle_keyevent = |r, c| {
        let sudoku = sudoku.clone();
        let cells = cells.clone();
        Callback::from(move |e| {
            sudoku.set(sudoku_change(*sudoku, r, c, &e));
            focus_change(&cells, r, c, &e);
            e.prevent_default();
            e.stop_propagation();
        })
    };
    html! {
        <div class="app">
            <div class="page-title">
                {"Sudoku solver"}
            </div>
            <div class="sudoku-wrapper">
                <div class={classes!("sudoku-grid", *state)}>
                    { (0..9).map(|r|
                        (0..9).map(|c| html! {
                            <div
                                class={format!("sudoku-cell sudoku-cell-{}-x sudoku-cell-x-{}", r, c)}
                                id={format!("sudoku-cell-{}-{}", r, c)} onkeydown={handle_keyevent(r, c)}
                            >
                                <div class="sudoku-cell-result"></div>
                                <div class="sudoku-cell-input" tabindex="0" type="number" ref={cells[r][c].clone()}>{
                                    if let Some(v) = sudoku[r][c] {
                                        v.to_string()
                                    } else {
                                        "".to_owned()
                                    }
                                }</div>
                            </div>
                        }).collect::<Html>()
                    ).collect::<Html>() }
                </div>
                <button onclick={handle_solve}>{"Solve"}</button>
            </div>
            <div class="page-footer">
                <a rel="noreferrer" href="https://github.com/rolandbernard/sudoku-solver">
                    <svg xmlns="http://www.w3.org/2000/svg" width="44" height="44" viewBox="0 0 11.641667 11.641667" version="1.1">
                        <g transform="matrix(0.35277777,0,0,-0.35277777,18.013154,-1.5555087)">
                            <path d="m -34.562409,-5.0218832 c -8.995,0 -16.288,-7.2929998 -16.288,-16.2899998 0,-7.197 4.667,-13.302 11.14,-15.457 0.815,-0.149 1.112,0.354 1.112,0.786 0,0.386 -0.014,1.411 -0.022,2.77 -4.531,-0.984 -5.487,2.184 -5.487,2.184 -0.741,1.882 -1.809,2.383 -1.809,2.383 -1.479,1.01 0.112,0.99 0.112,0.99 1.635,-0.115 2.495,-1.679 2.495,-1.679 1.453,-2.489 3.813,-1.77 4.741,-1.353 0.148,1.052 0.568,1.77 1.034,2.177 -3.617,0.411 -7.42,1.809 -7.42,8.051 0,1.778 0.635,3.232 1.677,4.371 -0.168,0.412 -0.727,2.068 0.159,4.311 0,0 1.368,0.438 4.48,-1.67 1.299,0.362 2.693,0.542 4.078,0.548 1.383,-0.006 2.777,-0.186 4.078,-0.548 3.11,2.108 4.475,1.67 4.475,1.67 0.889,-2.243 0.33,-3.899 0.162,-4.311 1.044,-1.139 1.675,-2.593 1.675,-4.371 0,-6.258 -3.809,-7.635 -7.438,-8.038 0.585,-0.503 1.106,-1.497 1.106,-3.017 0,-2.177 -0.02,-3.934 -0.02,-4.468 0,-0.436 0.293,-0.943 1.12,-0.784 6.468,2.159 11.131,8.26 11.131,15.455 0,8.997 -7.294,16.2899998 -16.291,16.2899998" style="fill-opacity:1;fill-rule:evenodd;stroke:none"></path>
                        </g>
                    </svg>
                    <span>{"GitHub"}</span>
                </a>
            </div>
        </div>
    }
}

