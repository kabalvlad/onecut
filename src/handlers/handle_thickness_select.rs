use web_sys::Event;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::{AppState, AppAction};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetThicknessArgs {
    pub thickness: f32,
}

pub fn handle_thickness_select(
    state: UseReducerHandle<AppState>,    
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(target) = e.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let value = select.value();
                
                if let Ok(thickness) = value.parse::<f32>() {
                    // Устанавливаем толщину в состояние
                    state.dispatch(AppAction::SetThickness(thickness.to_string()));
                    
                    // Сообщаем об установке толщины
                    state.dispatch(AppAction::AddHistoryMessage(
                        format!("Установлена толщина: {} мм", thickness)
                    ));
                    
                    // Проверяем, доступен ли Tauri API
                    let is_tauri_available = web_sys::window()
                        .and_then(|win| js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__")).ok())
                        .map(|tauri| !tauri.is_undefined() && !tauri.is_null())
                        .unwrap_or(false);
                    
                    if is_tauri_available {
                        // Отправляем значение в backend только если Tauri доступен
                        let args = SetThicknessArgs { thickness };
                        let state_clone = state.clone();
                        
                        spawn_local(async move {
                            match serde_wasm_bindgen::to_value(&args) {
                                Ok(args_js) => {
                                    let result = invoke("set_thickness", args_js).await;
                                    if result.is_undefined() {
                                        // Логируем ошибку в консоль
                                        web_sys::console::error_1(
                                            &format!("Ошибка записи толщины {} мм в backend", thickness).into()
                                        );
                                        // Отправляем сообщение об ошибке
                                        state_clone.dispatch(AppAction::AddHistoryMessage(
                                            format!("Ошибка: не удалось установить толщину {} мм в системе", thickness)
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
                            format!("Толщина {} мм установлена (режим веб-приложения)", thickness)
                        ));
                    }
                }
            }
        }
    })
}
