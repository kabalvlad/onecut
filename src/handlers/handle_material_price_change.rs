use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::{set_cost_material, get_cost_material, update_prices};

pub fn handle_material_price_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut input_value = input.value();
        
        // Заменяем запятую на точку для корректного парсинга
        input_value = input_value.replace(',', ".");
        
        // Проверяем, что введенное значение - корректное число с плавающей точкой
        let parse_result = if input_value.trim().is_empty() {
            Ok(0.0)
        } else {
            input_value.parse::<f32>()
        };
        
        match parse_result {
            Ok(value) => {
                // Убедимся, что цена не отрицательная
                let value = if value < 0.0 { 0.0 } else { value };
                
                // Обновляем поле ввода с форматированным значением
                input.set_value(&format!("{:.2}", value));
                
                // Выводим сообщение в консоль
                web_sys::console::log_1(
                    &format!("Установлена цена материала: {:.2} €", value).into()
                );
                
                // Отправляем значение в backend через Tauri API
                spawn_local(async move {
                    match set_cost_material(value).await {
                        Ok(_) => {
                            web_sys::console::log_1(
                                &format!("Цена материала {:.2} € успешно установлена в системе", value).into()
                            );
                        },
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Ошибка установки цены материала {:.2} €: {:?}", value, e).into()
                            );
                        }
                    }
                });
            },
            Err(_) => {
                // Выводим сообщение об ошибке в консоль
                web_sys::console::error_1(
                    &"Ошибка: введено некорректное значение цены материала".into()
                );
                
                // Получаем последнее корректное значение из бэкенда и обновляем поле ввода
                spawn_local(async move {
                    match get_cost_material().await {
                        Ok(price) => {
                            input.set_value(&format!("{:.2}", price));
                        },
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Ошибка получения цены материала: {:?}", e).into()
                            );
                            // Устанавливаем значение по умолчанию
                            input.set_value("0.00");
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
