use web_sys::Event;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::str::FromStr;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::set_cut_length;
use crate::tauri_api::update_prices;

pub fn handle_cut_length_input() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        // Если значение пустое, просто выходим
        if value.trim().is_empty() {
            return;
        }
        
        // Нормализуем значение, заменяя запятые на точки
        let normalized_value = value.replace(',', ".");
        
        match f32::from_str(&normalized_value) {
            Ok(length) => {
                if length >= 0.0 {
                    // Выводим сообщение в консоль
                    web_sys::console::log_1(
                        &format!("Установлена длина реза: {:.2} мм", length).into()
                    );

                    // Отправляем данные на бэкенд через Tauri API
                    let length_clone = length;
                    
                    spawn_local(async move {
                        match set_cut_length(length_clone).await {
                            Ok(_) => {
                                web_sys::console::log_1(
                                    &"Длина реза успешно сохранена на сервере".into()
                                );
                            },
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("Ошибка при установке длины реза: {:?}", e).into()
                                );
                            }
                        }
                    });
                } else {
                    // Отрицательное значение
                    web_sys::console::error_1(
                        &"Ошибка: длина реза не может быть отрицательной".into()
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
        spawn_local(async {
            if let Err(e) = update_prices().await {
                web_sys::console::error_1(
                    &format!("Не удалось обновить цены: {:?}", e).into()
                );
            }
        });
    })
}
