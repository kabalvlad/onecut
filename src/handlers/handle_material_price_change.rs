use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use crate::models::{AppState, AppAction};

pub fn handle_material_price_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut input_value = input.value();
        
        // Заменяем запятую на точку для корректного парсинга
        input_value = input_value.replace(',', ".");
        
        // Проверяем, что введенное значение - корректное число с плавающей точкой
        let parse_result = if input_value.trim().is_empty() {
            Ok(0.0)
        } else {
            input_value.parse::<f32>()
        };
        
        match parse_result {
            Ok(value) => {
                // Убедимся, что цена не отрицательная
                let value = if value < 0.0 { 0.0 } else { value };
                
                // Обновляем состояние цены материала через редуктор
                state.dispatch(AppAction::SetMaterialPrice(value));
                
                // Добавляем сообщение в историю
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Установлена цена материала: {:.2} €", value)
                ));
                
                // Обновляем поле ввода с форматированным значением
                input.set_value(&format!("{:.2}", value));
            },
            Err(_) => {
                // Добавляем сообщение об ошибке в историю
                state.dispatch(AppAction::AddHistoryMessage(
                    "Ошибка: введено некорректное значение цены материала".to_string()
                ));
                
                // Восстанавливаем последнее корректное значение в поле ввода
                input.set_value(&format!("{:.2}", state.material_price));
            }
        }
    })
}
