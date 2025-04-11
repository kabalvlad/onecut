//! Модуль для хранения данных о параметрах резки

pub mod laser_cutting_speeds;

pub use laser_cutting_speeds::{
    get_cutting_speed,
    get_available_thicknesses,
    CuttingSpeedData,
    MaterialType,
    LASER_4KW_STEEL,
    LASER_4KW_ALUMINUM,
};
