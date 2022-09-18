use macroquad::prelude::*;

// Çarpışma kontrolü yapan fonksiyon
// Burada aabb çarpışma tekniği kullanılmakta
// aabb = Axis-Aligned Bounding Box
// Rect tipi üstündeki intersect fonksiyonu bunu otomatik olarak icra ediyor
pub fn in_collision(rect1: &mut Rect, rect2: &Rect, velocity: &mut Vec2) -> bool {
    let intersection = match rect1.intersect(*rect2) {
        Some(intersection) => intersection,
        None => return false,
    };
    let rect1_center = rect1.point() + rect1.size() * 0.5;
    let rect2_center = rect2.point() + rect2.size() * 0.5;
    let to = rect2_center - rect1_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            rect1.y -= to_signum.y * intersection.h;
            velocity.y = -to_signum.y * velocity.y.abs();
        }
        false => {
            rect1.x -= to_signum.x * intersection.w;
            velocity.x = -to_signum.x * velocity.x.abs();
        }
    }
    true
}
