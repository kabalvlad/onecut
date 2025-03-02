use yew::prelude::*;
use std::collections::VecDeque;


//=======================================================================================================================

// Обработчик очистки толщины

//=======================================================================================================================



pub fn handle_clear_thickness(
    thickness_input: UseStateHandle<String>,
    filtered_thicknesses: UseStateHandle<Vec<f32>>,
    history: UseStateHandle<VecDeque<String>>
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Очищаем поле ввода
        thickness_input.set(String::new());
        
        // Очищаем отфильтрованные толщины
        filtered_thicknesses.set(Vec::new());
        
        // Добавляем сообщение в историю
        let message = "Значение толщины сброшено".to_string();
        let mut new_history = (*history).clone();
        if new_history.len() >= 30 {
            new_history.pop_back();
        }
        new_history.push_front(message);
        history.set(new_history);
    })
}
