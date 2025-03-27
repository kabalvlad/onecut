use yew::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::spawn_local;
use js_sys::ArrayBuffer;
use crate::models::{FileData, AppState, AppAction};
use serde::{Serialize, Deserialize};
use crate::dxf::dxf_processor::calculate_dxf_length;

#[derive(Debug, Serialize, Deserialize)]
pub struct DxfCalculationResult {
    pub total_length: f64,
    pub error: Option<String>,
}

pub fn handle_file_select(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let file_name = file.name();

                if !file_name.to_lowercase().ends_with(".dxf") {
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Неверный формат файла. Пожалуйста, загрузите файл формата - DXF/dxf".to_string()
                    ));
                    return;
                }

                state.dispatch(AppAction::AddHistoryMessage("Формат файла верный".to_string()));

                let state = state.clone();
                spawn_local(async move {
                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();

                    let onload = {
                        let state = state.clone();
                        let file_name = file_name.clone();
                        
                        Closure::wrap(Box::new(move |_: web_sys::Event| {
                            if let Ok(buffer) = reader_clone.result() {
                                if let Ok(array_buffer) = buffer.dyn_into::<ArrayBuffer>() {
                                    let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
                                
                                    match String::from_utf8(bytes) {
                                        Ok(content) => {
                                            let file_data_obj = FileData {
                                                name: file_name.clone(),
                                                content: content.clone(),
                                            };

                                            // Просто передаем объект в функцию расчета длины
                                            match calculate_dxf_length(file_data_obj) {
                                                Ok(result) => {
                                                    state.dispatch(AppAction::AddHistoryMessage(
                                                        format!("Общая длина всех линий: {:.2} мм", result)
                                                    ));

                                                    state.dispatch(AppAction::SetCutLength(result as f32));

                                                    state.dispatch(AppAction::AddHistoryMessage(
                                                        format!("Длина реза установлена: {:.2} мм", result)
                                                    ));
                                                },
                                                Err(err) => {
                                                    state.dispatch(AppAction::AddHistoryMessage(
                                                        format!("Ошибка при расчете длины: {}", err)
                                                    ));
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            state.dispatch(AppAction::AddHistoryMessage(
                                                "Ошибка при чтении содержимого файла".to_string()
                                            ));
                                        }
                                    }
                                }
                            }
                        }) as Box<dyn FnMut(_)>)
                    };

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    reader.read_as_array_buffer(&file).unwrap();
                });
            }
        }
    })
}
