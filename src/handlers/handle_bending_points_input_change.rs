use yew::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize)]
pub struct SetBendingPointsArgs {
    pub bending_points: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



pub fn handle_bending_points_input_change(
    bending_points_input: UseStateHandle<String>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        bending_points_input.set(value.clone());

        if let Ok(points) = value.parse::<i32>() {
            let args = SetBendingPointsArgs {
                bending_points: points,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_bending_points", args).await;
            });
        } else {
            // Обработка ошибки, например, вывод сообщения об ошибке
        }
    })
}
