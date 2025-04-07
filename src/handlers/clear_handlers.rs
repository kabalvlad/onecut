use yew::prelude::*;
use crate::models::{AppState, AppAction};

// Обработчик для очистки полей расчета стоимости
pub fn handle_clear_options(
    state: UseReducerHandle<AppState>
) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        // Сбрасываем только входные поля через dispatch
        state.dispatch(AppAction::SetPartsCount(1));
        state.dispatch(AppAction::SetMaterialPrice(0.0));
        state.dispatch(AppAction::SetMargin(28));
        
        // Добавляем сообщение в историю
        state.dispatch(AppAction::AddHistoryMessage("Очищены поля ввода расчета стоимости".to_string()));
    })
}
