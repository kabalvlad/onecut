use web_sys::Event;
use yew::prelude::*;
use std::collections::VecDeque;
use web_sys::HtmlInputElement;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Добавляем определение функции invoke
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SetCutLengthArgs {
    cut_length: f32,
}

pub fn handle_cut_length_input(
    cut_length: UseStateHandle<f32>,
    history: UseStateHandle<VecDeque<String>>
) -> Callback<Event> {
    let cut_length = cut_length.clone();
    let history = history.clone();

    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        let normalized_value = value.replace(',', ".");
        
        match f32::from_str(&normalized_value) {
            Ok(length) => {
                if length >= 0.0 {
                    cut_length.set(length);

                    let args = SetCutLengthArgs { cut_length: length };
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&args).unwrap();
                        let _ = invoke("set_cut_length", args).await;
                    });

                    let mut new_history = (*history).clone();
                    if new_history.len() >= 30 {
                        new_history.pop_back();
                    }
                    new_history.push_front(format!("Установлена длина реза: {:.2} мм", length));
                    history.set(new_history);
                }
            },
            Err(_) => {
                if !value.is_empty() {
                    let mut new_history = (*history).clone();
                    if new_history.len() >= 30 {
                        new_history.pop_back();
                    }
                    new_history.push_front("Ошибка: введите корректное числовое значение".to_string());
                    history.set(new_history);
                }
            }
        }
    })
}
