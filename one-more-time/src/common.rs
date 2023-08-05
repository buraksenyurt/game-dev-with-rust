use crate::enums::DonutType;

pub fn get_donut_cost(donut_type: DonutType) -> f32 {
    match donut_type {
        DonutType::Blue => 25.,
        DonutType::White => 50.,
        DonutType::Red => 125.,
    }
}

pub fn get_donut_penalty_cost(donut_type: DonutType) -> f32 {
    match donut_type {
        DonutType::Blue => 12.5,
        DonutType::White => 25.,
        DonutType::Red => 67.5,
    }
}

pub fn get_file_name(donut_type: DonutType) -> String {
    match donut_type {
        DonutType::Blue => "customer_blue.png".to_string(),
        DonutType::White => "customer_white.png".to_string(),
        DonutType::Red => "customer_red.png".to_string(),
    }
}
