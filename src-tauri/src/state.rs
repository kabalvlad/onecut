// src-tauri/src/state.rs

use std::sync::Mutex;
use tauri::State;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use crate::price_calculator;

// Существующие структуры
pub struct TypeCutting(pub Mutex<String>);
pub struct TypeMaterial(pub Mutex<String>);
pub struct Thickness(pub Mutex<f32>);
pub struct CutLength(pub Mutex<f32>);
pub struct BendingPoints(pub Mutex<Vec<i32>>);
pub struct ThreadsInsertsMats(pub Mutex<Vec<i32>>);
pub struct QuantityParts(pub Mutex<i32>);
pub struct CostMaterial(pub Mutex<f32>);
pub struct MarginDeal(pub Mutex<f32>);
pub struct PriceOnePart(pub Mutex<f32>);
pub struct PriceAllParts(pub Mutex<f32>);
pub struct SpeedCutting(pub Mutex<f32>);
pub struct PowerdCutting(pub Mutex<i32>);

// Новая структура для хранения всех данных
#[derive(Clone, Serialize, Deserialize)]
pub struct CuttingData {
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
    pub speed_cutting: f32,
    pub powerd_cutting: i32,
}

impl Default for CuttingData {
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
            speed_cutting: 0.0,
            powerd_cutting: 0,
        }
    }
}

// Глобальное состояние для CuttingData
pub static CUTTING_DATA: Lazy<Mutex<CuttingData>> = Lazy::new(|| {
    Mutex::new(CuttingData::default())
});

// Структура для использования в Tauri
pub struct AppState(pub Mutex<CuttingData>);

impl AppState {
    pub fn new() -> Self {
        AppState(Mutex::new(CuttingData::default()))
    }
}

// Реализации для существующих структур
impl PriceAllParts {
    pub fn new() -> Self {
        PriceAllParts(Mutex::new(0.0))
    }
}

impl PriceOnePart {
    pub fn new() -> Self {
        PriceOnePart(Mutex::new(0.0))
    }
}

impl MarginDeal {
    pub fn new() -> Self {
        MarginDeal(Mutex::new(0.0))
    }
}

impl CostMaterial {
    pub fn new() -> Self {
        CostMaterial(Mutex::new(0.0))
    }
}

impl QuantityParts {
    pub fn new() -> Self {
        QuantityParts(Mutex::new(0))
    }
}

impl ThreadsInsertsMats {
    pub fn new() -> Self {
        ThreadsInsertsMats(Mutex::new(Vec::new()))
    }
}

impl BendingPoints {
    pub fn new() -> Self {
        BendingPoints(Mutex::new(Vec::new()))
    }
}

impl TypeCutting {
    pub fn new() -> Self {
        TypeCutting(Mutex::new(String::new()))
    }
}

impl TypeMaterial {
    pub fn new() -> Self {
        TypeMaterial(Mutex::new(String::new()))
    }
}

impl Thickness {
    pub fn new() -> Self {
        Thickness(Mutex::new(0.0))
    }
}

impl CutLength {
    pub fn new() -> Self {
        CutLength(Mutex::new(0.0))
    }
}

impl SpeedCutting {
    pub fn new() -> Self {
        SpeedCutting(Mutex::new(0.0))
    }
    
}

impl PowerdCutting {
    pub fn new() -> Self {
        PowerdCutting(Mutex::new(0))
    }
}





// Tauri команда для запуска расчета цены резки
#[tauri::command]
pub fn calculate_cutting_price_command() -> Result<(), String> {
    // Вызываем функцию из модуля price_calculator
    price_calculator::calculate_cutting_price()
}



// Существующие функции для работы с отдельными полями
#[tauri::command]
pub fn set_quantity_parts(
    state: State<QuantityParts>,
    quantity: i32
) -> Result<(), String> {
    let mut quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *quantity_parts = quantity;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.parts_count = quantity;
    
    Ok(())
}

