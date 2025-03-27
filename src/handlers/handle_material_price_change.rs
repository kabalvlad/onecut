use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use crate::models::{AppState, AppAction};

pub fn handle_material_price_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        // Предотвращаем стандартное поведение события
        e.prevent_default();
        
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
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Установлена цена материала: {} €", value)
                ));
                
                // Обновляем поле ввода с форматированным значением
                input.set_value(&format!("{:.2}", value));
                
                // Пересчитываем цены
                let price_per_part = calculate_price_per_part(&state);
                let price_total = calculate_total_price(&state);
                
                state.dispatch(AppAction::UpdatePrices {
                    price_per_part,
                    price_total,
                });
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

// Вспомогательные функции для расчета цен
fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета цены за деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета общей цены
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}
