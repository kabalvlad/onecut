use yew::prelude::*;
use crate::models::{AppState, AppAction};

//=======================================================================================================================

// Обработчик очистки толщины

//=======================================================================================================================

pub fn handle_clear_thickness(
    state: UseReducerHandle<AppState>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Очищаем значение толщины через dispatch, передавая пустую строку
        state.dispatch(AppAction::SetThickness(String::new()));
                
        // Добавляем сообщение в историю
        state.dispatch(AppAction::AddHistoryMessage("Значение толщины сброшено".to_string()));
    })
}
