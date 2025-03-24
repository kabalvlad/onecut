use yew::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize)]
pub struct SetThreadsInsertsMatsArgs {
    pub threads_inserts_mats: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



pub fn handle_threads_inserts_mats_input_change(
    threads_inserts_mats_input: UseStateHandle<String>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        threads_inserts_mats_input.set(value.clone());

        if let Ok(point) = value.parse::<i32>() {
            let args = SetThreadsInsertsMatsArgs {
                threads_inserts_mats: point,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_threads_inserts_mats", args).await;
            });
        } else {
            // Обработка ошибки, например, вывод сообщения об ошибке
        }
    })
}
