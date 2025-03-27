use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::{AppState, AppAction};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SetMaterialArgs {
    pub material_type: String,
}

// Структура для хранения информации о материалах
struct MaterialInfo {
    code: &'static str,
    description: &'static str,
}

pub fn handle_material_change(
    state: UseReducerHandle<AppState>,
) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let value = target.value();
        
        // Получаем информацию о материале
        let material_info = match value.as_str() {
            "aluminum" => MaterialInfo { 
                code: "AL", 
                description: "Алюминий" 
            },
            "steel" => MaterialInfo { 
                code: "ST", 
                description: "Сталь" 
            },
            "stainless" => MaterialInfo { 
                code: "SS", 
                description: "Нержавеющая сталь" 
            },
            "copper" => MaterialInfo { 
                code: "COP", 
                description: "Латунь/Бронза/Медь" 
            },
            "plastic" => MaterialInfo { 
                code: "PLA", 
                description: "Пластик" 
            },
            _ => MaterialInfo { 
                code: "", 
                description: "материал" 
            },
        };

        // Обновляем состояние через редуктор
        state.dispatch(AppAction::SetMaterial(value.clone()));
        state.dispatch(AppAction::AddHistoryMessage(
            format!("Выбран материал: {}", material_info.description)
        ));

        // Отправляем информацию о материале на бэкенд
        let args = SetMaterialArgs {
            material_type: material_info.code.to_string(),
        };
        let state_clone = state.clone();
        
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            match invoke("set_type_material", args).await {
                result if !result.is_undefined() => {
                    // После успешного обновления на бэкенде, обновляем цены
                    state_clone.dispatch(AppAction::UpdatePrices {
                        price_per_part: calculate_price_per_part(&state_clone),
                        price_total: calculate_total_price(&state_clone),
                    });
                },
                _ => {
                    state_clone.dispatch(AppAction::AddHistoryMessage(
                        "Ошибка при установке типа материала".to_string()
                    ));
                }
            }
        });
    })
}

// Вспомогательные функции для расчета цен
fn calculate_price_per_part(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета цены за деталь
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}

fn calculate_total_price(state: &AppState) -> f32 {
    // Здесь должна быть ваша логика расчета общей цены
    // Используйте значения из state
    0.0 // Замените на реальный расчет
}
