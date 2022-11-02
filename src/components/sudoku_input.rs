use web_sys::HtmlElement;
use yew::{prelude::*, Children, Properties};

use crate::solver::sudoku::{empty_domains, Sudoku, SudokuDomains};

fn change_sudoku<const N: usize>(
    mut sudoku: Sudoku<N>,
    row: usize,
    col: usize,
    put: Option<u32>,
) -> Sudoku<N> {
    sudoku[row][col] = put;
    return sudoku;
}

fn sudoku_change<const N: usize>(
    mut sudoku: Sudoku<N>,
    row: usize,
    col: usize,
    event: &KeyboardEvent,
) -> Sudoku<N> {
    let key = event.key_code();
    if key >= ('1' as u32) && key <= ('9' as u32) && key - ('0' as u32) <= N as u32 {
        sudoku[row][col] = Some(key - ('0' as u32));
    } else if key >= ('A' as u32) && key <= ('F' as u32) && 10 + key - ('A' as u32) <= N as u32 {
        sudoku[row][col] = Some(10 + key - ('A' as u32));
    } else if key >= ('0' as u32) && 16 <= N {
        sudoku[row][col] = Some(16);
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
    #[prop_or(empty_domains())]
    pub unsure: SudokuDomains<N>,
    #[prop_or(false)]
    pub working: bool,
    pub on_change: Callback<Sudoku<N>>,
}

#[function_component(SudokuInput)]
pub fn sudoku_input<const N: usize>(props: &Props<N>) -> Html {
    let Props {
        children,
        sudoku,
        domains,
        unsure,
        working,
        on_change,
    } = props;
    let selected = use_state_eq(|| None);
    let last = use_state_eq(|| None);
    let mut cells = Vec::new();
    for _ in 0..N {
        let mut row = Vec::new();
        for _ in 0..N {
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
    html! {
        <div class="sudoku-input-wrapper">
            <div class={classes!("sudoku-grid-wrapper", grid_classes)}>
                <div class="status-row">{ children.clone() }</div>
                <div class={classes!("sudoku-grid", format!("sudoku-grid-{N}"))} {onkeydown} {onblur}>
                    { (0..N).map(|r|
                        (0..N).map(|c| {
                            let mut cell_classes = Vec::with_capacity(5);
                            cell_classes.push(format!("sudoku-cell-{}-x", r));
                            cell_classes.push(format!("sudoku-cell-x-{}", c));
                            if let Some((sr, sc)) = *selected {
                                if (r, c) == (sr, sc) {
                                    cell_classes.push("sudoku-cell-selected".to_owned());
                                }
                                let sq = (N as f64).sqrt() as usize;
                                if r == sr || c == sc || (r / sq, c / sq) == (sr / sq, sc / sq) {
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
                                        { domains[r][c].clone().map(|e| html!{
                                            <div class={classes!(
                                                if unsure[r][c].contains(e) {
                                                    "sudoku-result-unsure"
                                                } else {
                                                    "sudoku-result-sure"
                                                }
                                            )}>{format!("{:X}", (e + 1) & 0xf)}</div>
                                        }).collect::<Html>() }
                                    </div>
                                    <div class="sudoku-cell-input" tabindex="0" type="number" ref={cells[r][c].clone()}>{
                                        if let Some(v) = sudoku[r][c] {
                                            format!("{:X}", v & 0xf)
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
            <div class={classes!("sudoku-input", format!("sudoku-input-{N}"))}>
                { (0..=N as u32).map(|n| html! {
                    <button
                        class={classes!("number-button", format!("number-button-{}", n))}
                        onclick={onclick(n)}
                    >{ if n == 0 { "_".to_owned() } else { format!("{:X}", n & 0xf) } }</button>
                }).collect::<Html>() }
            </div>
        </div>
    }
}
