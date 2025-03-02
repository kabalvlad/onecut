// В начале файла добавьте импорт функции расчета
use crate::backend::calculate_dxf_length; // путь может отличаться в зависимости от структуры проекта

pub fn handle_file_select(
    history: UseStateHandle<VecDeque<String>>,
    file_data: UseStateHandle<Option<FileData>>
) -> Callback<Event> {
    let history_rc = Rc::new(RefCell::new((*history).clone()));
    let history_state = history.clone();

    Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let file_data = file_data.clone();
        let history_rc = history_rc.clone();
        let history_state = history_state.clone();

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

                    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Ok(buffer) = reader_clone.result() {
                            if let Ok(array_buffer) = buffer.dyn_into::<ArrayBuffer>() {
                                let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();

                                match String::from_utf8(bytes) {
                                    Ok(content) => {
                                        // Создаем объект FileData
                                        let file_data_obj = FileData {
                                            name: file_name.clone(),
                                            content: content.clone(),
                                        };

                                        // Создаем копию для расчета длины
                                        let file_data_for_calculation = FileData {
                                            name: file_data_obj.name.clone(),
                                            content: file_data_obj.content.clone(),
                                        };

                                        // Сохраняем оригинальные данные
                                        file_data.set(Some(file_data_obj));

                                        // Вызываем функцию расчета длины
                                        match calculate_dxf_length(file_data_for_calculation) {
                                            Ok(length) => {
                                                add_message(
                                                    &history_rc,
                                                    format!("Общая длина линий: {:.2} мм", length)
                                                );
                                            },
                                            Err(err) => {
                                                add_message(
                                                    &history_rc,
                                                    format!("Ошибка при расчете длины: {}", err)
                                                );
                                            }
                                        }

                                        add_message(&history_rc, "Файл успешно загружен".to_string());
                                        
                                        let lines_count = content.lines().count();
                                        let file_size = content.len();
                                        
                                        let first_lines = content.lines()
                                            .take(30)
                                            .collect::<Vec<_>>()
                                            .join("\n");
                                        
                                        let file_summary = format!(
                                            "Содержимое файла: {} строк, {} байт. Начало файла: {}...", 
                                            lines_count, 
                                            file_size,
                                            if first_lines.len() > 50 { &first_lines[0..50] } else { &first_lines }
                                        );
                                        
                                        add_message(&history_rc, file_summary);
                                        
                                        if content.contains("ENTITIES") && content.contains("SECTION") {
                                            add_message(&history_rc, "Обнаружена структура DXF файла".to_string());
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