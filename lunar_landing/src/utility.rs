use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn find_ground_height(points: &[Point], x: i32) -> i32 {
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if x >= p1.x && x <= p2.x {
            return interpolate_height(&[p1, p2], x);
        }
    }
    0
}

fn interpolate_height(points: &[Point], x: i32) -> i32 {
    if x <= points.first().unwrap().x {
        return points.first().unwrap().y;
    }
    if x >= points.last().unwrap().x {
        return points.last().unwrap().y;
    }

    for window in points.windows(2) {
        let (p1, p2) = (window[0], window[1]);
        if x >= p1.x && x <= p2.x {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            return p1.y + (x - p1.x) * dy / dx;
        }
    }
    points.first().unwrap().y
}

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let hex_value = u32::from_str_radix(hex, 16).unwrap();

    let r = ((hex_value >> 24) & 255) as u8;
    let g = ((hex_value >> 16) & 255) as u8;
    let b = ((hex_value >> 8) & 255) as u8;
    let a = (hex_value & 255) as u8;

    Color::RGBA(r, g, b, a)
}
