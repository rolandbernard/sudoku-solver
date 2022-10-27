
use yew::{prelude::*, Properties, Children};
use web_sys::HtmlElement;

use crate::solver::sudoku::{empty_domains, Sudoku, SudokuDomains};

fn change_sudoku<const N: usize>(mut sudoku: Sudoku<N>, row: usize, col: usize, put: Option<u32>) -> Sudoku<N> {
    sudoku[row][col] = put;
    return sudoku;
}

fn sudoku_change<const N: usize>(mut sudoku: Sudoku<N>, row: usize, col: usize, event: &KeyboardEvent) -> Sudoku<N> {
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
pub struct Props<const N: usize> {
    pub children: Children,
    pub sudoku: Sudoku<N>,
    #[prop_or(empty_domains())]
    pub domains: SudokuDomains<N>,
    #[prop_or(false)]
    pub working: bool,
    #[prop_or(false)]
    pub reducing: bool,
    pub on_change: Callback<Sudoku<N>>,
}

#[function_component(SudokuInput)]
pub fn sudoku_input<const N: usize>(props: &Props<N>) -> Html {
    let Props {children, sudoku, domains, working, reducing, on_change} = props;
    let selected = use_state_eq(|| None);
    let last = use_state_eq(|| None);
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
    let onclick = |v| {
        let last = last.clone();
        let sudoku = sudoku.clone();
        let on_change = on_change.clone();
        Callback::from(move |_| {
            if let Some((r, c)) = *last {
                if v == 0 {
                    on_change.emit(change_sudoku(sudoku, r, c, None));
                } else {
                    on_change.emit(change_sudoku(sudoku, r, c, Some(v)));
                }
            }
        })
    };
    let onfocus = |r, c| {
        let selected = selected.clone();
        let last = last.clone();
        Callback::from(move |_| {
            selected.set(Some((r, c)));
            last.set(Some((r, c)));
        })
    };
    let onblur = {
        let selected = selected.clone();
        Callback::from(move |_| {
            selected.set(None);
        })
    };
    let mut grid_classes = Vec::with_capacity(3);
    if *working {
        grid_classes.push("sudoku-working");
    }
    if *reducing {
        grid_classes.push("sudoku-reducing");
    }
    html! {
        <div class="sudoku-input-wrapper">
            <div class={classes!("sudoku-grid-wrapper", grid_classes)}>
                <div class="status-row">{ children.clone() }</div>
                <div class="sudoku-grid" {onkeydown} {onblur}>
                    { (0..9).map(|r|
                        (0..9).map(|c| {
                            let mut cell_classes = Vec::with_capacity(5);
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
                                    class={classes!("sudoku-cell", sudoku[r][c].and_then(|_| Some("sudoku-cell-set")), cell_classes)}
                                    onfocus={onfocus(r, c)}
                                >
                                    <div class={classes!("sudoku-cell-result", format!("sudoku-results-{}", domains[r][c].len()))}>
                                        { domains[r][c].clone().map(|e| html!{ <div>{(e + 1).to_string()}</div> }).collect::<Html>() }
                                    </div>
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
            </div>
            <div class="sudoku-input">
                { (0..=9).map(|n| html! {
                    <button
                        class={classes!("number-button", format!("number-button-{}", n))}
                        onclick={onclick(n)}
                    >{ if n == 0 { "_".to_owned() } else { n.to_string() } }</button>
                }).collect::<Html>() }
            </div>
        </div>
    }
}

