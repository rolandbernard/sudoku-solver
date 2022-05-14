
use yew::{prelude::*, Properties};
use web_sys::HtmlElement;

use crate::solver::sudoku::Sudoku;

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
    } else if key == 40 {
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
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub sudoku: Sudoku,
    #[prop_or(false)]
    pub working: bool,
    pub on_change: Callback<Sudoku>,
}

#[function_component(SudokuInput)]
pub fn sudoku_input(props: &Props) -> Html {
    let Props {sudoku, working, on_change} = props;
    let selected = use_state(|| None);
    let mut cells = Vec::new();
    for _ in 0..9 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(use_node_ref());
        }
        cells.push(row);
    }
    let onkeydown = {
        let selected = selected.clone();
        let sudoku = sudoku.clone();
        let on_change = on_change.clone();
        let cells = cells.clone();
        Callback::from(move |e| {
            if let Some((r, c)) = *selected {
                focus_change(&cells, r, c, &e);
                on_change.emit(sudoku_change(sudoku, r, c, &e));
            }
        })
    };
    let onclick = |r, c| {
        let selected = selected.clone();
        Callback::from(move |_| {
            selected.set(Some((r, c)));
        })
    };
    let onblur = {
        let selected = selected.clone();
        Callback::from(move |_| {
            selected.set(None);
        })
    };
    let mut grid_classes = Vec::with_capacity(2);
    if *working {
        grid_classes.push("sudoku-working");
    }
    html! {
        <div class={classes!("sudoku-grid", grid_classes)} {onkeydown} {onblur}>
            { (0..9).map(|r|
                (0..9).map(|c| {
                    let mut cell_classes = Vec::with_capacity(4);
                    cell_classes.push(format!("sudoku-cell-{}-x", r));
                    cell_classes.push(format!("sudoku-cell-x-{}", c));
                    if let Some((sr, sc)) = *selected {
                        if (r, c) == (sr, sc) {
                            cell_classes.push("sudoku-cell-selected".to_owned());
                        }
                        if r == sr || c == sc || (r / 3, c / 3) == (sr / 3, sc / 3) {
                            cell_classes.push("sudoku-cell-constraint".to_owned());
                        }
                    }
                    html! {
                        <div
                            id={format!("sudoku-cell-{}-{}", r, c)}
                            class={classes!("sudoku-cell", cell_classes)}
                            onfocus={onclick(r, c)}
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
                    }
                }).collect::<Html>()
            ).collect::<Html>() }
        </div>
    }
}
