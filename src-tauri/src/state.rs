// src-tauri/src/state.rs
use std::sync::Mutex;
use tauri::State;
use crate::price_calculator;
use once_cell::sync::Lazy;
use log::info;

//----------------------------------------------------------------------
// СТРУКТУРЫ И СТАТИЧЕСКИЕ ПЕРЕМЕННЫЕ
//----------------------------------------------------------------------

// Статические переменные для хранения состояния
static TYPE_CUTTING: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static TYPE_MATERIAL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static THICKNESS: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static CUT_LENGTH: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static BENDING_POINTS: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(Vec::new()));
static THREADS_INSERTS_MATS: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(Vec::new()));
static COST_MATERIAL: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static QUANTITY_PARTS: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(1));
static MARGIN_DEAL: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(28.0));
static PRICE_ONE_PART: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static PRICE_ALL_PARTS: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static SPEED_CUTTING: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static POWERD_CUTTING: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

// Структуры для хранения состояния в Tauri
pub struct TypeCutting(pub Mutex<String>);
pub struct TypeMaterial(pub Mutex<String>);
pub struct Thickness(pub Mutex<f32>);
pub struct CutLength(pub Mutex<f32>);
pub struct BendingPoints(pub Mutex<Vec<i32>>);
pub struct ThreadsInsertsMats(pub Mutex<Vec<i32>>);
pub struct CostMaterial(pub Mutex<f32>);
pub struct QuantityParts(pub Mutex<i32>);
pub struct MarginDeal(pub Mutex<f32>);
pub struct PriceOnePart(pub Mutex<f32>);
pub struct PriceAllParts(pub Mutex<f32>);
//pub struct SpeedCutting(pub Mutex<f32>);
//pub struct PowerdCutting(pub Mutex<i32>);

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ТИПОМ РЕЗКИ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_type_cutting(
    state: State<TypeCutting>,
    cutting_type: String
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cutting = cutting_type.clone();
    
    // Обновляем статическую переменную
    let mut static_cutting = TYPE_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    *static_cutting = cutting_type;
    
    Ok(())
}

#[tauri::command]
pub fn get_type_cutting(
    state: State<TypeCutting>
) -> Result<String, String> {
    let cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(cutting.clone())
}

