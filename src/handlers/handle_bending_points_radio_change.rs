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

pub fn handle_bending_points_radio_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        
        if value == "no" {
            // Обновляем состояние через dispatch
            state.dispatch(AppAction::SetBendingPoints {
                enabled: false,
                count: Some(0)
            });
            
            // Добавляем сообщения в историю
            state.dispatch(AppAction::AddHistoryMessage("Выбрано: Нет точек гиба".to_string()));
            state.dispatch(AppAction::AddHistoryMessage("Установлено количество точек гиба: 0".to_string()));
            
            // Отправляем данные на бэкенд
            let args = SetBendingPointsArgs {
                bending_points: 0,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_bending_points", args).await;
            });
        } else {
            // Обновляем состояние для выбора "Да"
            state.dispatch(AppAction::SetBendingPoints {
                enabled: true,
                count: None
            });
            
            // Добавляем сообщение в историю
            state.dispatch(AppAction::AddHistoryMessage(
                "Выбрано: Есть точки гиба. Укажите количество.".to_string()
            ));
        }
    })
}
