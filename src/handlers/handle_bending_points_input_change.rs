use yew::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::{AppState, AppAction};

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
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        if let Ok(points) = value.parse::<i32>() {
            // Обновляем состояние через dispatch
            state.dispatch(AppAction::SetBendingPoints {
                enabled: true,
                count: Some(points)
            });
            
            // Создаем сообщение об успешном вводе
            let message = format!("Установлено количество точек гиба: {}", points);
            state.dispatch(AppAction::AddHistoryMessage(message));

            // Отправляем данные на бэкенд
            let args = SetBendingPointsArgs {
                bending_points: points,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_bending_points", args).await;
            });
            
            // Обновляем цены через существующее действие
            state.dispatch(AppAction::UpdatePrices {
                price_per_part: 0.0,
                price_total: 0.0
            });
        } else {
            // Обработка ошибки
            state.dispatch(AppAction::AddHistoryMessage(
                "Ошибка: введите корректное целое число для количества точек гиба".to_string()
            ));
        }
    })
}
