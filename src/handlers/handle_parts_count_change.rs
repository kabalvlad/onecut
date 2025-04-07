use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use crate::models::{AppState, AppAction};

pub fn handle_parts_count_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value();
        
        // Проверяем, что введенное значение - корректное целое число
        match input_value.parse::<i32>() {
            Ok(value) => {
                // Убедимся, что количество деталей не меньше 1
                let value = if value < 1 { 1 } else { value };
                
                // Обновляем состояние количества деталей через редуктор
                state.dispatch(AppAction::SetPartsCount(value));
                
                // Добавляем сообщение в историю
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Установлено количество деталей: {}", value)
                ));
                
                // Обновляем поле ввода с корректным значением
                input.set_value(&value.to_string());
            },
            Err(_) => {
                // Добавляем сообщение об ошибке в историю
                state.dispatch(AppAction::AddHistoryMessage(
                    "Ошибка: введено некорректное количество деталей".to_string()
                ));
                
                // Возвращаем последнее корректное значение в поле ввода
                input.set_value(&state.parts_count.to_string());
            }
        }
    })
}
