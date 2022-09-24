pub fn calculate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let a = (x1 - x2).powi(2);
    let b = (y1 - y2).powi(2);
    let c = (a + b).sqrt();
    //println!("Px,Py = {}:{}, Ex,Ey = {}:{}, Mesafe {}", x1, y1, x2, y2, c);
    c
}
