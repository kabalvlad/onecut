use std::sync::Mutex;
use tauri::State;

pub struct TypeCutting(pub Mutex<String>);
pub struct TypeMaterial(pub Mutex<String>);
pub struct Thickness(pub Mutex<f32>);
pub struct CutLength(pub Mutex<f32>); // Новая структура для длины реза
pub struct BendingPoints(pub Mutex<Vec<i32>>);
pub struct ThreadsInsertsMats(pub Mutex<Vec<i32>>); // Новая структура для вставок и ниток и мats
pub struct QuantityParts(pub Mutex<i32>); // kolichestvo deatlei
pub struct CostMaterial(pub Mutex<f32>); // cena materiala
pub struct MarginDeal(pub Mutex<f32>); // margin zakaza
pub struct PriceOnePart(pub Mutex<f32>); // cena odnoy deatlei
pub struct PriceAllParts(pub Mutex<f32>); // cena vseh deatlei

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

#[tauri::command]
pub fn set_quantity_parts(
    state: State<QuantityParts>,
    quantity: i32
) -> Result<(), String> {
    let mut quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *quantity_parts = quantity;
    Ok(())
}

#[tauri::command]
pub fn get_quantity_parts(
    state: State<QuantityParts>
) -> Result<i32, String> {
    let quantity_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*quantity_parts)
}


#[tauri::command]
pub fn set_price_all_parts(
    state: State<PriceAllParts>,
    price: f32
) -> Result<(), String> {
    let mut price_all_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_all_parts = price;
    Ok(())
}

#[tauri::command]
pub fn get_price_all_parts(
    state: State<PriceAllParts>
) -> Result<f32, String> {
    let price_all_parts = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*price_all_parts)
}


#[tauri::command]
pub fn set_price_one_part(
    state: State<PriceOnePart>,
    price: f32
) -> Result<(), String> {
    let mut price_one_part = state.0.lock().map_err(|_| "Failed to lock state")?;
    *price_one_part = price;
    Ok(())
}

#[tauri::command]
pub fn get_price_one_part(
    state: State<PriceOnePart>
) -> Result<f32, String> {
    let price_one_part = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*price_one_part)
}

#[tauri::command]
pub fn set_margin_deal(
    state: State<MarginDeal>,
    margin: f32
) -> Result<(), String> {
    let mut margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    *margin_deal = margin;
    Ok(())
}

#[tauri::command]
pub fn get_margin_deal(
    state: State<MarginDeal>
) -> Result<f32, String> {
    let margin_deal = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*margin_deal)
}

#[tauri::command]
pub fn set_cost_material(
    state: State<CostMaterial>,
    cost: f32
) -> Result<(), String> {
    let mut cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cost_material = cost;
    Ok(())
}

#[tauri::command]
pub fn get_cost_material(
    state: State<CostMaterial>
) -> Result<f32, String> {
    let cost_material = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cost_material)
}


#[tauri::command]
pub fn set_threads_inserts_mats(
    state: State<ThreadsInsertsMats>,
    mats: Vec<i32>
) -> Result<(), String> {
    let mut threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    *threads_inserts_mats = mats;
    Ok(())
}


#[tauri::command]
pub fn get_threads_inserts_mats(
    state: State<ThreadsInsertsMats>
) -> Result<Vec<i32>, String> {
    let threads_inserts_mats = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(threads_inserts_mats.clone())    
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
    *bending_points = points;
    Ok(())
}

#[tauri::command]
pub fn set_type_cutting(
    state: State<TypeCutting>,
    cutting_type: String
) -> Result<(), String> {
    let mut cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cutting = cutting_type;
    Ok(())
}

#[tauri::command]
pub fn get_type_cutting(
    state: State<TypeCutting>
) -> Result<String, String> {
    let cutting = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(cutting.clone())
}

#[tauri::command]
pub fn set_type_material(
    state: State<TypeMaterial>,
    material_type: String
) -> Result<(), String> {
    let mut materialing = state.0.lock().map_err(|_| "Failed to lock state")?;
    *materialing = material_type;
    Ok(())
}

#[tauri::command]
pub fn get_type_material(
    state: State<TypeMaterial>
) -> Result<String, String> {
    let materialing = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(materialing.clone())
}

#[tauri::command]
pub fn set_thickness(
    state: State<Thickness>,
    thickness: f32
) -> Result<(), String> {
    let mut thickness_value = state.0.lock().map_err(|_| "Failed to lock state")?;
    *thickness_value = thickness;
    Ok(())
}

#[tauri::command]
pub fn get_thickness(
    state: State<Thickness>
) -> Result<f32, String> {
    let thickness = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*thickness)
}

#[tauri::command]
pub fn set_cut_length(
    state: State<CutLength>,
    length: f32
) -> Result<(), String> {
    let mut cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    *cut_length = length;
    Ok(())
}

#[tauri::command]
pub fn get_cut_length(
    state: State<CutLength>
) -> Result<f32, String> {
    let cut_length = state.0.lock().map_err(|_| "Failed to lock state")?;
    Ok(*cut_length)
}


