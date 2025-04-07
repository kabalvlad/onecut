use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use crate::models::{AppState, AppAction};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetThicknessArgs {
    pub thickness: f32,
}

pub fn handle_thickness_input(
    state: UseReducerHandle<AppState>,
    all_thicknesses: Vec<f32>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        // Обновляем значение поля ввода
        state.dispatch(AppAction::SetThickness(value.clone()));
        
        // Если строка пустая, просто выходим
        if value.trim().is_empty() {
            return;
        }

        // Заменяем запятую на точку для корректного парсинга
        let normalized_value = value.replace(',', ".");
        
        // Проверяем, содержит ли строка только цифры, точку или запятую
        if !normalized_value.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            state.dispatch(AppAction::AddHistoryMessage(
                "Ошибка: Пожалуйста, введите только числовое значение толщины без посторонних символов".to_string()
            ));
            return;
        }

        // Пытаемся преобразовать строку в число
        match f32::from_str(&normalized_value) {
            Ok(input_num) => {
                if input_num <= 0.0 {
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: Толщина должна быть положительным числом".to_string()
                    ));
                    return;
                }

                // Находим ближайшее значение из списка
                let nearest = all_thicknesses
                    .iter()
                    .min_by(|&&a, &b| {
                        (a - input_num).abs()
                            .partial_cmp(&(b - input_num).abs())
                            .unwrap()
                    })
                    .copied()
                    .unwrap_or(all_thicknesses[0]);
                
                // Обновляем поле ввода и добавляем сообщение в историю
                state.dispatch(AppAction::SetThickness(nearest.to_string()));
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Выбрана ближайшая доступная толщина: {} мм", nearest)
                ));

                // Проверяем доступность Tauri API
                let is_tauri_available = web_sys::window()
                    .and_then(|win| js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__")).ok())
                    .map(|tauri| !tauri.is_undefined() && !tauri.is_null())
                    .unwrap_or(false);

                if is_tauri_available {
                    // Отправляем значение в backend через Tauri
                    let args = SetThicknessArgs { thickness: nearest };
                    let state_clone = state.clone();

                    spawn_local(async move {
                        match serde_wasm_bindgen::to_value(&args) {
                            Ok(args_js) => {
                                let result = invoke("set_thickness", args_js).await;
                                if result.is_undefined() {
                                    web_sys::console::error_1(
                                        &format!("Ошибка при установке толщины {} мм в backend", nearest).into()
                                    );
                                    state_clone.dispatch(AppAction::AddHistoryMessage(
                                        format!("Ошибка: не удалось установить толщину {} мм в системе", nearest)
                                    ));
                                }
                            },
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("Ошибка сериализации данных: {:?}", e).into()
                                );
                            }
                        }
                    });
                } else {
                    // Если Tauri недоступен, выводим сообщение
                    web_sys::console::log_1(&"Tauri API недоступен, работаем в режиме веб-приложения".into());
                }
            },
            Err(_) => {
                state.dispatch(AppAction::AddHistoryMessage(
                    "БОРОДА что ТЫ ДЕЛАЕШЬ ??? : дай нормальне число ".to_string()
                ));
            }
        }
    })
}
