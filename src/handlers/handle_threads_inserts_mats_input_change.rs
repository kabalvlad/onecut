use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use crate::bridge::set_threads_inserts_mats;
use wasm_bindgen_futures::spawn_local;
use crate::bridge::update_prices;

pub fn handle_threads_inserts_mats_input_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Если значение пустое, просто выходим
        if value.trim().is_empty() {
            return;
        }

        match value.parse::<i32>() {
            Ok(points) => {
                if points >= 0 {
                    // Выводим сообщение в консоль
                    web_sys::console::log_1(&format!(
                        "Установлено количество резьбы/вставок/цековок: {}", 
                        points
                    ).into());

                    // Создаем вектор с заданным количеством точек
                    let mats_vec = vec![1; points as usize];

                    spawn_local(async move {
                        match set_threads_inserts_mats(mats_vec.into()).await {
                            Ok(_) => {
                                web_sys::console::log_1(&"Успешно отправлено на бэкенд".into());
                            },
                            Err(e) => {
                                web_sys::console::error_1(&format!(
                                    "Ошибка при установке резьбы/вставок/цековок: {:?}", 
                                    e
                                ).into());
                            }
                        }
                    });
                } else {
                    // Отрицательное значение
                    web_sys::console::error_1(
                        &"Ошибка: количество резьбы/вставок/цековок не может быть отрицательным"
                        .into()
                    );
                }
            },
            Err(_) => {
                // Некорректное числовое значение
                web_sys::console::error_1(
                    &format!("Ошибка: '{}' не является корректным числовым значением", value)
                    .into()
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
