use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetMaterialArgs {
    pub material_type: String,
}


pub fn handle_material_change(
    selected_material: UseStateHandle<String>,
    history: UseStateHandle<VecDeque<String>>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        selected_material.set(value.clone());

        let materialing_code = match value.as_str() {
            "aluminum" => "AL",
            "steel" => "ST",
            "stainless" => "SS",
            "copper" => "COP",
            "plastic" => "PLA",
            _ => "",
        };

        let args = SetMaterialArgs {
            material_type: materialing_code.to_string(),
        };
        
        wasm_bindgen_futures::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let _ = invoke("set_type_material", args).await;
        });

        let message = match value.as_str() {
            "aluminum" => "Выбран материал: Алюминий",
            "steel" => "Выбран материал: Сталь",
            "stainless" => "Выбран материал: Нержавеющая сталь",
            "copper" => "Выбран материал: Латунь/Бронза/Медь",
            "plastic" => "Выбран материал: Пластик",
            _ => "Выберите материал",
        }.to_string();

        let mut new_history = (*history).clone();
        if new_history.len() >= 30 {
            new_history.pop_back();
        }
        new_history.push_front(message);
        history.set(new_history);
    })
}
