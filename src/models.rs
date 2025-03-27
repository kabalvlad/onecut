// src/models.rs
use yew::prelude::*;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone)]
pub struct FileData {
    pub name: String,
    pub content: String,
}

// 1. Определяем все возможные действия (Action)
#[derive(Clone)]
pub enum AppAction {
    SetCuttingType(String),
    SetMaterial(String),
    SetThickness(String),
    SetCutLength(f32),
    SetBendingPoints { enabled: bool, count: Option<i32> },
    SetThreadsInsertsMats { enabled: bool, count: Option<i32> },
    SetPartsCount(i32),
    SetMaterialPrice(f32),
    SetMargin(i32),
    UpdatePrices { price_per_part: f32, price_total: f32 },
    AddHistoryMessage(String),
    SwitchToFileMode,
    SwitchToManualMode,
}

// Добавьте эти поля в структуру AppState
#[derive(Clone, PartialEq)]
pub struct AppState {
    pub cutting_type: String,
    pub material: String,
    pub thickness: String,
    pub cut_length: f32,
    pub bending_points_enabled: bool,
    pub bending_points_count: Option<i32>,
    pub threads_inserts_mats_enabled: bool,
    pub threads_inserts_mats_count: Option<i32>,
    pub parts_count: i32,
    pub material_price: f32,
    pub margin: i32,
    pub price_per_part: f32,
    pub price_total: f32,
    pub history: VecDeque<String>,
    // Добавляем недостающие поля
    pub is_file_selected: bool,
    pub is_manual_input: bool,
    pub file_path: String,
    pub manual_path: String,
}

// И обновите Default для добавления значений по умолчанию
impl Default for AppState {
    fn default() -> Self {
        Self {
            cutting_type: String::new(),
            material: String::new(),
            thickness: String::new(),
            cut_length: 0.0,
            bending_points_enabled: false,
            bending_points_count: None,
            threads_inserts_mats_enabled: false,
            threads_inserts_mats_count: None,
            parts_count: 1,
            material_price: 0.0,
            margin: 28,
            price_per_part: 0.0,
            price_total: 0.0,
            history: VecDeque::with_capacity(30),
            // Значения по умолчанию для новых полей
            is_file_selected: true,
            is_manual_input: false,
            file_path: String::new(),
            manual_path: String::new(),
        }
    }
}


// Реализуем трейт Reducible для AppState
impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        
        match action {
            AppAction::SetCuttingType(cutting_type) => {
                next_state.cutting_type = cutting_type;
                next_state.history.push_front(format!("Выбран тип резки: {}", next_state.cutting_type));
            },
            AppAction::SwitchToFileMode => {
                next_state.is_file_selected = true;
                next_state.is_manual_input = false;
                next_state.manual_path = String::new();
                next_state.history.push_front("Переключено на режим выбора файла".to_string());
            },
            AppAction::SwitchToManualMode => {
                next_state.is_file_selected = false;
                next_state.is_manual_input = true;
                next_state.file_path = String::new();
                next_state.history.push_front("Переключено на режим ручного ввода".to_string());
            },
            
            AppAction::SetMaterial(material) => {
                next_state.material = material;
                next_state.history.push_front(format!("Выбран материал: {}", next_state.material));
            },
            AppAction::SetThickness(thickness) => {
                next_state.thickness = thickness;
                next_state.history.push_front(format!("Установлена толщина: {} мм", next_state.thickness));
            },
            AppAction::SetCutLength(length) => {
                next_state.cut_length = length;
                next_state.history.push_front(format!("Установлена длина реза: {} мм", length));
            },
            AppAction::SetBendingPoints { enabled, count } => {
                next_state.bending_points_enabled = enabled;
                next_state.bending_points_count = count;
                if enabled {
                    if let Some(count) = count {
                        next_state.history.push_front(format!("Установлено {} точек сгиба", count));
                    }
                } else {
                    next_state.history.push_front("Точки сгиба отключены".to_string());
                }
            },
            AppAction::SetThreadsInsertsMats { enabled, count } => {
                next_state.threads_inserts_mats_enabled = enabled;
                next_state.threads_inserts_mats_count = count;
                if enabled {
                    if let Some(count) = count {
                        next_state.history.push_front(format!("Установлено {} резьб/вставок/цековок", count));
                    }
                } else {
                    next_state.history.push_front("Резьбы/вставки/цековки отключены".to_string());
                }
            },
            AppAction::SetPartsCount(count) => {
                next_state.parts_count = count;
                next_state.history.push_front(format!("Количество деталей: {}", count));
            },
            AppAction::SetMaterialPrice(price) => {
                next_state.material_price = price;
                next_state.history.push_front(format!("Цена материала: {:.2} €", price));
            },
            AppAction::SetMargin(margin) => {
                next_state.margin = margin;
                next_state.history.push_front(format!("Установлена маржа: {}%", margin));
            },
            AppAction::UpdatePrices { price_per_part, price_total } => {
                next_state.price_per_part = price_per_part;
                next_state.price_total = price_total;
                next_state.history.push_front(format!(
                    "Обновлены цены: за деталь {:.2} €, всего {:.2} €",
                    price_per_part, price_total
                ));
            },
            AppAction::AddHistoryMessage(message) => {
                next_state.history.push_front(message);
                if next_state.history.len() > 30 {
                    next_state.history.pop_back();
                }
            },
        }
        
        Rc::new(next_state)
    }
}

// Теперь функция app_reducer не нужна, так как её функциональность перенесена в реализацию Reducible
// Можно удалить или оставить для совместимости с существующим кодом
pub fn app_reducer(state: &AppState, action: AppAction) -> AppState {
    let cloned_state = state.clone();
    let rc_state = Rc::new(cloned_state);
    let new_rc_state = Reducible::reduce(rc_state, action);
    (*new_rc_state).clone()
}




/* 
#[derive(Clone)]
pub struct AppState {
    pub selected_cutting: UseStateHandle<String>,
    pub selected_material: UseStateHandle<String>,
    pub thickness_input: UseStateHandle<String>,
    pub filtered_thicknesses: UseStateHandle<Vec<f32>>,
    pub file_path: UseStateHandle<String>,
    pub manual_path: UseStateHandle<String>,
    pub is_file_selected: UseStateHandle<bool>,
    pub is_manual_input: UseStateHandle<bool>,
    pub cut_length: UseStateHandle<f32>,
    pub file_data: UseStateHandle<Option<FileData>>,
    pub bending_points_radio_state: UseStateHandle<String>,
    pub threads_inserts_mats_radio_state: UseStateHandle<String>,
    pub bending_points_input: UseStateHandle<String>,
    pub threads_inserts_mats_input: UseStateHandle<String>,
    pub parts_count: UseStateHandle<i32>,
    pub material_price: UseStateHandle<f32>,
    pub margin: UseStateHandle<i32>,
    pub history: UseStateHandle<VecDeque<String>>,
    pub price_per_part: UseStateHandle<f32>,
    pub price_total: UseStateHandle<f32>,
}
*/