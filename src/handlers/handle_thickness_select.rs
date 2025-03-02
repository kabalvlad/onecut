use web_sys::Event;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

//=======================================================================================================================

// Обработчик выбора толщины

//=======================================================================================================================


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetThicknessArgs {
    pub thickness: f32,
}

pub fn handle_thickness_select(
    history: UseStateHandle<VecDeque<String>>,
    thickness_input: UseStateHandle<String>,
    filtered_thicknesses: UseStateHandle<Vec<f32>>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(target) = e.target() {
            let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let value = select.value();
            
            // Преобразуем строковое значение в число
            if let Ok(thickness) = value.parse::<f32>() {
                // Обновляем значение в поле ввода
                thickness_input.set(thickness.to_string());
                
                // Обновляем отфильтрованные толщины
                filtered_thicknesses.set(vec![thickness]);
                
                // Добавляем сообщение в историю
                let message = format!("Выбрана толщина: {} мм", thickness);
                let mut new_history = (*history).clone();
                if new_history.len() >= 30 {
                    new_history.pop_back();
                }
                new_history.push_front(message);
                history.set(new_history);
            }
        }
    })
}
