// src-tauri/src/price_calculator.rs

use crate::state::{
    get_bending_points_internal, get_cost_material_internal, get_cut_length_internal,
      get_margin_deal_internal, get_powerd_cutting_internal,
      get_quantity_parts_internal, get_thickness_internal, get_threads_inserts_mats_internal, 
      get_type_cutting_internal, get_type_material_internal, 
       set_powerd_cutting_internal, set_price_all_parts_internal, set_price_one_part_internal,
        set_speed_cutting_internal
};
use crate::state::get_speed_cutting_internal;
use crate::cutting_data::laser_cutting_speeds::{get_available_thicknesses, get_cutting_speed};
use crate::cutting_data::MaterialType;


// Функция для обработки типа материала 
fn process_material_type() {
    // Получаем тип материала напрямую через get-функцию
    if let Ok(material_type) = get_type_material_internal() {
        // Используем match для обработки разных типов материалов
        match material_type.as_str() {
            "steel" => {
                // Для стали выполняем специальную логику
                calculate_steel_coefficient();
            },
            "aluminum" => {
                // Для алюминия ничего не делаем
            },
            "stainless" => {
                // Для нержавеющей стали ничего не делаем
            },
            "copper" => {
                // Для меди ничего не делаем
            },
            "brass" => {
                // Для латуни ничего не делаем
            },
            _ => println!("Unsupported material type: {}", material_type)
        }
    }
}

fn calculate_steel_coefficient() {
    // Получаем необходимые значения
    if let Ok(thickness_str) = get_thickness_internal() {
        if let Ok(material_type_str) = get_type_material_internal() {
            if let Ok(cutting_type) = get_type_cutting_internal() {
                // Преобразуем строку thickness в f32
                if let Ok(thickness) = thickness_str.parse::<f32>() {
                    // Преобразуем строку в MaterialType
                    let material_type = match material_type_str.as_str() {
                        "steel" => Some(MaterialType::Steel),
                        "aluminum" => Some(MaterialType::Aluminum),
                        _ => None,
                    };

                    if let Some(material_type) = material_type {
                        if cutting_type == "laser" {
                            if get_available_thicknesses(material_type).contains(&thickness) {
                                // Получаем скорость резки для данной толщины и материала
                                if let Some(speed) = get_cutting_speed(thickness, material_type) {
                                    let _ = set_speed_cutting_internal(speed);
                                    let _ = set_powerd_cutting_internal(4);
                                } else {
                                    println!("Не удалось получить скорость резки для толщины {} мм", thickness);
                                }
                            } else {
                                println!("Толщина {} мм не доступна", thickness);
                            }
                        }
                    } else {
                        println!("Неподдерживаемый тип материала: {}", material_type_str);
                    }
                } else {
                    println!("Не удалось преобразовать толщину '{}' в число", thickness_str);
                }
            }
        }
    }
}



    // Если тип резки не "лазер", то выводи
   
    



    pub fn calculate_cutting_price() -> Result<(), String> {
        // Читаем значения из конфигурационного файла
        if let Ok(config_content) = std::fs::read_to_string("config.txt") {
            let values: Vec<f32> = config_content
                .lines()
                .filter_map(|line| line.trim().parse().ok())
                .collect();
    
            if values.len() >= 13 {
                let employees_salary = values[0];
                let shock_absorption_station = values[1];
                let cost_electricity = values[2];
                let service_refect = values[3];
                let consumable_materials = values[4];
                let lease_pothek = values[5];
                let insurance = values[6];
                let administrative_expenses = values[7];
                let equipment_wear = values[8];
                let training_personals = values[9];
                let unforeseen_expenses = values[10];
                let percent_profits = values[11];
                let tax_statement = values[12];         
                
                // Вызываем process_material_type
                process_material_type();
                
                // Получаем значение power_cutting и обрабатываем возможную ошибку
                let power_cutting = get_powerd_cutting_internal()?;
                let cutting_length = get_cut_length_internal()?;
                let cutting_speed = get_speed_cutting_internal()?;
                let count_part = get_quantity_parts_internal()?;
                let cost_matereial = get_cost_material_internal()?;
                let margin = get_margin_deal_internal()?;
                
                // Теперь можем использовать power_cutting в умножении
                let hour_cost =
                    (employees_salary + shock_absorption_station + (cost_electricity * power_cutting as f32) + 
                    service_refect + consumable_materials + lease_pothek + insurance + 
                    administrative_expenses + equipment_wear + training_personals + unforeseen_expenses) * 
                    (1.0 + percent_profits / 100.0) * (1.0 + tax_statement / 100.0);
                
                let cutting_price_one = (((hour_cost / 60.0) * ((cutting_length / 100.0) / cutting_speed)) + 
                cost_matereial) * (1.0 + margin as f32 / 100.0);
                let cutting_price_all = cutting_price_one * count_part as f32;
                let _ = set_price_one_part_internal(cutting_price_one);
                let _ = set_price_all_parts_internal(cutting_price_all);               
                
                Ok(())
            } else {
                Err("Invalid config file format".to_string())
            }
        } else {
            Err("Failed to read config file".to_string())
        }
    }
    
    
    


