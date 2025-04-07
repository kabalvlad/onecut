use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::models::{AppState, AppAction};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Функция для получения цены за одну деталь из бэкенда
pub async fn get_price_one_part() -> Result<f32, String> {
    let result = invoke("get_price_one_part", JsValue::NULL).await;
    
    // Преобразуем результат в f32
    match serde_wasm_bindgen::from_value::<f32>(result) {
        Ok(price) => Ok(price),
        Err(_) => Err("Не удалось получить цену за деталь".to_string())
    }
}

/// Обработчик для получения цены за одну деталь
pub fn handle_get_price_one_part(state: UseReducerHandle<AppState>) -> Callback<()> {
    Callback::from(move |_| {
        let state_clone = state.clone();
        
        spawn_local(async move {
            if let Ok(price) = get_price_one_part().await {
                // Обновляем только цену за одну деталь в состоянии
                state_clone.dispatch(AppAction::UpdatePrices {
                    price_per_part: price, // Обновляем цену за деталь
                    price_total: state_clone.price_total, // Сохраняем текущую общую цену
                });
            }
        });
    })
}
