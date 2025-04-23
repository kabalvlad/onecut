use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::bridge::set_type_material;
use crate::bridge::update_prices;

// Структура для хранения информации о материалах
struct MaterialInfo {
    code: &'static str,
    description: &'static str,
}

// В файле с обработчиками
pub fn handle_material_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Получаем информацию о материале
        let material_info = match value.as_str() {
            "aluminum" => MaterialInfo { 
                code: "aluminum", 
                description: "Алюминий" 
            },
            "steel" => MaterialInfo { 
                code: "steel", 
                description: "Сталь" 
            },
            "stainless" => MaterialInfo { 
                code: "stainless", 
                description: "Нержавеющая сталь" 
            },
            "copper" => MaterialInfo { 
                code: "copper", 
                description: "Латунь/Бронза/Медь" 
            },
            "plastic" => MaterialInfo { 
                code: "plastic", 
                description: "Пластик" 
            },
            _ => MaterialInfo { 
                code: "", 
                description: "материал" 
            },
        };

        // Отправляем код материала на бэкенд
        let material_code = material_info.code.to_string();
        let material_desc = material_info.description.to_string();
        
        spawn_local(async move {
            match set_type_material(material_code.clone()).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &format!("Материал {} успешно установлен", material_desc).into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при установке материала {}: {:?}", material_desc, e).into()
                    );
                }
            }
        });
        
        // Обновляем цены
        spawn_local(async move {
            match update_prices().await {
                Ok(_) => {
                    web_sys::console::log_1(&"СООБЩЕНИЕ ИЗ ОБРАБОТЧИКА ЕГО НУЖНО ИГНОРИТЬ Цены успешно обновлены".into());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Ошибка обновления цен: {:?}", e).into());
                }
            }
        });
    })
}