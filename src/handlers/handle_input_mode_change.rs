use yew::prelude::*;
use crate::models::{AppState, AppAction};

pub fn handle_input_mode_change(
    state: UseReducerHandle<AppState>,
) -> Callback<&'static str> {
    Callback::from(move |mode: &str| {
        match mode {
            "file" => {
                state.dispatch(AppAction::SwitchToFileMode);
            },
            "manual" => {
                state.dispatch(AppAction::SwitchToManualMode);
            },
            _ => {}
        }
    })
}
