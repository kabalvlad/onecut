mod state; // объявляем модуль
use state::{TypeCutting, set_type_cutting, get_type_cutting};
use state::{TypeMaterial, set_type_material, get_type_material};
use state::{Thickness, set_thickness, get_thickness};
use state::{CutLength, set_cut_length, get_cut_length};
use state::{BendingPoints, set_bending_points, get_bending_points};
use state::{ThreadsInsertsMats, set_threads_inserts_mats, get_threads_inserts_mats};








// Атрибут, который указывает, что эта функция является точкой входа для мобильных платформ
#[cfg_attr(mobile, tauri::mobile_entry_point)]

// Публичная функция run(), которая инициализирует приложение
pub fn run() {
    // Создаем новый экземпляр Builder с настройками по умолчанию
    tauri::Builder::default()
        // Добавляем плагин opener для работы с файлами
        .plugin(tauri_plugin_opener::init())
        
        // Регистрируем состояние TypeCutting для использования во всем приложении
        // TypeCutting.new() создает новый экземпляр состояния
        .manage(TypeCutting::new())
        .manage(TypeMaterial::new())
        .manage(Thickness::new())
        .manage(CutLength::new())
        .manage(BendingPoints::new())
        .manage(ThreadsInsertsMats::new())
        
        
        // Регистрируем обработчики команд, которые можно вызывать из frontend
        // set_type_cutting и get_type_cutting - это функции, определенные в state.rs
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
            get_threads_inserts_mats
                     

        ])
        
        // Запускаем приложение с сгенерированным контекстом
        .run(tauri::generate_context!())
        
        // Обрабатываем возможные ошибки при запуске
        // Если произойдет ошибка, программа паникует с указанным сообщением
        .expect("error while running tauri application");
}



