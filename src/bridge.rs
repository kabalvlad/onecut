use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setTypeMaterial", catch)]
    pub async fn set_type_material(material_type: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getTypeMaterial", catch)]
    pub async fn get_type_material() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setTypeCutting", catch)]
    pub async fn set_type_cutting(cutting_type: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getTypeCutting", catch)]
    pub async fn get_type_cutting() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setThickness", catch)]
    pub async fn set_thickness(thickness: f32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getThickness", catch)]
    pub async fn get_thickness() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setCutLength", catch)]
    pub async fn set_cut_length(length: f32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getCutLength", catch)]
    pub async fn get_cut_length() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setBendingPoints", catch)]
    pub async fn set_bending_points(points: Box<[i32]>) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getBendingPoints", catch)]
    pub async fn get_bending_points() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setThreadsInsertsMats", catch)]
    pub async fn set_threads_inserts_mats(mats: Box<[i32]>) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getThreadsInsertsMats", catch)]
    pub async fn get_threads_inserts_mats() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setQuantityParts", catch)]
    pub async fn set_quantity_parts(quantity: i32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getQuantityParts", catch)]
    pub async fn get_quantity_parts() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setCostMaterial", catch)]
    pub async fn set_cost_material(cost: f32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getCostMaterial", catch)]
    pub async fn get_cost_material() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "setMarginDeal", catch)]
    pub async fn set_margin_deal(margin: f32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getMarginDeal", catch)]
    pub async fn get_margin_deal() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getPriceOnePart", catch)]
    pub async fn get_price_one_part() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "getPriceAllParts", catch)]
    pub async fn get_price_all_parts() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["window", "bridge"], js_name = "calculateCuttingPrice", catch)]
    pub async fn calculate_cutting_price() -> Result<JsValue, JsValue>;
    
}

#[wasm_bindgen(js_name = "updatePrices")]
pub async fn update_prices() -> Result<JsValue, JsValue> {
    use web_sys::console;
    
    console::log_1(&"Начало update_prices".into());
    
    // Вызываем функции напрямую, без использования this
    // Сначала вызываем расчет цены
    match calculate_cutting_price().await {
        Ok(_) => console::log_1(&"Расчет цены успешен".into()),
        Err(e) => {
            console::error_1(&format!("Ошибка при расчете цены: {:?}", e).into());
            return Err(e);
        }
    }
    
    // Получаем цену одной детали
    let price_one_part = match get_price_one_part().await {
        Ok(price) => {
            console::log_1(&format!("Цена одной детали: {:?}", price).into());
            price
        },
        Err(e) => {
            console::error_1(&format!("Ошибка при получении цены одной детали: {:?}", e).into());
            return Err(e);
        }
    };
    
    // Получаем цену всех деталей
    let price_all_parts = match get_price_all_parts().await {
        Ok(price) => {
            console::log_1(&format!("Цена всех деталей: {:?}", price).into());
            price
        },
        Err(e) => {
            console::error_1(&format!("Ошибка при получении цены всех деталей: {:?}", e).into());
            return Err(e);
        }
    };
    
    // Создаем объект для возврата результатов
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"priceOnePart".into(), &price_one_part)?;
    js_sys::Reflect::set(&result, &"priceAllParts".into(), &price_all_parts)?;
    
    console::log_1(&format!("Возвращаем результат: {:?}", result).into());
    
    Ok(result.into())
}



