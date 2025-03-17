use yew::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::spawn_local;
use js_sys::ArrayBuffer;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use crate::models::FileData;
use serde::{Serialize, Deserialize};
use crate::dxf::dxf_processor::calculate_dxf_length;

#[derive(Debug, Serialize, Deserialize)]
pub struct DxfCalculationResult {
    pub total_length: f64,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SetCutLengthArgs {
    cut_length: f32,
}



// Изменяем функцию add_message для работы с Rc<RefCell>
fn add_message(history: &Rc<RefCell<VecDeque<String>>>, message: String) {
    let mut history = history.borrow_mut();
    if history.len() >= 30 {
        history.pop_back();
    }
    history.push_front(message);
}

pub fn handle_file_select(
    cut_length: UseStateHandle<f32>,
    history: UseStateHandle<VecDeque<String>>,
    file_data: UseStateHandle<Option<FileData>>
) -> Callback<Event> {
    let history_rc = Rc::new(RefCell::new((*history).clone()));
    let history_state = history.clone();
    let cut_length = cut_length.clone();

    Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let file_data = file_data.clone();
        let history_rc = history_rc.clone();
        let history_state = history_state.clone();
        let cut_length = cut_length.clone();

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let file_name = file.name();

                if !file_name.to_lowercase().ends_with(".dxf") {
                    add_message(
                        &history_rc,
                        "Неверный формат файла. Пожалуйста, загрузите файл формата - DXF/dxf".to_string()
                    );
                    history_state.set(history_rc.borrow().clone());
                    return;
                }

                add_message(&history_rc, "Формат файла верный".to_string());
                history_state.set(history_rc.borrow().clone());

                spawn_local(async move {
                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    let history_rc = history_rc.clone();
                    let history_state = history_state;
                    let cut_length = cut_length.clone();

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

                                        file_data.set(Some(file_data_obj.clone()));

                                        match calculate_dxf_length(file_data_obj) {
                                            Ok(result) => {
                                                // Добавляем все сообщения о результатах
                                                add_message(&history_rc, format!("Общая длина всех линий: {:.2} мм", result));
                                                history_state.set(history_rc.borrow().clone());

                                                // Записываем длину реза в переменную
                                                cut_length.set(result as f32);

                                                // Выводим сообщение о результате
                                                add_message(&history_rc, format!("Длина реза установлена: {:.2} мм", result));
                                                history_state.set(history_rc.borrow().clone());
                                            },
                                            Err(err) => {
                                                add_message(
                                                    &history_rc,
                                                    format!("Ошибка при расчете длины: {}", err)
                                                );
                                                history_state.set(history_rc.borrow().clone());
                                            }
                                        }

                                    }
                                    Err(_) => add_message(&history_rc, "Ошибка при чтении содержимого файла".to_string()),
                                    
                                }
                            }
                        }

                        history_state.set(history_rc.borrow().clone());
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();
                    reader.read_as_array_buffer(&file).unwrap();
                });
            }
        }
    })
}
