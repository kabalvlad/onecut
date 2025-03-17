use dxf::Drawing;
use dxf::entities::EntityType;
use std::error::Error;
use std::f64::consts::PI;
use nalgebra::Vector3;
use crate::models::FileData;
use std::io::Cursor;

/// Функция для расчета длины дуги
fn calculate_arc_length(radius: f64, start_angle: f64, end_angle: f64) -> f64 {
    let angle = if end_angle < start_angle {
        end_angle + 2.0 * PI - start_angle
    } else {
        end_angle - start_angle
    };
    radius * angle
}

/// Функция для расчета длины эллипса
fn calculate_ellipse_length(major_axis: f64, minor_axis: f64, start_param: f64, end_param: f64) -> f64 {
    // Количество сегментов для численного интегрирования
    let num_segments = 1000;
    
    // Если параметры описывают полный эллипс (0 до 2π)
    let is_full_ellipse = (end_param - start_param).abs() >= 2.0 * PI;
    
    let mut length = 0.0;
    let dt = (end_param - start_param) / num_segments as f64;
    
    for i in 0..num_segments {
        let t = start_param + i as f64 * dt;
        
        // Производные параметрического уравнения эллипса
        let dx = -major_axis * t.sin();
        let dy = minor_axis * t.cos();
        
        // Элемент длины дуги
        let element_length = (dx * dx + dy * dy).sqrt() * dt;
        length += element_length;
    }
    
    if is_full_ellipse {
        // Для полного эллипса используем приближенную формулу Рамануджана
        let h = ((major_axis - minor_axis) / (major_axis + minor_axis)).powi(2);
        length = PI * (major_axis + minor_axis) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()));
    }
    
    length
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k); // Используем симметрию
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn calculate_spline_length(spline: &dxf::entities::Spline) -> f64 {
    let mut length = 0.0;
    
    let num_segments = 1000;
    
    let control_points = &spline.control_points;
    if control_points.len() < 2 {
        return 0.0;
    }

    if !spline.fit_points.is_empty() {
        let fit_points = &spline.fit_points;
        for i in 0..fit_points.len() - 1 {
            let p1 = &fit_points[i];
            let p2 = &fit_points[i + 1];
            
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let dz = p2.z - p1.z;
            
            let segment_length = (dx * dx + dy * dy + dz * dz).sqrt();
            length += segment_length;
        }
    } else {
        let control_vectors: Vec<Vector3<f64>> = control_points
            .iter()
            .map(|p| Vector3::new(p.x, p.y, p.z))
            .collect();

        let t_start = 0.0;
        let t_end = 1.0;
        let dt = (t_end - t_start) / num_segments as f64;

        let mut prev_point = control_vectors[0];
        
        for i in 1..=num_segments {
            let t = t_start + i as f64 * dt;
            
            let mut curr_point = Vector3::new(0.0, 0.0, 0.0);
            let n = control_vectors.len() - 1;
            
            for j in 0..=n {
                let basis = binomial_coefficient(n, j) as f64 
                    * (1.0 - t).powi((n - j) as i32) 
                    * t.powi(j as i32);
                curr_point += control_vectors[j] * basis;
            }
            
            let dx = curr_point.x - prev_point.x;
            let dy = curr_point.y - prev_point.y;
            let dz = curr_point.z - prev_point.z;
            
            let segment_length = (dx * dx + dy * dy + dz * dz).sqrt();
            length += segment_length;
            
            prev_point = curr_point;
        }
    }

    length * 1.99
}

/// Функция расчета общей длины всех линий в DXF файле
/// Принимает структуру FileData с содержимым файла
/// Возвращает общую длину в мм (f64) или ошибку
pub fn calculate_dxf_length(file_data: FileData) -> Result<f64, Box<dyn Error>> {
    // Загружаем DXF из строки содержимого файла
    let mut cursor = Cursor::new(file_data.content);
    let drawing = Drawing::load(&mut cursor)?;
    
    let mut total_length = 0.0;
     
   
 

    for entity in drawing.entities() {
        match &entity.specific {
            EntityType::Line(line) => {
                let dx = line.p2.x - line.p1.x;
                let dy = line.p2.y - line.p1.y;
                let dz = line.p2.z - line.p1.z;
                let length = (dx * dx + dy * dy + dz * dz).sqrt();
                
                total_length += length;
                
            },
            EntityType::Polyline(polyline) => {
                let mut poly_length = 0.0;
                let vertices: Vec<_> = polyline.vertices().collect();
                
                if !vertices.is_empty() {
                    for i in 0..vertices.len() - 1 {
                        let v1 = &vertices[i];
                        let v2 = &vertices[i + 1];
                        let dx = v2.location.x - v1.location.x;
                        let dy = v2.location.y - v1.location.y;
                        let dz = v2.location.z - v1.location.z;
                        let segment_length = (dx * dx + dy * dy + dz * dz).sqrt();
                        poly_length += segment_length;
                    }
                    
                    if polyline.is_closed() {
                        let first = &vertices[0];
                        let last = &vertices[vertices.len() - 1];
                        let dx = last.location.x - first.location.x;
                        let dy = last.location.y - first.location.y;
                        let dz = last.location.z - first.location.z;
                        let segment_length = (dx * dx + dy * dy + dz * dz).sqrt();
                        poly_length += segment_length;
                    }
                }

                total_length += poly_length;
                
            },
            EntityType::Arc(arc) => {
                let length = calculate_arc_length(arc.radius, arc.start_angle.to_radians(), arc.end_angle.to_radians());
                total_length += length;
                
            },
            EntityType::Circle(circle) => {
                let circumference = 2.0 * PI * circle.radius;
                total_length += circumference;
                
            },
            EntityType::Spline(spline) => {
                let spline_length = calculate_spline_length(&spline);
                total_length += spline_length;
                
            },
            EntityType::Ellipse(ellipse) => {
                let major_length = (ellipse.major_axis.x.powi(2) + 
                                  ellipse.major_axis.y.powi(2) + 
                                  ellipse.major_axis.z.powi(2)).sqrt();
                let minor_length = major_length * ellipse.minor_axis_ratio;
                
                let length = calculate_ellipse_length(
                    major_length,
                    minor_length,
                    ellipse.start_parameter,
                    ellipse.end_parameter
                );
                
                total_length += length;
                
            },
            _ => {} // Игнорируем другие типы объектов
        }
    }

    // Возвращаем общую длину всех элементов в ф64
    Ok(total_length)
}
