use yew::prelude::*;
use wasm_bindgen::JsValue;
use crate::handlers::handle_bending_points_input_change::SetBendingPointsArgs;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn handle_bending_points_radio_change(
    bending_points_radio_state: UseStateHandle<String>,
    bending_points_input: UseStateHandle<String>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        bending_points_radio_state.set(value.clone());

        if value == "no" {
            bending_points_input.set("0".to_string());
            let args = SetBendingPointsArgs {
                bending_points: 0,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_bending_points", args).await;
            });
        } else {
            bending_points_input.set("".to_string());
        }
    })
}
