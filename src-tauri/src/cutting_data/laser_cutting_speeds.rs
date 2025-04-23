//! Модуль содержит данные о скорости резки лазерного станка
//! в зависимости от толщины материала и мощности станка.


use once_cell::sync::Lazy;

/// Структура для хранения данных о скорости резки
#[derive(Debug, Clone)]
pub struct CuttingSpeedData {
    /// Толщина материала в мм
    pub thickness: f32,
    /// Скорость резки в мм/мин
    pub speed: f32,
}

/// Данные о скорости резки для cтали
pub static LASER_4KW_STEEL: Lazy<Vec<CuttingSpeedData>> = Lazy::new(|| {
    vec![
        CuttingSpeedData {
            thickness: 0.5,
            speed: 33.0,   
        },
        CuttingSpeedData {
            thickness: 0.7,
            speed: 31.0,
        },
        CuttingSpeedData {
            thickness: 0.8,
            speed: 30.0,
        },
        CuttingSpeedData {
            thickness: 1.0,
            speed: 29.0,
        },
        CuttingSpeedData {
            thickness: 1.2,
            speed: 18.0,
        },
        CuttingSpeedData {
            thickness: 1.5,
            speed: 8.0, 
        },
        CuttingSpeedData {
            thickness: 2.0,
            speed: 5.2, 
        },
        CuttingSpeedData {
            thickness: 2.5,
            speed: 4.0,
        },
        CuttingSpeedData {
            thickness: 3.0,
            speed: 3.1,  
        },
        CuttingSpeedData {
            thickness: 4.0,
            speed: 3.0, 
        },
        CuttingSpeedData {
            thickness: 5.0,
            speed: 2.7, 
        },
        CuttingSpeedData {
            thickness: 6.0,
            speed: 2.5,
        },
        CuttingSpeedData {
            thickness: 8.0,
            speed: 2.0,
        },
        CuttingSpeedData {
            thickness: 10.0,
            speed: 1.5,
        },
        CuttingSpeedData {
            thickness: 12.0,
            speed: 1.2,
        },
    ]
});

/// Данные о скорости резки для алюминия
pub static LASER_4KW_ALUMINUM: Lazy<Vec<CuttingSpeedData>> = Lazy::new(|| {
    vec![
        CuttingSpeedData {
            thickness: 0.5,
            speed: 33.0,   
        },
        CuttingSpeedData {
            thickness: 0.7,
            speed: 31.0,
        },
        CuttingSpeedData {
            thickness: 0.8,
            speed: 30.0,
        },
        CuttingSpeedData {
            thickness: 1.0,
            speed: 29.0,
        },
        CuttingSpeedData {
            thickness: 1.2,
            speed: 18.0,
        },
        CuttingSpeedData {
            thickness: 1.5,
            speed: 15.0, 
        },
        CuttingSpeedData {
            thickness: 2.0,
            speed: 13.0, 
        },
        CuttingSpeedData {
            thickness: 2.5,
            speed: 10.0,
        },
        CuttingSpeedData {
            thickness: 3.0,
            speed: 7.0,  
        },
        CuttingSpeedData {
            thickness: 4.0,
            speed: 4.0, 
        },
        CuttingSpeedData {
            thickness: 5.0,
            speed: 3.0, 
        },
        CuttingSpeedData {
            thickness: 6.0,
            speed: 2.0,
        },
    ]
});

/// Перечисление типов материалов
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Steel,
    Aluminum,
}

/// Функция для получения скорости резки по толщине материала и типу материала
pub fn get_cutting_speed(thickness: f32, material: MaterialType) -> Option<f32> {
    let data = match material {
        MaterialType::Steel => &*LASER_4KW_STEEL,
        MaterialType::Aluminum => &*LASER_4KW_ALUMINUM,
    };
    
    // Поиск точного совпадения
    data.iter()
        .find(|entry| (entry.thickness - thickness).abs() < 0.01)
        .map(|entry| entry.speed)
}

/// Функция для получения всех доступных толщин для заданного материала
pub fn get_available_thicknesses(material: MaterialType) -> Vec<f32> {
    match material {
        MaterialType::Steel => LASER_4KW_STEEL.iter().map(|data| data.thickness).collect(),
        MaterialType::Aluminum => LASER_4KW_ALUMINUM.iter().map(|data| data.thickness).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exact_match() {
        assert_eq!(get_cutting_speed(1.0, MaterialType::Steel), Some(29.0));
        assert_eq!(get_cutting_speed(2.0, MaterialType::Aluminum), Some(13.0));
    }
    
    #[test]
    fn test_no_match() {
        // Проверка, что для несуществующей толщины возвращается None
        assert_eq!(get_cutting_speed(1.1, MaterialType::Steel), None);
        assert_eq!(get_cutting_speed(7.0, MaterialType::Aluminum), None);
    }
    
    #[test]
    fn test_available_thicknesses() {
        let thicknesses_steel = get_available_thicknesses(MaterialType::Steel);
        assert!(thicknesses_steel.contains(&1.0));
        assert!(thicknesses_steel.contains(&5.0));
        
        let thicknesses_aluminum = get_available_thicknesses(MaterialType::Aluminum);
        assert!(thicknesses_aluminum.contains(&2.0));
        assert!(thicknesses_aluminum.contains(&6.0));
    }
}
