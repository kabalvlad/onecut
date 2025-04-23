use yew::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::spawn_local;
use js_sys::ArrayBuffer;
use crate::models::FileData;
use crate::dxf::dxf_processor::calculate_dxf_length;
use crate::bridge::set_cut_length;
use crate::bridge::update_prices;

pub fn handle_file_select() -> Callback<Event> {
    Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let file_name = file.name();

                if !file_name.to_lowercase().ends_with(".dxf") {
                    web_sys::console::error_1(
                        &"Неверный формат файла. Пожалуйста, загрузите файл формата - DXF/dxf".into()
                    );
                    return;
                }

                web_sys::console::log_1(&"Формат файла верный".into());

                spawn_local(async move {
                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();

                    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(buffer) = reader_clone.result() {
                            if let Ok(array_buffer) = buffer.dyn_into::<ArrayBuffer>() {
                                let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
                            
                                match String::from_utf8(bytes) {
                                    Ok(content) => {
                                        let file_data_obj = FileData {
                                            name: file_name.clone(),
                                            content: content.clone(),
                                        };

                                        // Передаем объект в функцию расчета длины
                                        match calculate_dxf_length(file_data_obj) {
                                            Ok(result) => {
                                                web_sys::console::log_1(
                                                    &format!("Общая длина всех линий: {:.2} мм", result).into()
                                                );

                                                // Отправляем длину реза в бэкенд через Tauri API
                                                let cut_length = result as f32;
                                                spawn_local(async move {
                                                    match set_cut_length(cut_length).await {
                                                        Ok(_) => {
                                                            web_sys::console::log_1(
                                                                &format!("Длина реза установлена: {:.2} мм", cut_length).into()
                                                            );
                                                        },
                                                        Err(e) => {
                                                            web_sys::console::error_1(
                                                                &format!("Ошибка при установке длины реза: {:?}", e).into()
                                                            );
                                                        }
                                                    }
                                                });
                                            },
                                            Err(err) => {
                                                web_sys::console::error_1(
                                                    &format!("Ошибка при расчете длины: {}", err).into()
                                                );
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        web_sys::console::error_1(
                                            &"Ошибка при чтении содержимого файла".into()
                                        );
                                    }
                                }
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    reader.read_as_array_buffer(&file).unwrap();
                });
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
