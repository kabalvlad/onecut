use web_sys::Event;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::models::{AppState, AppAction};

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
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        let normalized_value = value.replace(',', ".");
        

        
        match f32::from_str(&normalized_value) {
            Ok(length) => {
                if length >= 0.0 {
                    // Обновляем длину реза через dispatch
                    state.dispatch(AppAction::SetCutLength(length));

                    // Отправляем данные на бэкенд
                    let args = SetCutLengthArgs { cut_length: length };
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&args).unwrap();
                        let _ = invoke("set_cut_length", args).await;
                    });

                    // Добавляем сообщение в историю
                    state.dispatch(AppAction::AddHistoryMessage(
                        format!("Установлена длина реза: {:.2} мм", length)
                    ));
                }
            },
            Err(_) => {
                if !value.is_empty() {
                    // Добавляем сообщение об ошибке в историю
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: введите корректное числовое значение".to_string()
                    ));
                }
            }
        }
    })
}
