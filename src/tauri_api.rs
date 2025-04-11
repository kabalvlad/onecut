use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use serde::{Serialize, Deserialize};

// Проверка доступности Tauri API
pub fn is_tauri_available() -> bool {
    web_sys::window()
        .and_then(|win| js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__")).ok())
        .and_then(|tauri| js_sys::Reflect::get(&tauri, &JsValue::from_str("tauri")).ok())
        .and_then(|tauri_mod| js_sys::Reflect::get(&tauri_mod, &JsValue::from_str("invoke")).ok())
        .map(|invoke_fn| !invoke_fn.is_undefined() && !invoke_fn.is_null())
        .unwrap_or(false)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn invoke(cmd: &str, args: JsValue) -> Promise;
}

pub async fn invoke_tauri_command<T: Serialize>(command: &str, args: Option<T>) -> Result<JsValue, JsValue> {
    // Проверяем доступность Tauri API
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, команда {} не будет выполнена", command).into());
        
        // Возвращаем пустое значение вместо ошибки
        return Ok(JsValue::NULL);
    }
    
    let args_value = match args {
        Some(args) => serde_wasm_bindgen::to_value(&args)?,
        None => JsValue::null(),
    };
    
    let promise = invoke(command, args_value);
    JsFuture::from(promise).await
}

// Функции для работы с типом резки
pub async fn set_type_cutting(cutting_type: String) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, тип резки {} не будет установлен", cutting_type).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_type_cutting", Some(&serde_json::json!({
        "cutting_type": cutting_type
    }))).await?;
    Ok(())
}


// Функции для работы с типом материала
pub async fn set_type_material(material_type: String) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, тип материала {} не будет установлен", material_type).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_type_material", Some(&serde_json::json!({
        "material_type": material_type
    }))).await?;
    Ok(())
}


// Функции для работы с толщиной
pub async fn set_thickness(thickness: f32) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, толщина {} не будет установлена", thickness).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_thickness", Some(&serde_json::json!({
        "thickness": thickness
    }))).await?;
    Ok(())
}



// Функции для работы с длиной реза
pub async fn set_cut_length(length: f32) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, длина реза {} не будет установлена", length).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_cut_length", Some(&serde_json::json!({
        "length": length
    }))).await?;
    Ok(())
}



// Функции для работы с точками сгиба
pub async fn set_bending_points(points: Vec<i32>) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, точки сгиба {:?} не будут установлены", points).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_bending_points", Some(&serde_json::json!({
        "points": points
    }))).await?;
    Ok(())
}


// Функции для работы с резьбами/вставками/цековками
pub async fn set_threads_inserts_mats(mats: Vec<i32>) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, резьбы/вставки/цековки {:?} не будут установлены", mats).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_threads_inserts_mats", Some(&serde_json::json!({
        "mats": mats
    }))).await?;
    Ok(())
}



// Функции для работы с количеством деталей
pub async fn set_quantity_parts(quantity: i32) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, количество деталей {} не будет установлено", quantity).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_quantity_parts", Some(&serde_json::json!({
        "quantity": quantity
    }))).await?;
    Ok(())
}

pub async fn get_quantity_parts() -> Result<i32, JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, количество деталей не будет получено".into());
        return Ok(1);
    }
    
    let result = invoke_tauri_command::<()>("get_quantity_parts", None).await?;
    let quantity = result.as_f64().unwrap_or_default() as i32;
    Ok(quantity)
}

// Функции для работы с ценой материала
pub async fn set_cost_material(cost: f32) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, цена материала {} не будет установлена", cost).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_cost_material", Some(&serde_json::json!({
        "cost": cost
    }))).await?;
    Ok(())
}

pub async fn get_cost_material() -> Result<f32, JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, цена материала не будет получена".into());
        return Ok(0.0);
    }
    
    let result = invoke_tauri_command::<()>("get_cost_material", None).await?;
    let cost = result.as_f64().unwrap_or_default() as f32;
    Ok(cost)
}

// Функции для работы с маржой
pub async fn set_margin_deal(margin: f32) -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&format!("Tauri API недоступен, маржа {} не будет установлена", margin).into());
        return Ok(());
    }
    
    invoke_tauri_command("set_margin_deal", Some(&serde_json::json!({
        "margin": margin
    }))).await?;
    Ok(())
}


// Функции для работы с ценой одной детали
pub async fn get_price_one_part() -> Result<f32, JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, цена за деталь не будет получена".into());
        return Ok(0.0);
    }
    
    let result = invoke_tauri_command::<()>("get_price_one_part", None).await?;
    let price = result.as_f64().unwrap_or_default() as f32;
    Ok(price)
}

// Функции для работы с ценой всех деталей
pub async fn get_price_all_parts() -> Result<f32, JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, общая цена не будет получена".into());
        return Ok(0.0);
    }
    
    let result = invoke_tauri_command::<()>("get_price_all_parts", None).await?;
    let price = result.as_f64().unwrap_or_default() as f32;
    Ok(price)
}

// Функция-обертка для вызова команды calculate_cutting_price_command
pub async fn calculate_cutting_price() -> Result<(), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, расчет цены не будет выполнен".into());
        return Ok(());
    }
    
    invoke_tauri_command("calculate_cutting_price_command", None::<()>).await?;
    Ok(())
}

// Функция для отправки события обновления цен
pub fn dispatch_price_update_event(price_one: f32, price_all: f32) {
    // Создаем объект с данными для события
    let detail = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &detail,
        &"pricePerPart".into(),
        &price_one.into()
    );
    let _ = js_sys::Reflect::set(
        &detail,
        &"priceTotal".into(),
        &price_all.into()
    );
    
    // Создаем параметры для события
    let event_init = web_sys::CustomEventInit::new();
    event_init.set_detail(&detail);
    event_init.set_bubbles(true);
    
    // Создаем и отправляем событие
    if let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(
        "price-update",
        &event_init
    ) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.dispatch_event(&event);
            }
        }
    }
}

// Функция для обновления цен
pub async fn update_prices() -> Result<(f32, f32), JsValue> {
    if !is_tauri_available() {
        web_sys::console::log_1(&"Tauri API недоступен, обновление цен не будет выполнено".into());
        // Возвращаем фиктивные значения для тестирования
        return Ok((0.0, 0.0));
    }
    
    match calculate_cutting_price().await {
        Ok(_) => {
            // Вызываем функции для получения цен
            let price_one = match get_price_one_part().await {
                Ok(price) => {
                    web_sys::console::log_1(
                        &format!("Новая цена за одну деталь: {:.2} €", price).into()
                    );
                    price
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при получении цены за деталь: {:?}", e).into()
                    );
                    return Err(e);
                }
            };
            
            let price_all = match get_price_all_parts().await {
                Ok(price) => {
                    web_sys::console::log_1(
                        &format!("Новая общая цена: {:.2} €", price).into()
                    );
                    price
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при получении общей цены: {:?}", e).into()
                    );
                    return Err(e);
                }
            };
            
            // Отправляем событие с новыми ценами
            dispatch_price_update_event(price_one, price_all);
            
            Ok((price_one, price_all))
        },
        Err(e) => {
            web_sys::console::error_1(
                &format!("Ошибка при расчете цены: {:?}", e).into()
            );
            Err(e)
        }
    }
}
