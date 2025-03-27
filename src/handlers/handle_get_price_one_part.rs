use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;


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
pub fn handle_get_price_one_part(price_per_part: UseStateHandle<f32>) {
    spawn_local(async move {
        if let Ok(price) = get_price_one_part().await {
            price_per_part.set(price);
        }
    });
}