// Внутренняя функция для получения типа резки
pub fn get_type_cutting_internal() -> Result<String, String> {
    let cutting = TYPE_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    Ok(cutting.clone())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ТИПОМ МАТЕРИАЛА
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_type_material(
    state: State<TypeMaterial>,
    material_type: String
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut material = state.0.lock().map_err(|_| "Failed to lock state")?;
    *material = material_type.clone();
    
    // Обновляем статическую переменную
    let mut static_material = TYPE_MATERIAL.lock().map_err(|_| "Failed to lock static state")?;
    *static_material = material_type;
    
    Ok(())
}

#[tauri::command]
pub fn get_type_material(
    state: State<TypeMaterial>    
) -> Result<String, String> {
    let materialing = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(materialing.clone())
}

// Внутренняя функция для получения типа материала
pub fn get_type_material_internal() -> Result<String, String> {
    let material = TYPE_MATERIAL.lock().map_err(|_| "Failed to lock static state")?;
    Ok(material.clone())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ТОЛЩИНОЙ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_thickness(
    state: State<Thickness>,
    thickness: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut thickness_value = state.0.lock().map_err(|_| "Failed to lock state")?;
    *thickness_value = thickness;
    
    // Обновляем статическую переменную
    let mut static_thickness = THICKNESS.lock().map_err(|_| "Failed to lock static state")?;
    *static_thickness = thickness;

    
    Ok(())
}

#[tauri::command]
pub fn get_thickness(
    state: State<Thickness>
) -> Result<f32, String> {
    let thickness = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*thickness)
}

// Внутренняя функция для получения толщины
pub fn get_thickness_internal() -> Result<String, String> {
    let thickness = THICKNESS.lock().map_err(|_| "Failed to lock static state")?;
    Ok(thickness.to_string())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ДЛИНОЙ РЕЗА
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_cut_length(
    state: State<CutLength>,
    length: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cut_length = length;
    
    // Обновляем статическую переменную
    let mut static_length = CUT_LENGTH.lock().map_err(|_| "Failed to lock static state")?;
    *static_length = length;

    
    Ok(())
}

#[tauri::command]
pub fn get_cut_length(
    state: State<CutLength>
) -> Result<f32, String> {
    let cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cut_length)
}

// Внутренняя функция для получения длины реза
pub fn get_cut_length_internal() -> Result<f32, String> {
    let cut_length = CUT_LENGTH.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*cut_length)
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С РЕЗЬБАМИ/ВСТАВКАМИ/ЦЕКОВКАМИ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_threads_inserts_mats(
    state: State<ThreadsInsertsMats>,
    mats: Vec<i32>
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    *threads_inserts_mats = mats.clone();
    
    // Обновляем статическую переменную
    let mut static_mats = THREADS_INSERTS_MATS.lock().map_err(|_| "Failed to lock static state")?;
    *static_mats = mats;

    
    Ok(())
}

#[tauri::command]
pub fn get_threads_inserts_mats(
    state: State<ThreadsInsertsMats>
) -> Result<Vec<i32>, String> {
    let threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(threads_inserts_mats.clone())    
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ТОЧКАМИ СГИБА
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_bending_points(
    state: State<BendingPoints>,
    points: Vec<i32>
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut bending_points = state.0.lock().map_err(|_| "Failed to lock state")?;
    *bending_points = points.clone();
    
    // Обновляем статическую переменную
    let mut static_points = BENDING_POINTS.lock().map_err(|_| "Failed to lock static state")?;
    *static_points = points;

    
    Ok(())
}

#[tauri::command]
pub fn get_bending_points(
    state: State<BendingPoints>
) -> Result<Vec<i32>, String> {
    let bending_points = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(bending_points.clone())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ СО СТОИМОСТЬЮ МАТЕРИАЛА
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_cost_material(
    state: State<CostMaterial>,
    cost: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cost_material = cost;
    
    // Обновляем статическую переменную
    let mut static_cost = COST_MATERIAL.lock().map_err(|_| "Failed to lock static state")?;
    *static_cost = cost;

    
    Ok(())
}

#[tauri::command]
pub fn get_cost_material(
    state: State<CostMaterial>
) -> Result<f32, String> {
    let cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cost_material)
}

// Внутренняя функция для получения стоимости материала
pub fn get_cost_material_internal() -> Result<f32, String> {
    let cost_material = COST_MATERIAL.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*cost_material)
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С КОЛИЧЕСТВОМ ДЕТАЛЕЙ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_quantity_parts(
    state: State<QuantityParts>,
    quantity: i32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *quantity_parts = quantity;
    
    // Обновляем статическую переменную
    let mut static_quantity = QUANTITY_PARTS.lock().map_err(|_| "Failed to lock static state")?;
    *static_quantity = quantity;

    
    Ok(())
}

#[tauri::command]
pub fn get_quantity_parts(
    state: State<QuantityParts>
) -> Result<i32, String> {
    let quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*quantity_parts)
}

// Внутренняя функция для получения количества деталей
pub fn get_quantity_parts_internal() -> Result<i32, String> {
    let quantity_parts = QUANTITY_PARTS.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*quantity_parts)
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С МАРЖОЙ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_margin_deal(
    state: State<MarginDeal>,
    margin: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    *margin_deal = margin;
    
    // Обновляем статическую переменную
    let mut static_margin = MARGIN_DEAL.lock().map_err(|_| "Failed to lock static state")?;
    *static_margin = margin;

    
    Ok(())
}

#[tauri::command]
pub fn get_margin_deal(
    state: State<MarginDeal>
) -> Result<f32, String> {
    let margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*margin_deal)
}

