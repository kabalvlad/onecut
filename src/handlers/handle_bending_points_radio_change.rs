use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen_futures::spawn_local;
use crate::bridge::set_bending_points;
use crate::bridge::update_prices;

pub fn handle_bending_points_radio_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();

        if value == "no" {
            // Выводим сообщения в консоль
            web_sys::console::log_1(&"Выбрано: Нет точек гиба".into());
            web_sys::console::log_1(&"Установлено количество точек гиба: 0".into());
            
            // Отправляем пустой вектор на бэкенд через Tauri API
            spawn_local(async {
                // Передаем пустой вектор, что означает отсутствие точек гиба
                match set_bending_points(vec![].into()).await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &"Настройки точек гиба успешно сохранены на сервере".into()
                        );
                        
                        // После успешной установки запускаем обновление цен
                        let _ = update_prices().await;
                    },
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Ошибка при установке точек гиба: {:?}", e).into()
                        );
                    }
                }
            });
        } else {
            // Выводим сообщение в консоль для выбора "Да"
            web_sys::console::log_1(
                &"Выбрано: Есть точки гиба. Укажите количество.".into()
            );
            
            // Для варианта "Да" мы отправляем вектор с одним элементом,
            // показывая, что точки гиба включены, но количество пока не задано
            spawn_local(async {
                match set_bending_points(vec![0].into()).await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &"Точки гиба включены, ожидание ввода количества".into()
                        );
                        
                        // После успешной установки запускаем обновление цен
                        let _ = update_prices().await;
                    },
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Ошибка при включении точек гиба: {:?}", e).into()
                        );
                    }
                }
            });
        }

    })
}
