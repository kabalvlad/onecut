use web_sys::Event;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::models::{AppState, AppAction};

// Определение функции invoke для взаимодействия с Tauri
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
        
        // Если значение пустое, просто выходим
        if value.trim().is_empty() {
            return;
        }
        
        // Нормализуем значение, заменяя запятые на точки
        let normalized_value = value.replace(',', ".");
        
        match f32::from_str(&normalized_value) {
            Ok(length) => {
                if length >= 0.0 {
                    // Обновляем длину реза через dispatch
                    state.dispatch(AppAction::SetCutLength(length));
                    
                    // Сразу выводим сообщение о установке длины
                    state.dispatch(AppAction::AddHistoryMessage(
                        format!("Установлена длина реза: {:.2} мм", length)
                    ));

                    // Отправляем данные на бэкенд
                    let args = SetCutLengthArgs { cut_length: length };
                    let state_clone = state.clone();
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        let args = match serde_wasm_bindgen::to_value(&args) {
                            Ok(args) => args,
                            Err(_) => {
                                state_clone.dispatch(AppAction::AddHistoryMessage(
                                    "Ошибка: не удалось сериализовать данные для отправки".to_string()
                                ));
                                return;
                            }
                        };
                        
                        match invoke("set_cut_length", args).await.as_bool() {
                            Some(true) => {
                                // Успешно отправлено на бэкенд
                                state_clone.dispatch(AppAction::AddHistoryMessage(
                                    "Длина реза успешно сохранена на сервере".to_string()
                                ));
                            },
                            _ => {
                                // Ошибка при отправке на бэкенд
                                state_clone.dispatch(AppAction::AddHistoryMessage(
                                    "Ошибка: не удалось отправить длину реза на сервер".to_string()
                                ));
                                // Можно также сбросить значение в состоянии, если нужно
                                // state_clone.dispatch(AppAction::SetCutLength(0.0));
                            }
                        }
                    });
                } else {
                    // Отрицательное значение
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: длина реза не может быть отрицательной".to_string()
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
