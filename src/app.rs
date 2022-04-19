
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let inc = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let dec = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };
    html! {
        <div>
            <button onclick={inc}>{ "INC value" }</button>
            <button onclick={dec}>{ "DED value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}

