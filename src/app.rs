
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="sudoku-wrapper">
            <div class="sudoku-grid">
                { (0..9).map(|i|
                    (0..9).map(|j| html! {
                        <div
                            class={format!("sudoku-cell sudoku-cell-{}-x sudoku-cell-x-{}", i, j)}
                            id={format!("sudoku-cell-{}-{}", i, j)}
                        >
                            <div class="sudoku-cell-input" tabindex="0">{(i + j) % 9 + 1}</div>
                        </div>
                    }).collect::<Html>()
                ).collect::<Html>() }
            </div>
        </div>
    }
}

