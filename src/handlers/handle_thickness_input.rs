use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use std::str::FromStr;
use wasm_bindgen_futures::spawn_local;
use crate::bridge::set_thickness;
use crate::bridge::update_prices;

pub fn handle_thickness_input(
    all_thicknesses: Vec<f32>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        // Если строка пустая, просто выходим
        if value.trim().is_empty() {
            return;
        }

        // Заменяем запятую на точку для корректного парсинга
        let normalized_value = value.replace(',', ".");
        
        // Проверяем, содержит ли строка только цифры, точку или запятую
        if !normalized_value.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            web_sys::console::error_1(
                &"Ошибка: Пожалуйста, введите только числовое значение толщины без посторонних символов".into()
            );
            return;
        }

        // Пытаемся преобразовать строку в число
        match f32::from_str(&normalized_value) {
            Ok(input_num) => {
                if input_num <= 0.0 {
                    web_sys::console::error_1(
                        &"Ошибка: Толщина должна быть положительным числом".into()
                    );
                    return;
                }

                // Находим ближайшее значение из списка
                let nearest = all_thicknesses
                    .iter()
                    .min_by(|&&a, &b| {
                        (a - input_num).abs()
                            .partial_cmp(&(b - input_num).abs())
                            .unwrap()
                    })
                    .copied()
                    .unwrap_or(all_thicknesses[0]);
                
                // Обновляем значение в поле ввода
                input.set_value(&nearest.to_string());
                
                // Выводим сообщение в консоль
                web_sys::console::log_1(
                    &format!("Выбрана ближайшая доступная толщина: {} мм", nearest).into()
                );

                // Отправляем значение в backend через Tauri API
                spawn_local(async move {
                    match set_thickness(nearest).await {
                        Ok(_) => {
                            web_sys::console::log_1(
                                &format!("Толщина {} мм успешно установлена в системе", nearest).into()
                            );
                        },
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Ошибка установки толщины {} мм: {:?}", nearest, e).into()
                            );
                        }
                    }
                });
            },
            Err(_) => {
                web_sys::console::error_1(
                    &"БОРОДА что ТЫ ДЕЛАЕШЬ ??? : дай нормальне число ".into()
                );
            }
        }
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
