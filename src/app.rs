
use yew::prelude::*;

type Sudoku = [[Option<u32>; 9]; 9];

fn sudoku_change(mut sudoku: Sudoku, row: usize, col: usize, event: KeyboardEvent) -> Sudoku {
    let key = event.key_code();
    if key >= ('1' as u32) && key <= ('9' as u32) {
        sudoku[row][col] = Some(key - ('0' as u32));
    } else if key == (' ' as u32) || key == 8 {
        sudoku[row][col] = None;
    }
    return sudoku;
}

#[function_component(App)]
pub fn app() -> Html {
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
                            <div class="sudoku-cell-input" tabindex="0" onkeydown={
                                let sudoku = sudoku.clone();
                                Callback::from(move |e| {
                                    sudoku.set(sudoku_change(*sudoku, r, c, e));
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

