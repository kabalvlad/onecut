use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Функция для получения цены за все детали из бэкенда
pub async fn get_price_all_parts() -> Result<f32, String> {
    let result = invoke("get_price_all_parts", JsValue::NULL).await;
    
    // Преобразуем результат в f32
    match serde_wasm_bindgen::from_value::<f32>(result) {
        Ok(price) => Ok(price),
        Err(_) => Err("Не удалось получить общую цену".to_string())
    }
}

// Обработчик для получения цены за все детали
pub fn handle_get_price_all_parts(price_total: UseStateHandle<f32>) {
    let price_total_clone = price_total.clone();
    
    spawn_local(async move {
        if let Ok(price) = get_price_all_parts().await {
            price_total_clone.set(price);
        }
    });
}
