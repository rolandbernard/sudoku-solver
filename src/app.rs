
use yew::prelude::*;
use web_sys::HtmlElement;

type Sudoku = [[Option<u32>; 9]; 9];

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
    let mut cells = Vec::new();
    for _ in 0..9 {
        let mut row = Vec::new();
        for _ in 0..9 {
            row.push(use_node_ref());
        }
        cells.push(row);
    }
    let sudoku = use_state(|| [[Option::<u32>::None; 9]; 9]);
    html! {
        <div class="sudoku-wrapper">
            <div class="sudoku-grid">
                { (0..9).map(|r|
                    (0..9).map(|c| html! {
                        <div
                            class={format!("sudoku-cell sudoku-cell-{}-x sudoku-cell-x-{}", r, c)}
                            id={format!("sudoku-cell-{}-{}", r, c)}
                        >
                            <div class="sudoku-cell-result"></div>
                            <div class="sudoku-cell-input" tabindex="0" ref={cells[r][c].clone()} onkeydown={
                                let sudoku = sudoku.clone();
                                let cells = cells.clone();
                                Callback::from(move |e| {
                                    sudoku.set(sudoku_change(*sudoku, r, c, &e));
                                    focus_change(&cells, r, c, &e);
                                })
                            }>{
                                if let Some(v) = sudoku[r][c] {
                                    html! { {v} }
                                } else {
                                    html! { }
                                }
                            }</div>
                        </div>
                    }).collect::<Html>()
                ).collect::<Html>() }
            </div>
        </div>
    }
}

