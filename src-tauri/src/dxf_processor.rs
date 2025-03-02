use dxf::Drawing;
use dxf::entities::EntityType;
use std::io::Cursor;
use std::f64::consts::PI;
use onecut_ui::models::FileData;
use std::sync::Arc;

// Добавляем структуру для результатов расчета
#[derive(Debug, Clone)]
pub struct CalculationResult {
    pub total_length: f64,
    pub entity_counts: EntityCounts,
    pub calculation_time: std::time::Duration,
}

// Структура для подсчета количества различных типов элементов
#[derive(Debug, Clone, Default)]
pub struct EntityCounts {
    pub lines: usize,
    pub arcs: usize,
    pub circles: usize,
    pub polylines: usize,
    pub lwpolylines: usize,
    pub ellipses: usize,
}

pub fn calculate_dxf_length(file_data: Arc<FileData>) -> Result<CalculationResult, String> {
    let start_time = std::time::Instant::now();
    let mut reader = Cursor::new(file_data.content.as_bytes());
    let drawing = Drawing::load(&mut reader).map_err(|e| e.to_string())?;

    let mut cut_length = 0.0;
    let mut entity_counts = EntityCounts::default();

    for entity in drawing.entities() {
        match &entity.specific {
            EntityType::Line(line) => {
                entity_counts.lines += 1;
                let dx = line.p2.x - line.p1.x;
                let dy = line.p2.y - line.p1.y;
                cut_length += (dx * dx + dy * dy).sqrt();
            },

            EntityType::Arc(arc) => {
                entity_counts.arcs += 1;
                let radius = arc.radius;
                let start_angle = arc.start_angle.to_radians();
                let end_angle = arc.end_angle.to_radians();
                let angle = if end_angle < start_angle {
                    end_angle + 2.0 * PI - start_angle
                } else {
                    end_angle - start_angle
                };
                cut_length += radius * angle;
            },

            EntityType::Circle(circle) => {
                entity_counts.circles += 1;
                cut_length += 2.0 * PI * circle.radius;
            },

            EntityType::Polyline(polyline) => {
                entity_counts.polylines += 1;
                let vertices: Vec<_> = polyline.vertices().collect();
                for i in 1..vertices.len() {
                    let dx = vertices[i].location.x - vertices[i-1].location.x;
                    let dy = vertices[i].location.y - vertices[i-1].location.y;
                    cut_length += (dx * dx + dy * dy).sqrt();
                }

                if polyline.is_closed() {
                    if let (Some(first), Some(last)) = (vertices.first(), vertices.last()) {
                        let dx = last.location.x - first.location.x;
                        let dy = last.location.y - first.location.y;
                        cut_length += (dx * dx + dy * dy).sqrt();
                    }
                }
            },

            EntityType::LwPolyline(lwpolyline) => {
                entity_counts.lwpolylines += 1;
                let vertices = &lwpolyline.vertices;
                for i in 1..vertices.len() {
                    let dx = vertices[i].x - vertices[i-1].x;
                    let dy = vertices[i].y - vertices[i-1].y;
                    cut_length += (dx * dx + dy * dy).sqrt();
                }

                if lwpolyline.is_closed() {
                    if let (Some(first), Some(last)) = (vertices.first(), vertices.last()) {
                        let dx = last.x - first.x;
                        let dy = last.y - first.y;
                        cut_length += (dx * dx + dy * dy).sqrt();
                    }
                }
            },

            EntityType::Ellipse(ellipse) => {
                entity_counts.ellipses += 1;
                let major_length = (ellipse.major_axis.x.powi(2) +
                                  ellipse.major_axis.y.powi(2)).sqrt();
                let a = major_length / 2.0;
                let b = a * ellipse.minor_axis_ratio;
                let h = ((a - b) * (a - b)) / ((a + b) * (a + b));
                cut_length += PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt()));
            },

            _ => continue,
        }
    }

    Ok(CalculationResult {
        total_length: cut_length,
        entity_counts,
        calculation_time: start_time.elapsed(),
    })
}

// Вспомогательная функция для форматирования результатов
pub fn format_calculation_results(result: &CalculationResult) -> Vec<String> {
    vec![
        format!("Общая длина: {:.2} мм", result.total_length),
        format!("Время расчета: {:?}", result.calculation_time),
        format!("Количество элементов:"),
        format!("- Линий: {}", result.entity_counts.lines),
        format!("- Дуг: {}", result.entity_counts.arcs),
        format!("- Окружностей: {}", result.entity_counts.circles),
        format!("- Полилиний: {}", result.entity_counts.polylines),
        format!("- LWПолилиний: {}", result.entity_counts.lwpolylines),
        format!("- Эллипсов: {}", result.entity_counts.ellipses),
    ]
}