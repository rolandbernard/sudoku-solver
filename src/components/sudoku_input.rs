
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
        let on_change = on_change.clone();
        let cells = cells.clone();
        Callback::from(move |e| {
            focus_change(&cells, r, c, &e);
            on_change.emit(sudoku_change(sudoku, r, c, &e));
        })
    };
    let mut styles = Vec::with_capacity(2);
    if *working {
        styles.push("sudoku-working");
    }
    html! {
        <div class={classes!("sudoku-grid", styles)}>
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
    }
}

