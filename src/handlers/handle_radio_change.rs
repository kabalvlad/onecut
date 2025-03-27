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
        
        // Обновляем выбранный тип резки через редуктор
        state.dispatch(AppAction::SetCuttingType(value.clone()));
        
        let cutting_code = match value.as_str() {
            "laser" => "LC",
            "plasma" => "PC",
            "hydro" => "HC",
            _ => "",
        };
        
        let args = SetTypeCuttingArgs {
            cutting_type: cutting_code.to_string(),
        };
        
        let state_clone = state.clone();
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            match invoke("set_type_cutting", args).await {
                result if !result.is_undefined() => {
                    // Добавляем сообщение в историю
                    let message = match value.as_str() {
                        "laser" => "Выбрана лазерная резка",
                        "plasma" => "Выбрана плазменная резка",
                        "hydro" => "Выбрана гидроабразивная резка",
                        _ => "Выберите тип резки",
                    }.to_string();
                    
                    state_clone.dispatch(AppAction::AddHistoryMessage(message));
                    
                    // Пересчитываем цены
                    let price_per_part = calculate_price_per_part(&state_clone);
                    let price_total = calculate_total_price(&state_clone);
                    
                    state_clone.dispatch(AppAction::UpdatePrices {
                        price_per_part,
                        price_total,
                    });
                },
                _ => {
                    state_clone.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка при установке типа резки".to_string()
                    ));
                }
            }
        });
    })
}

// Вспомогательные функции для расчета цен
fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета цены за деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета общей цены
    // Обычно это price_per_part * parts_count
    state.price_per_part * state.parts_count as f32
}
