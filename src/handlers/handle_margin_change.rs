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
        e.prevent_default();
        
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
        
        // Отправляем новое значение маржи на бэкенд
        let margin_value = value as f32;
        let state_clone = state.clone();
        
        spawn_local(async move {
            let args = MarginArgs { margin: margin_value };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            
            match invoke("set_margin_deal", args).await {
                result if !result.is_undefined() => {
                    state_clone.dispatch(AppAction::SetMargin(value));
                    state_clone.dispatch(AppAction::AddHistoryMessage(
                        format!("Маржа {}% установлена", margin_value)
                    ));
                    
                    // Пересчитываем цены
                    // Предполагается, что у вас есть соответствующие действия для обновления цен
                    state_clone.dispatch(AppAction::UpdatePrices {
                        price_per_part: calculate_price_per_part(&state_clone),
                        price_total: calculate_total_price(&state_clone),
                    });
                },
                _ => {
                    state_clone.dispatch(AppAction::AddHistoryMessage(
                        format!("Ошибка при установке маржи {}%", margin_value)
                    ));
                }
            }
        });
    })
}

// Вспомогательные функции для расчета цен
fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь ваша логика расчета цены за одну деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState) -> f32 {
    // Здесь ваша логика расчета общей цены
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}
