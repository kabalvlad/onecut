use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::set_margin_deal;
use crate::tauri_api::update_prices;

pub fn handle_margin_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value_str = input.value();
        
        // Получаем введенное значение и проверяем его корректность
        let value = match value_str.parse::<i32>() {
            Ok(val) => {
                // Ограничиваем маржу от 0 до 100
                if val < 0 { 0 } else if val > 100 { 100 } else { val }
            },
            Err(_) => {
                // Если введено некорректное значение, выводим сообщение об ошибке
                web_sys::console::error_1(
                    &"Ошибка: введено некорректное значение маржи".into()
                );
                return;
            }
        };
        
        // Обновляем значение в поле ввода
        input.set_value(&value.to_string());
        
        // Выводим сообщение в консоль
        web_sys::console::log_1(
            &format!("Маржа {}% установлена", value).into()
        );
        
        // Отправляем новое значение маржи на бэкенд через Tauri API
        let margin_value = value as f32;
        
        spawn_local(async move {
            match set_margin_deal(margin_value).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &format!("Маржа {}% успешно установлена в системе", value).into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при установке маржи {}%: {:?}", value, e).into()
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
