use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::bridge::set_thickness;
use crate::bridge::update_prices;

//=======================================================================================================================

// Обработчик очистки толщины

//=======================================================================================================================

pub fn handle_clear_thickness() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Выводим сообщение в консоль
        web_sys::console::log_1(
            &"Очистка значения толщины".into()
        );
        
        // Отправляем нулевое значение толщины на бэкенд через Tauri API
        spawn_local(async move {
            match set_thickness(0.0).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &"Значение толщины успешно сброшено".into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при сбросе значения толщины: {:?}", e).into()
                    );
                }
            }
        });
        // В конце обработчика
        spawn_local(async move {
            match update_prices().await {
                Ok(_) => {
                    web_sys::console::log_1(&"Цены успешно обновлены".into());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Ошибка обновления цен: {:?}", e).into());
                }
            }
        });
    })
}
