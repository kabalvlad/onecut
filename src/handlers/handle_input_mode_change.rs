use yew::prelude::*;
use crate::models::{AppState, AppAction};

pub fn handle_input_mode_change(state: UseReducerHandle<AppState>) -> Callback<String> {
    Callback::from(move |mode: String| {
        match mode.as_str() {
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

