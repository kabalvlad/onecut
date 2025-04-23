mod state;
mod price_calculator;
mod cutting_data;
use log::info;
use simplelog::{WriteLogger, LevelFilter, Config, TermLogger, TerminalMode, ColorChoice};
use std::fs::OpenOptions;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Настраиваем логирование
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("onecut_debug.log")
        .expect("Не удалось открыть файл лога");
    
    // Используем стандартную конфигурацию
    let config = Config::default();
    
    // Инициализируем логгер с фильтром по уровню
    simplelog::CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Info,  // Показывать только Info и выше (Error, Warn)
                config.clone(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                config,
                log_file,
            ),
        ]
    ).expect("Не удалось инициализировать логгер");
    
    // Устанавливаем фильтры для определенных модулей
    log::set_max_level(LevelFilter::Info);
    
    info!("Приложение запущено");

    tauri::Builder::default()
        // Регистрируем состояния
        .manage(state::TypeCutting(Mutex::new(String::new())))
        .manage(state::TypeMaterial(Mutex::new(String::new())))
        .manage(state::Thickness(Mutex::new(0.0)))
        .manage(state::CutLength(Mutex::new(0.0)))
        .manage(state::BendingPoints(Mutex::new(Vec::new())))
        .manage(state::ThreadsInsertsMats(Mutex::new(Vec::new())))
        .manage(state::CostMaterial(Mutex::new(0.0)))
        .manage(state::QuantityParts(Mutex::new(1)))
        .manage(state::MarginDeal(Mutex::new(28.0)))
        .manage(state::PriceOnePart(Mutex::new(0.0)))
        .manage(state::PriceAllParts(Mutex::new(0.0)))
    
        // Регистрируем обработчики команд напрямую из модуля state
        .invoke_handler(tauri::generate_handler![
            state::set_type_cutting,
            state::get_type_cutting,
            state::set_type_material,
            state::get_type_material,
            state::set_thickness,
            state::get_thickness,
            state::set_cut_length,
            state::get_cut_length,
            state::set_bending_points,
            state::get_bending_points,
            state::set_threads_inserts_mats,
            state::get_threads_inserts_mats,
            state::set_quantity_parts,
            state::get_quantity_parts,
            state::set_cost_material,
            state::get_cost_material,
            state::set_margin_deal,
            state::get_margin_deal,
            state::set_price_one_part,
            state::get_price_one_part,
            state::set_price_all_parts,
            state::get_price_all_parts,
            state::calculate_cutting_price_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
