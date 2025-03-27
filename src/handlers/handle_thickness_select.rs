use web_sys::Event;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::{AppState, AppAction};

//=======================================================================================================================

// Обработчик выбора толщины

//=======================================================================================================================

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
            let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let value = select.value();
            
            // Преобразуем строковое значение в число
            if let Ok(thickness) = value.parse::<f32>() {
                // Обновляем значение в поле ввода
                state.dispatch(AppAction::SetThickness(thickness.to_string()));
                
                // Обновляем отфильтрованные толщины
                // Предполагаем, что у вас есть действие для обновления списка толщин
                // Используем вспомогательную функцию для обновления списка толщин
                update_filtered_thicknesses(&state, vec![thickness]);
                
                // Добавляем сообщение в историю
                let message = format!("Выбрана толщина: {} мм", thickness);
                state.dispatch(AppAction::AddHistoryMessage(message));
                
                // Обновляем цены
                calculate_and_update_prices(&state);
            }
        }
    })
}

// Вспомогательная функция для обновления списка толщин
fn update_filtered_thicknesses(state: &UseReducerHandle<AppState>, thicknesses: Vec<f32>) {
    // Здесь нужно использовать правильный вариант из вашего AppAction
    // Например:
    //state.dispatch(AppAction::UpdateFilteredThicknesses(thicknesses));
    // Замените на правильное имя действия из вашего AppAction
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