#[tauri::command]
pub fn get_quantity_parts(
    state: State<QuantityParts>
) -> Result<i32, String> {
    let quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*quantity_parts)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_quantity_parts_internal() -> Result<i32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.parts_count)
}

pub fn set_quantity_parts_internal(quantity: i32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.parts_count = quantity;
    Ok(())
}

#[tauri::command]
pub fn set_price_all_parts(
    state: State<PriceAllParts>,
    price: f32
) -> Result<(), String> {
    let mut price_all_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_all_parts = price;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.price_total = price;
    
    Ok(())
}

#[tauri::command]
pub fn get_price_all_parts(
    state: State<PriceAllParts>
) -> Result<f32, String> {
    let price_all_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*price_all_parts)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_price_all_parts_internal() -> Result<f32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.price_total)
}

pub fn set_price_all_parts_internal(price: f32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.price_total = price;
    Ok(())
}

#[tauri::command]
pub fn set_price_one_part(
    state: State<PriceOnePart>,
    price: f32
) -> Result<(), String> {
    let mut price_one_part = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_one_part = price;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.price_per_part = price;
    
    Ok(())
}

#[tauri::command]
pub fn get_price_one_part(
    state: State<PriceOnePart>
) -> Result<f32, String> {
    let price_one_part = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*price_one_part)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_price_one_part_internal() -> Result<f32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.price_per_part)
}

pub fn set_price_one_part_internal(price: f32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.price_per_part = price;
    Ok(())
}

#[tauri::command]
pub fn set_margin_deal(
    state: State<MarginDeal>,
    margin: f32
) -> Result<(), String> {
    let mut margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    *margin_deal = margin;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.margin = margin as i32;
    
    Ok(())
}

#[tauri::command]
pub fn get_margin_deal(
    state: State<MarginDeal>
) -> Result<f32, String> {
    let margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*margin_deal)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_margin_deal_internal() -> Result<i32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.margin)
}

pub fn set_margin_deal_internal(margin: i32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.margin = margin;
    Ok(())
}

#[tauri::command]
pub fn set_cost_material(
    state: State<CostMaterial>,
    cost: f32
) -> Result<(), String> {
    let mut cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cost_material = cost;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.material_price = cost;
    
    Ok(())
}

#[tauri::command]
pub fn get_cost_material(
    state: State<CostMaterial>
) -> Result<f32, String> {
    let cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cost_material)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_cost_material_internal() -> Result<f32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.material_price)
}

pub fn set_cost_material_internal(cost: f32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.material_price = cost;
    Ok(())
}

#[tauri::command]
pub fn set_threads_inserts_mats(
    state: State<ThreadsInsertsMats>,
    mats: Vec<i32>
) -> Result<(), String> {
    let mut threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    *threads_inserts_mats = mats.clone();
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.threads_inserts_mats_enabled = !mats.is_empty();
    data.threads_inserts_mats_count = if mats.is_empty() { None } else { Some(mats.len() as i32) };
    
    Ok(())
}

#[tauri::command]
pub fn get_threads_inserts_mats(
    state: State<ThreadsInsertsMats>
) -> Result<Vec<i32>, String> {
    let threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(threads_inserts_mats.clone())    
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_threads_inserts_mats_internal() -> Result<(bool, Option<i32>), String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok((data.threads_inserts_mats_enabled, data.threads_inserts_mats_count))
}

pub fn set_threads_inserts_mats_internal(enabled: bool, count: Option<i32>) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.threads_inserts_mats_enabled = enabled;
    data.threads_inserts_mats_count = count;
    Ok(())
}

#[tauri::command]
pub fn get_bending_points(
    state: State<BendingPoints>
) -> Result<Vec<i32>, String> {
    let bending_points = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(bending_points.clone())
}

