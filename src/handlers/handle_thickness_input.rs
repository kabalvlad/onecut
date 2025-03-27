use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use std::str::FromStr;
use crate::models::{AppState, AppAction};

pub fn handle_thickness_input(
    state: UseReducerHandle<AppState>,
    all_thicknesses: Vec<f32>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        
        // Обновляем значение поля ввода
        state.dispatch(AppAction::SetThickness(value.clone()));
        
        // Если строка пустая, показываем все толщины
        if value.trim().is_empty() {
            // Используем существующий вариант для обновления списка толщин
            // Предполагаем, что у вас есть действие для обновления списка толщин
            update_thickness_list(&state, all_thicknesses.clone());
            return;
        }

        // Заменяем запятую на точку для корректного парсинга
        let normalized_value = value.replace(',', ".");
        
        // Проверяем, содержит ли строка только цифры, точку или запятую
        if !normalized_value.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            state.dispatch(AppAction::AddHistoryMessage(
                "Ошибка: Пожалуйста, введите только числовое значение толщины без посторонних символов".to_string()
            ));
            update_thickness_list(&state, all_thicknesses.clone());
            return;
        }

        // Пытаемся преобразовать строку в число
        match f32::from_str(&normalized_value) {
            Ok(input_num) => {
                if input_num <= 0.0 {
                    state.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка: Толщина должна быть положительным числом".to_string()
                    ));
                    return;
                }

                // Находим ближайшее значение из списка
                let nearest = all_thicknesses
                    .iter()
                    .min_by(|&&a, &b| {
                        (a - input_num).abs()
                            .partial_cmp(&(b - input_num).abs())
                            .unwrap()
                    })
                    .copied()
                    .unwrap_or(all_thicknesses[0]);

                // Устанавливаем ближайшее значение как единственное в фильтрованном списке
                update_thickness_list(&state, vec![nearest]);
                
                // Обновляем поле ввода и добавляем сообщение в историю
                state.dispatch(AppAction::SetThickness(nearest.to_string()));
                state.dispatch(AppAction::AddHistoryMessage(
                    format!("Выбрана ближайшая доступная толщина: {} мм", nearest)
                ));

                // Пересчитываем цены
                let price_per_part = calculate_price_per_part(&state);
                let price_total = calculate_total_price(&state);
                
                state.dispatch(AppAction::UpdatePrices {
                    price_per_part,
                    price_total,
                });
            },
            Err(_) => {
                state.dispatch(AppAction::AddHistoryMessage(
                    "БОРОДА что ТЫ ДЕЛАЕШЬ ??? : дай нормальне число ".to_string()
                ));
                update_thickness_list(&state, all_thicknesses.clone());
            }
        }
    })
}

// Вспомогательная функция для обновления списка толщин
// Эта функция будет использовать правильный вариант AppAction
fn update_thickness_list(state: &UseReducerHandle<AppState>, thicknesses: Vec<f32>) {
    // Здесь нужно использовать правильный вариант из вашего AppAction
    // Например:
    //state.dispatch(AppAction::UpdateFilteredThicknesses(thicknesses));
    // или
    //state.dispatch(AppAction::SetAvailableThicknesses(thicknesses));
    // или другой вариант, который у вас определен
}

fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета цены за деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета общей цены
    state.price_per_part * state.parts_count as f32
}
