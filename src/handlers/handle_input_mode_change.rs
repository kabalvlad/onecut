use yew::prelude::*;

//===============================================================
//Обработчик для переключения режима ввода
//===============================================================

pub fn handle_input_mode_change(
    is_file_selected: UseStateHandle<bool>,
    is_manual_input: UseStateHandle<bool>,
    manual_path: UseStateHandle<String>,
    file_path: UseStateHandle<String>,
) -> Callback<&'static str> {
    Callback::from(move |mode: &str| {
        match mode {
            "file" => {
                is_file_selected.set(true);
                is_manual_input.set(false);
                manual_path.set(String::new()); // Очищаем ручной ввод
            },
            "manual" => {
                is_file_selected.set(false);
                is_manual_input.set(true);
                file_path.set(String::new()); // Очищаем путь к файлу
            },
            _ => {}
        }
    })
}
