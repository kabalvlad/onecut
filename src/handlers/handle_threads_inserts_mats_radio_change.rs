use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use crate::tauri_api::set_threads_inserts_mats;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::update_prices;

pub fn handle_threads_inserts_mats_radio_change() -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        spawn_local(async move {
            if value == "no" {
                // Если выбрано "нет", отправляем пустой вектор (нет резьб/вставок/цековок)
                match set_threads_inserts_mats(vec![]).await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &"Резьбы/вставки/цековки отключены".into()
                        );
                        
                        // После успешной установки запускаем обновление цен
                        let _ = update_prices().await;
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!(
                            "Ошибка при установке резьб/вставок/цековок: {:?}", 
                            e
                        ).into());
                    }
                }
            } else {
                // Если выбрано "да", отправляем вектор с одним элементом
                // Это показывает, что резьбы/вставки/цековки включены, но количество пока не задано
                match set_threads_inserts_mats(vec![0]).await {
                    Ok(_) => {
                        web_sys::console::log_1(
                            &"Резьбы/вставки/цековки включены, ожидание ввода количества".into()
                        );
                        
                        // После успешной установки запускаем обновление цен
                        let _ = update_prices().await;
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!(
                            "Ошибка при установке резьб/вставок/цековок: {:?}", 
                            e
                        ).into());
                    }
                }
            }
        });
    })
}




