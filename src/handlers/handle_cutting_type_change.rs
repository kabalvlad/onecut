use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::set_type_cutting;
use crate::tauri_api::update_prices;

//===============================================================
// Обработчик для выбора типа резки
//===============================================================

pub fn handle_cutting_type_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Получаем код и описание типа резки
        let (cutting_code, cutting_description) = match value.as_str() {
            "laser" => ("laser", "Лазерная резка"),
            "plasma" => ("plasma", "Плазменная резка"),
            "hydro" => ("hydro", "Гидроабразивная резка"),
            _ => ("", "Неизвестный тип резки"),
        };
        
        // Выводим сообщение в консоль
        web_sys::console::log_1(
            &format!("Выбран тип резки: {}", cutting_description).into()
        );
        
        // Отправляем информацию о типе резки на бэкенд через Tauri API
        let cutting_code_clone = cutting_code.to_string();
        let cutting_desc_clone = cutting_description.to_string();
        
        spawn_local(async move {
            match set_type_cutting(cutting_code_clone.clone()).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &format!("Тип резки {} успешно установлен в системе", cutting_desc_clone).into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при установке типа резки {}: {:?}", cutting_desc_clone, e).into()
                    );
                }
            }
        });
        // В конце обработчика
        spawn_local(async {
            if let Err(e) = update_prices().await {
                web_sys::console::error_1(
                    &format!("Не удалось обновить цены: {:?}", e).into()
                );
            }
        });
    })
}
