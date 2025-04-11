use yew::prelude::*;

pub fn handle_input_mode_change() -> Callback<String> {
    Callback::from(move |mode: String| {
        match mode.as_str() {
            "file" => {
                web_sys::console::log_1(
                    &"Переключение в режим выбора файла".into()
                );
            },
            "manual" => {
                web_sys::console::log_1(
                    &"Переключение в режим ручного ввода".into()
                );
            },
            _ => {
                web_sys::console::error_1(
                    &format!("Неизвестный режим ввода: {}", mode).into()
                );
            }
        }
    })
}