#[tauri::command]
pub fn set_bending_points(
    state: State<BendingPoints>,
    points: Vec<i32>
) -> Result<(), String> {
    let mut bending_points = state.0.lock().map_err(|_| "Failed to lock state")?;
    *bending_points = points.clone();
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.bending_points_enabled = !points.is_empty();
    data.bending_points_count = if points.is_empty() { None } else { Some(points.len() as i32) };
    
    Ok(())
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_bending_points_internal() -> Result<(bool, Option<i32>), String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok((data.bending_points_enabled, data.bending_points_count))
}

pub fn set_bending_points_internal(enabled: bool, count: Option<i32>) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.bending_points_enabled = enabled;
    data.bending_points_count = count;
    Ok(())
}

#[tauri::command]
pub fn set_type_cutting(
    state: State<TypeCutting>,
    cutting_type: String
) -> Result<(), String> {
    let mut cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cutting = cutting_type.clone();
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.cutting_type = cutting_type;
    
    Ok(())
}

#[tauri::command]
pub fn get_type_cutting(
    state: State<TypeCutting>
) -> Result<String, String> {
    let cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(cutting.clone())
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_type_cutting_internal() -> Result<String, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.cutting_type.clone())
}

pub fn set_type_cutting_internal(cutting_type: String) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.cutting_type = cutting_type;
    Ok(())
}

#[tauri::command]
pub fn set_type_material(
    state: State<TypeMaterial>,
    material_type: String
) -> Result<(), String> {
    let mut materialing = state.0.lock().map_err(|_| "Failed to lock state")?;
    *materialing = material_type.clone();
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.material = material_type;
    
    Ok(())
}

#[tauri::command]
pub fn get_type_material(
    state: State<TypeMaterial>
) -> Result<String, String> {
    let materialing = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(materialing.clone())
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_type_material_internal() -> Result<String, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.material.clone())
}

pub fn set_type_material_internal(material_type: String) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.material = material_type;
    Ok(())
}

#[tauri::command]
pub fn set_thickness(
    state: State<Thickness>,
    thickness: f32
) -> Result<(), String> {
    let mut thickness_value = state.0.lock().map_err(|_| "Failed to lock state")?;
    *thickness_value = thickness;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.thickness = thickness.to_string();
    
    Ok(())
}

#[tauri::command]
pub fn get_thickness(
    state: State<Thickness>
) -> Result<f32, String> {
    let thickness = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*thickness)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_thickness_internal() -> Result<String, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.thickness.clone())
}

pub fn set_thickness_internal(thickness: String) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.thickness = thickness;
    Ok(())
}

#[tauri::command]
pub fn set_cut_length(
    state: State<CutLength>,
    length: f32
) -> Result<(), String> {
    let mut cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cut_length = length;
    
    // Обновляем также глобальное состояние
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.cut_length = length;
    
    Ok(())
}

#[tauri::command]
pub fn get_cut_length(
    state: State<CutLength>
) -> Result<f32, String> {
    let cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cut_length)
}

// Внутренние функции для работы с отдельными полями (без Tauri State)
pub fn get_cut_length_internal() -> Result<f32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    Ok(data.cut_length)
}

pub fn set_cut_length_internal(length: f32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock global state")?;
    data.cut_length = length;
    Ok(())
}

pub fn get_speed_cutting_internal() -> Result<f32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock state")?;
    Ok(data.speed_cutting)
}

pub fn set_speed_cutting_internal(value: f32) -> Result<(), String> {
    let mut data = CUTTING_DATA.lock().map_err(|_| "Failed to lock state")?;
    data.speed_cutting = value;
    Ok(())
}

pub fn get_powerd_cutting_internal() -> Result<i32, String> {
    let data = CUTTING_DATA.lock().map_err(|_| "Failed to lock state")?;
    Ok(data.powerd_cutting)
}

pub fn set_powerd_cutting_internal(value: i32) -> Result<(), String> {
    let mut  data = CUTTING_DATA.lock().map_err(|_| "Failed to lock state")?;
    data.powerd_cutting = value;
    Ok(())
}
