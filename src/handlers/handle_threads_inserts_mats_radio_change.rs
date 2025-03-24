use yew::prelude::*;
use wasm_bindgen::JsValue;
use crate::handlers::handle_threads_inserts_mats_input_change::SetThreadsInsertsMatsArgs;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn handle_threads_inserts_mats_radio_change(
    threads_inserts_mats_radio_state: UseStateHandle<String>,
    threads_inserts_mats_input: UseStateHandle<String>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        threads_inserts_mats_radio_state.set(value.clone());

        if value == "no" {
            threads_inserts_mats_input.set("0".to_string());
            let args = SetThreadsInsertsMatsArgs {
                threads_inserts_mats: 0,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_threads_inserts_mats", args).await;
            });
        } else {
            threads_inserts_mats_input.set("".to_string());
        }
    })
}