// Внутренняя функция для получения маржи
pub fn get_margin_deal_internal() -> Result<i32, String> {
    let margin_deal = MARGIN_DEAL.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*margin_deal as i32)
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ЦЕНОЙ ОДНОЙ ДЕТАЛИ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_price_one_part(
    state: State<PriceOnePart>,
    price: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut price_one_part = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_one_part = price;
    
    // Обновляем статическую переменную
    let mut static_price = PRICE_ONE_PART.lock().map_err(|_| "Failed to lock static state")?;
    *static_price = price;
    
    Ok(())
}

#[tauri::command]
pub fn get_price_one_part(
    state: State<PriceOnePart>
) -> Result<f32, String> {
    // Получаем значение из статической переменной вместо состояния Tauri
    let price_one_part = PRICE_ONE_PART.lock().map_err(|_| "Failed to lock static state")?;
    info!("Запрошена цена одной детали: {}", *price_one_part);
    Ok(*price_one_part)
}

// Внутренняя функция для установки цены одной детали
pub fn set_price_one_part_internal(price: f32) -> Result<(), String> {
    let mut static_price = PRICE_ONE_PART.lock().map_err(|_| "Failed to lock static state")?;
    *static_price = price;
    info!("Установлена цена одной детали: {:.2}", price);
    Ok(())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ С ЦЕНОЙ ВСЕХ ДЕТАЛЕЙ
//----------------------------------------------------------------------

#[tauri::command]
pub fn set_price_all_parts(
    state: State<PriceAllParts>,
    price: f32
) -> Result<(), String> {
    // Обновляем состояние Tauri
    let mut price_all_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_all_parts = price;
    
    // Обновляем статическую переменную
    let mut static_price = PRICE_ALL_PARTS.lock().map_err(|_| "Failed to lock static state")?;
    *static_price = price;
    
    Ok(())
}

#[tauri::command]
pub fn get_price_all_parts(
    state: State<PriceAllParts>
) -> Result<f32, String> {
    // Получаем значение из статической переменной вместо состояния Tauri
    let price_all_parts = PRICE_ALL_PARTS.lock().map_err(|_| "Failed to lock static state")?;
    info!("Запрошена цена всех деталей: {}", *price_all_parts);
    Ok(*price_all_parts)
}

// Внутренняя функция для установки цены всех деталей
pub fn set_price_all_parts_internal(price: f32) -> Result<(), String> {
    let mut static_price = PRICE_ALL_PARTS.lock().map_err(|_| "Failed to lock static state")?;
    *static_price = price;
    info!("Установлена цена всех деталей: {:.2}", price);
    Ok(())
}

//----------------------------------------------------------------------
// ФУНКЦИИ ДЛЯ РАБОТЫ СО СКОРОСТЬЮ И МОЩНОСТЬЮ РЕЗКИ
//----------------------------------------------------------------------

// Внутренняя функция для получения скорости резки
pub fn get_speed_cutting_internal() -> Result<f32, String> {
    let speed_cutting = SPEED_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*speed_cutting)
}

// Внутренняя функция для установки скорости резки
pub fn set_speed_cutting_internal(speed: f32) -> Result<(), String> {
    let mut static_speed = SPEED_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    *static_speed = speed;
    Ok(())
}

// Внутренняя функция для получения мощности резки
pub fn get_powerd_cutting_internal() -> Result<i32, String> {
    let powerd_cutting = POWERD_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    Ok(*powerd_cutting)
}

// Внутренняя функция для установки мощности резки
pub fn set_powerd_cutting_internal(power: i32) -> Result<(), String> {
    let mut static_power = POWERD_CUTTING.lock().map_err(|_| "Failed to lock static state")?;
    *static_power = power;
    Ok(())
}

//----------------------------------------------------------------------
// КОМАНДА ДЛЯ РАСЧЕТА ЦЕНЫ РЕЗКИ
//----------------------------------------------------------------------

#[tauri::command]
pub fn calculate_cutting_price_command() -> Result<(), String> {
    // Вызываем функцию из модуля price_calculator
    price_calculator::calculate_cutting_price()
    
}
