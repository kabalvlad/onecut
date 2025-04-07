use yew::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::{AppState, AppAction};

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
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Если значение пустое, просто выходим
        if value.trim().is_empty() {
            return;
        }

        match value.parse::<i32>() {
            Ok(points) => {
                if points >= 0 {
                    // Обновляем количество точек через dispatch
                    state.dispatch(AppAction::SetThreadsInsertsMats { 
                        enabled: true, 
                        count: Some(points) 
                    });
                    
                    // Сразу выводим сообщение о установке точек
                    state.dispatch(AppAction::AddHistoryMessage(
                        format!("Установлено количество резьбы/вставок/цековок: {}", points)
                    ));

                    // Отправляем данные на бэкенд
                    let args = SetThreadsInsertsMatsArgs { 
                        threads_inserts_mats: points 
                    };
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        let args = match serde_wasm_bindgen::to_value(&args) {
                            Ok(args) => args,
                            Err(_) => return, // Просто выходим в случае ошибки сериализации
                        };
                        
                        // Просто вызываем бэкенд без дополнительных сообщений об успехе
                        let _ = invoke("set_threads_inserts_mats", args).await;
                    });
                } else {
                    // Отрицательное значение
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: количество резьбы/вставок/цековок не может быть отрицательным".to_string()
                    ));
                }
            },
            Err(_) => {
                // Некорректное числовое значение
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Ошибка: '{}' не является корректным числовым значением", value)
                ));
            }
        }
    })
}
