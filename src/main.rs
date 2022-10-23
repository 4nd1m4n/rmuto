use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    html! {
      <div>
        <button class="border-2 border-solid bg-stone-300 border-stone-600" {onclick}>{ "+1 Inc." }</button>
        <p class="text-3xl font-bold underline bg-blue-800">{ state.value }</p>
      </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
