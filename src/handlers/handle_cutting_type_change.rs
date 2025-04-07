use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::{AppState, AppAction};

//===============================================================
// Обработчик для выбора типа резки
//===============================================================

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetTypeCuttingArgs {
    pub cutting_type: String,
}

pub fn handle_cutting_type_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Получаем код и описание типа резки
        let (cutting_code, cutting_description) = match value.as_str() {
            "laser" => ("LC", "Лазерная резка"),
            "plasma" => ("PC", "Плазменная резка"),
            "hydro" => ("HC", "Гидроабразивная резка"),
            _ => ("", "Неизвестный тип резки"),
        };
        
        // Обновляем выбранный тип резки через редуктор
        state.dispatch(AppAction::SetCuttingType(value.clone()));
        state.dispatch(AppAction::AddHistoryMessage(
            format!("Выбран тип резки: {}", cutting_description)
        ));
        
        // Проверяем, доступен ли Tauri API
        let is_tauri_available = web_sys::window()
            .and_then(|win| js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__")).ok())
            .map(|tauri| !tauri.is_undefined() && !tauri.is_null())
            .unwrap_or(false);
        
        if is_tauri_available {
            // Отправляем информацию о типе резки на бэкенд через Tauri
            let args = SetTypeCuttingArgs {
                cutting_type: cutting_code.to_string(),
            };
            let state_clone = state.clone();
            
            spawn_local(async move {
                match serde_wasm_bindgen::to_value(&args) {
                    Ok(args_js) => {
                        let result = invoke("set_type_cutting", args_js).await;
                        if result.is_undefined() {
                            // Логируем ошибку в консоль
                            web_sys::console::error_1(
                                &format!("Ошибка при установке типа резки {} в backend", cutting_code).into()
                            );
                            // Отправляем сообщение об ошибке
                            state_clone.dispatch(AppAction::AddHistoryMessage(
                                format!("Ошибка: не удалось установить тип резки {} в системе", cutting_description)
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
            // Если Tauri недоступен, просто выводим сообщение
            web_sys::console::log_1(&"Tauri API недоступен, работаем в режиме веб-приложения".into());
            state.dispatch(AppAction::AddHistoryMessage(
                format!("Тип резки {} установлен (режим веб-приложения)", cutting_description)
            ));
        }
    })
}
