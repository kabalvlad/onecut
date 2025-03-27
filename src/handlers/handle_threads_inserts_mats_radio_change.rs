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
            state.dispatch(AppAction::AddHistoryMessage(
                "Установлено количество резьбы/вставок/цековок: 0".to_string()
            ));
            
            // Устанавливаем значение поля ввода в 0
            state.dispatch(AppAction::SetThreadsInsertsMats { 
                enabled: false, 
                count: Some(0) 
            });
            
            // Отправляем данные на бэкенд
            let args = SetThreadsInsertsMatsArgs {
                threads_inserts_mats: 0,
            };

            let state_clone = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&args).unwrap();
                let _ = invoke("set_threads_inserts_mats", args).await;
                
                // После отправки данных на бэкенд, обновляем цены
                calculate_and_update_prices(&state_clone);
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
            
            // Обновляем цены
            calculate_and_update_prices(&state);
        }
    })
}

// Вспомогательная функция для расчета и обновления цен
fn calculate_and_update_prices(state: &UseReducerHandle<AppState>) {
    // Расчет цены за одну деталь
    let price_per_part = calculate_price_per_part(&state);
    
    // Расчет общей цены
    let price_total = calculate_total_price(&state, price_per_part);
    
    // Обновление цен в состоянии
    state.dispatch(AppAction::UpdatePrices {
        price_per_part,
        price_total,
    });
}

fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета цены за деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState, price_per_part: f32) -> f32 {
    // Здесь должна быть ваша логика расчета общей цены
    price_per_part * state.parts_count as f32
}
