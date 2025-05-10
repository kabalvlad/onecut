use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use std::str::FromStr;
use wasm_bindgen_futures::spawn_local;
use crate::bridge::set_bending_points;
use crate::bridge::update_prices;

pub fn handle_bending_points_input_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        // Если значение пустое, просто выходим
        if value.trim().is_empty() {
            return;
        }
        
        // Нормализуем значение, заменяя запятые на точки
        let normalized_value = value.replace(',', ".");
        
        match i32::from_str(&normalized_value) {
            Ok(points) => {
                if points >= 0 {
                    // Выводим сообщение в консоль
                    web_sys::console::log_1(
                        &format!("Установлено количество точек сгиба: {}", points).into()
                    );

                    // Создаем вектор с указанным количеством точек
                    let points_vec = vec![1; points as usize];

                    // Отправляем данные на бэкенд через Tauri API
                    spawn_local(async move {
                        match set_bending_points(points_vec.into()).await {
                            Ok(_) => {
                                web_sys::console::log_1(
                                    &format!("Количество точек сгиба ({}) успешно сохранено", points).into()
                                );
                            },
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("Ошибка при установке точек сгиба: {:?}", e).into()
                                );
                            }
                        }
                    });
                } else {
                    // Отрицательное значение
                    web_sys::console::error_1(
                        &"Ошибка: количество точек сгиба не может быть отрицательным".into()
                    );
                }
            },
            Err(_) => {
                // Некорректное числовое значение
                web_sys::console::error_1(
                    &format!("Ошибка: '{}' не является корректным числовым значением", value).into()
                );
            }
        }
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
