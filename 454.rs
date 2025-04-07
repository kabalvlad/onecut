let handle_thickness_select = {
    let state = state.clone();
    Callback::from(move |e: Event| {
        let select: HtmlInputElement = e.target_unchecked_into();
        state.dispatch(AppAction::SetThickness(select.value()));
        
        // Клонируем message перед использованием
        let message = state.message.clone();
        state.dispatch(AppAction::AddHistoryMessage(message));
    })
};
