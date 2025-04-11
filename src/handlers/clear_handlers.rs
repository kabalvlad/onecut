use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::{set_quantity_parts, set_cost_material, set_margin_deal, update_prices};

// Обработчик для очистки полей расчета стоимости
pub fn handle_clear_options() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        // Выводим сообщение в консоль
        web_sys::console::log_1(
            &"Очистка полей ввода расчета стоимости".into()
        );
        
        // Сбрасываем поля через Tauri API
        
        // Сбрасываем количество деталей на 1
        spawn_local(async {
            match set_quantity_parts(1).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &"Количество деталей сброшено на 1".into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при сбросе количества деталей: {:?}", e).into()
                    );
                }
            }
        });
        
        // Сбрасываем цену материала на 0.0
        spawn_local(async {
            match set_cost_material(0.0).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &"Цена материала сброшена на 0.0".into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при сбросе цены материала: {:?}", e).into()
                    );
                }
            }
        });
        
        // Сбрасываем маржу на 28%
        spawn_local(async {
            match set_margin_deal(28.0).await {
                Ok(_) => {
                    web_sys::console::log_1(
                        &"Маржа сброшена на 28%".into()
                    );
                },
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Ошибка при сбросе маржи: {:?}", e).into()
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
