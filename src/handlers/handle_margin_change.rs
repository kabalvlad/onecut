use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::models::{AppState, AppAction};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct MarginArgs {
    margin: f32,
}

pub fn handle_margin_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value_str = input.value();
        
        // Получаем введенное значение и проверяем его корректность
        let value = match value_str.parse::<i32>() {
            Ok(val) => {
                // Ограничиваем маржу от 0 до 100
                if val < 0 { 0 } else if val > 100 { 100 } else { val }
            },
            Err(_) => {
                // Если введено некорректное значение, используем текущее значение маржи
                input.set_value(&state.margin.to_string());
                state.dispatch(AppAction::AddHistoryMessage(
                    "Ошибка: введено некорректное значение маржи".to_string()
                ));
                return;
            }
        };
        
        // Обновляем значение в поле ввода
        input.set_value(&value.to_string());
        
        // Обновляем состояние
        state.dispatch(AppAction::SetMargin(value));
        
        // Добавляем сообщение в историю
        state.dispatch(AppAction::AddHistoryMessage(
            format!("Маржа {}% установлена", value)
        ));
        
        // Отправляем новое значение маржи на бэкенд
        let margin_value = value as f32;
        
        spawn_local(async move {
            let args = match serde_wasm_bindgen::to_value(&MarginArgs { margin: margin_value }) {
                Ok(args) => args,
                Err(_) => return, // Просто выходим в случае ошибки сериализации
            };
            
            // Просто вызываем бэкенд без дополнительных сообщений об успехе
            let _ = invoke("set_margin_deal", args).await;
        });
    })
}
