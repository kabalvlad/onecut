use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::{set_quantity_parts, get_quantity_parts, update_prices};

pub fn handle_parts_count_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value();
        
        // Проверяем, что введенное значение - корректное целое число
        match input_value.parse::<i32>() {
            Ok(value) => {
                // Убедимся, что количество деталей не меньше 1
                let value = if value < 1 { 1 } else { value };
                
                // Обновляем поле ввода с корректным значением
                input.set_value(&value.to_string());
                
                // Выводим сообщение в консоль
                web_sys::console::log_1(
                    &format!("Установлено количество деталей: {}", value).into()
                );
                
                // Отправляем значение в backend через Tauri API
                spawn_local(async move {
                    match set_quantity_parts(value).await {
                        Ok(_) => {
                            web_sys::console::log_1(
                                &format!("Количество деталей {} успешно установлено в системе", value).into()
                            );
                        },
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Ошибка установки количества деталей {}: {:?}", value, e).into()
                            );
                        }
                    }
                });
            },
            Err(_) => {
                // Выводим сообщение об ошибке в консоль
                web_sys::console::error_1(
                    &"Ошибка: введено некорректное количество деталей".into()
                );
                
                // Получаем последнее корректное значение из бэкенда и обновляем поле ввода
                spawn_local(async move {
                    match get_quantity_parts().await {
                        Ok(count) => {
                            input.set_value(&count.to_string());
                        },
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Ошибка получения количества деталей: {:?}", e).into()
                            );
                            // Устанавливаем значение по умолчанию
                            input.set_value("1");
                        }
                    }
                });
            }
        }
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
