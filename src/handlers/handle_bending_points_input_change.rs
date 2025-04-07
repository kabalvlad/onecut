use yew::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::{AppState, AppAction};
use std::str::FromStr;

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
        
        match i32::from_str(&normalized_value) {
            Ok(points) => {
                if points >= 0 {
                    // Обновляем количество точек сгиба через dispatch
                    state.dispatch(AppAction::SetBendingPoints { 
                        enabled: true, 
                        count: Some(points) 
                    });
                    
                    // Сразу выводим сообщение о установке точек сгиба
                    state.dispatch(AppAction::AddHistoryMessage(
                        format!("Установлено количество точек сгиба: {}", points)
                    ));

                    // Отправляем данные на бэкенд
                    let args = SetBendingPointsArgs { bending_points: points };
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
                        
                        // Просто вызываем бэкенд без дополнительных сообщений об успехе
                        let _ = invoke("set_bending_points", args).await;
                    });
                } else {
                    // Отрицательное значение
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: количество точек сгиба не может быть отрицательным".to_string()
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
