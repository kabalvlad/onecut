use web_sys::Event;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri_api::set_thickness;
use crate::tauri_api::update_prices;

pub fn handle_thickness_select() -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(target) = e.target() {
            if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                let value = select.value();
                
                if let Ok(thickness) = value.parse::<f32>() {
                    // Выводим сообщение в консоль
                    web_sys::console::log_1(
                        &format!("Установлена толщина: {} мм", thickness).into()
                    );
                    
                    // Отправляем значение в backend через Tauri API
                    spawn_local(async move {
                        match set_thickness(thickness).await {
                            Ok(_) => {
                                web_sys::console::log_1(
                                    &format!("Толщина {} мм успешно установлена в системе", thickness).into()
                                );
                            },
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("Ошибка установки толщины {} мм: {:?}", thickness, e).into()
                                );
                            }
                        }
                    });
                } else {
                    web_sys::console::error_1(
                        &format!("Ошибка парсинга значения толщины: {}", value).into()
                    );
                }
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
