use crate::entity::Shuttle;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

#[derive(PartialEq)]
pub struct LandingPlatform {
    pub p1: Point,
    pub p2: Point,
    pub left_leg: Point,
    pub right_leg: Point,
}

impl LandingPlatform {
    pub fn new(p1: Point, p2: Point, left_leg: Point, right_leg: Point) -> Self {
        Self {
            p1,
            p2,
            left_leg,
            right_leg,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        canvas.draw_line(self.p1, self.p2)?;
        canvas.draw_line(self.p1, self.left_leg)?;
        canvas.draw_line(self.p2, self.right_leg)?;

        Ok(())
    }
    fn get_top_edge(&self) -> (Point, Point) {
        (self.p1, self.p2)
    }
    pub fn check_collision(&self, shuttle: &Shuttle) -> bool {
        let ((shuttle_left_foot, _), (shuttle_right_foot, _)) = shuttle.calculate_foot_points();
        // println!("Shuttle:{:?}:{:?}", shuttle_left_foot, shuttle_right_foot);
        let (platform_start, platform_end) = self.get_top_edge();
        let feet_above_platform = shuttle_left_foot.y >= platform_start.y - 1
            && shuttle_right_foot.y <= platform_start.y + 1;
        let feet_within_platform =
            shuttle_left_foot.x >= platform_start.x && shuttle_right_foot.x <= platform_end.x;

        feet_above_platform && feet_within_platform
    }
}
