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

pub fn handle_threads_inserts_mats_radio_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Обновляем историю и состояние в зависимости от выбора
        if value == "no" {
            // Добавляем сообщения в историю
            state.dispatch(AppAction::AddHistoryMessage(
                "Выбрано: Нет резьбы, вставок и цековок".to_string()
            ));
            
            // Устанавливаем значение поля ввода в 0
            state.dispatch(AppAction::SetThreadsInsertsMats { 
                enabled: false, 
                count: Some(0) 
            });
            
            // Добавляем сообщение о количестве
            state.dispatch(AppAction::AddHistoryMessage(
                "Установлено количество резьбы/вставок/цековок: 0".to_string()
            ));
            
            // Отправляем данные на бэкенд
            let args = SetThreadsInsertsMatsArgs {
                threads_inserts_mats: 0,
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
            // Обновляем историю для выбора "Да"
            state.dispatch(AppAction::AddHistoryMessage(
                "Выбрано: Есть резьба, вставки или цековки. Укажите количество.".to_string()
            ));
            
            // Очищаем поле ввода
            state.dispatch(AppAction::SetThreadsInsertsMats { 
                enabled: true, 
                count: None 
            });
        }
    })
}
