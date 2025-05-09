use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::{AppState, AppAction};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetMaterialArgs {
    pub material_type: String,
}

// Структура для хранения информации о материалах
struct MaterialInfo {
    code: &'static str,
    description: &'static str,
}

pub fn handle_material_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Получаем информацию о материале
        let material_info = match value.as_str() {
            "aluminum" => MaterialInfo { 
                code: "AL", 
                description: "Алюминий" 
            },
            "steel" => MaterialInfo { 
                code: "ST", 
                description: "Сталь" 
            },
            "stainless" => MaterialInfo { 
                code: "SS", 
                description: "Нержавеющая сталь" 
            },
            "copper" => MaterialInfo { 
                code: "COP", 
                description: "Латунь/Бронза/Медь" 
            },
            "plastic" => MaterialInfo { 
                code: "PLA", 
                description: "Пластик" 
            },
            _ => MaterialInfo { 
                code: "", 
                description: "материал" 
            },
        };

        // Обновляем состояние через редуктор
        state.dispatch(AppAction::SetMaterial(value.clone()));
        state.dispatch(AppAction::AddHistoryMessage(
            format!("Выбран материал: {}", material_info.description)
        ));

        // Проверяем, доступен ли Tauri API
        let is_tauri_available = web_sys::window()
            .and_then(|win| js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__")).ok())
            .map(|tauri| !tauri.is_undefined() && !tauri.is_null())
            .unwrap_or(false);
        
        if is_tauri_available {
            // Отправляем информацию о материале на бэкенд через Tauri
            let args = SetMaterialArgs {
                material_type: material_info.code.to_string(),
            };
            let state_clone = state.clone();
            
            spawn_local(async move {
                match serde_wasm_bindgen::to_value(&args) {
                    Ok(args_js) => {
                        let result = invoke("set_type_material", args_js).await;
                        if result.is_undefined() {
                            // Логируем ошибку в консоль
                            web_sys::console::error_1(
                                &format!("Ошибка при установке материала {} в backend", args.material_type).into()
                            );
                            // Отправляем сообщение об ошибке
                            state_clone.dispatch(AppAction::AddHistoryMessage(
                                format!("Ошибка: не удалось установить материал {} в системе", args.material_type)
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
                format!("Материал {} установлен (режим веб-приложения)", material_info.description)
            ));
        }
    })
}
