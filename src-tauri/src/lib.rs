// src-tauri/src/lib.rs

mod state;
mod price_calculator;
mod cutting_data;


use state::{
    TypeCutting, TypeMaterial, Thickness, CutLength, BendingPoints,
    ThreadsInsertsMats, QuantityParts, CostMaterial, MarginDeal,
    PriceOnePart, PriceAllParts,
    set_type_cutting, get_type_cutting,
    set_type_material, get_type_material,
    set_thickness, get_thickness,
    set_cut_length, get_cut_length,
    set_bending_points, get_bending_points,
    set_threads_inserts_mats, get_threads_inserts_mats,
    set_quantity_parts, get_quantity_parts,
    set_cost_material, get_cost_material,
    set_margin_deal, get_margin_deal,
    set_price_one_part, get_price_one_part,
    set_price_all_parts, get_price_all_parts,
    calculate_cutting_price_command
   
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        
        // Регистрируем существующие состояния
        .manage(TypeCutting::new())
        .manage(TypeMaterial::new())
        .manage(Thickness::new())
        .manage(CutLength::new())
        .manage(BendingPoints::new())
        .manage(ThreadsInsertsMats::new())
        .manage(QuantityParts::new())
        .manage(CostMaterial::new())
        .manage(MarginDeal::new())
        .manage(PriceOnePart::new())
        .manage(PriceAllParts::new())

        
        // Регистрируем обработчики команд
        .invoke_handler(tauri::generate_handler![
            set_type_cutting,
            get_type_cutting,
            set_type_material,
            get_type_material,
            set_thickness,
            get_thickness,
            set_cut_length,
            get_cut_length,
            set_bending_points,
            get_bending_points,
            set_threads_inserts_mats,
            get_threads_inserts_mats,
            set_quantity_parts,
            get_quantity_parts,
            set_cost_material,
            get_cost_material,
            set_margin_deal,
            get_margin_deal,
            set_price_one_part,
            get_price_one_part,
            set_price_all_parts,
            get_price_all_parts,
            calculate_cutting_price_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
