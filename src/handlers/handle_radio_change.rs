use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

//===============================================================
// Обработчик для выбора типа резки
//===============================================================

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetTypeCuttingArgs {
    pub cutting_type: String,
}

pub fn handle_cutting_type_change(
    selected_cutting: UseStateHandle<String>,
    history: UseStateHandle<VecDeque<String>>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        selected_cutting.set(value.clone());
        
        let cutting_code = match value.as_str() {
            "laser" => "LC",
            "plasma" => "PC",
            "hydro" => "HC",
            _ => "",
        };

        let args = SetTypeCuttingArgs {
            cutting_type: cutting_code.to_string(),
        };
        
        wasm_bindgen_futures::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let _ = invoke("set_type_cutting", args).await;
        });

        let message = match value.as_str() {
            "laser" => "Выбрана лазерная резка",
            "plasma" => "Выбрана плазменная резка",
            "hydro" => "Выбрана гидроабразивная резка",
            _ => "Выберите тип резки",
        }.to_string();

        let mut new_history = (*history).clone();
        if new_history.len() >= 30 {
            new_history.pop_back();
        }
        new_history.push_front(message);
        history.set(new_history);
    })
}
