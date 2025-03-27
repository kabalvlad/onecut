// src/models.rs

#[derive(Clone, PartialEq)]  // Добавьте PartialEq здесь
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
    pub history: Vec<String>,
}
