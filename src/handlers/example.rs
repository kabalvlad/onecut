use web_sys::Event;
use yew::prelude::*;
use std::collections::VecDeque;

pub fn handle_cut_length_input(
    cut_length: UseStateHandle<f32>,
    history: UseStateHandle<VecDeque<String>>
) -> Callback<Event> {
    // Здесь весь код вашего обработчика
    Callback::from(move |e: Event| {
        // Код обработчика
    })
}
