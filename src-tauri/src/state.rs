use std::sync::Mutex;
use tauri::State;

pub struct TypeCutting(pub Mutex<String>);
pub struct TypeMaterial(pub Mutex<String>);
pub struct Thickness(pub Mutex<f32>);
pub struct CutLength(pub Mutex<f32>); // Новая структура для длины реза
pub struct DxfPath(pub Mutex<String>);


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

impl DxfPath {
    pub fn new() -> Self {
        DxfPath(Mutex::new(String::new()))
    }
}

#[tauri::command]
pub fn set_dxf_path(
    state: State<DxfPath>,
    dxf_path: String
) -> Result<String, String> {
    println!("Попытка установить путь к DXF файлу: {}", dxf_path);  // Логирование
    let mut path = state.0.lock().map_err(|_| "Failed to lock state")?;
    *path = dxf_path.clone();
    println!("Путь к DXF файлу успешно установлен: {}", dxf_path);  // Логирование
    Ok(format!("DXF путь успешно установлен: {}", dxf_path))
}

#[tauri::command]
pub fn get_dxf_path(
    state: State<DxfPath>
) -> Result<String, String> {
    println!("Запрос на получение пути к DXF файлу");  // Логирование
    let path = state.0.lock().map_err(|_| "Failed to lock state")?;
    println!("Получен путь к DXF файлу: {}", path);  // Логирование
    Ok(format!("Текущий путь к DXF файлу: {}", path))
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


