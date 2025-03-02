use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use std::collections::VecDeque;
use std::str::FromStr;

//===============================================================
// Обработчик для ввода толщины
//===============================================================


pub fn handle_thickness_input(
    thickness_input: UseStateHandle<String>,
    filtered_thicknesses: UseStateHandle<Vec<f32>>,
    history: UseStateHandle<VecDeque<String>>,
    all_thicknesses: Vec<f32>
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = input.value();
        thickness_input.set(value.clone());

        // Если строка пустая, просто показываем все толщины
        if value.trim().is_empty() {
            filtered_thicknesses.set(all_thicknesses.clone());
            return;
        }

        // Заменяем запятую на точку для корректного парсинга
        let normalized_value = value.replace(',', ".");
        
        // Проверяем, содержит ли строка только цифры, точку или запятую
        if !normalized_value.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') {
            // Добавляем сообщение об ошибке в историю
            let error_message = "Ошибка: Пожалуйста, введите только числовое значение толщины без посторонних символов".to_string();
            let mut new_history = (*history).clone();
            if new_history.len() >= 5 {
                new_history.pop_back();
            }
            new_history.push_front(error_message);
            history.set(new_history);
            
            // Показываем все доступные толщины
            filtered_thicknesses.set(all_thicknesses.clone());
            return;
        }

        // Пытаемся преобразовать строку в число
        match f32::from_str(&normalized_value) {
            Ok(input_num) => {
                if input_num <= 0.0 {
                    // Добавляем сообщение об ошибке для отрицательных чисел
                    let error_message = "Ошибка: Толщина должна быть положительным числом".to_string();
                    let mut new_history = (*history).clone();
                    if new_history.len() >= 30 {
                        new_history.pop_back();
                    }
                    new_history.push_front(error_message);
                    history.set(new_history);
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

                // Находим несколько ближайших значений
                let mut nearest_values: Vec<f32> = all_thicknesses
                    .iter()
                    .copied()
                    .collect();
                nearest_values.sort_by(|a, b| {
                    (a - input_num).abs()
                        .partial_cmp(&(b - input_num).abs())
                        .unwrap()
                });

                // Берем ближайшее значение
                filtered_thicknesses.set(nearest_values.into_iter().take(1).collect());

                // Добавляем сообщение о ближайшей толщине
                let message = format!("Выбрана ближайшая доступная толщина: {} мм", nearest);
                let mut new_history = (*history).clone();
                if new_history.len() >= 5 {
                    new_history.pop_back();
                }
                new_history.push_front(message);
                history.set(new_history);

                // Устанавливаем значение поля ввода как ближайшее найденное
                thickness_input.set(nearest.to_string());
            },
            Err(_) => {
                // Добавляем сообщение об ошибке при неверном формате числа
                let error_message = "БОРОДА что ТЫ ДЕЛАЕШЬ ??? : дай нормальне число ".to_string();
                let mut new_history = (*history).clone();
                if new_history.len() >= 5 {
                    new_history.pop_back();
                }
                new_history.push_front(error_message);
                history.set(new_history);
                
                // Показываем все доступные толщины
                filtered_thicknesses.set(all_thicknesses.clone());
            }
        }
    })
}
