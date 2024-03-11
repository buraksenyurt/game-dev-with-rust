use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

#[derive(PartialEq)]
pub struct Star {
    pub h_line: (Point, Point),
    pub v_line: (Point, Point),
    pub lc_line: (Point, Point),
    pub rc_line: (Point, Point),
}

impl Star {
    pub fn new(
        h_line: (Point, Point),
        v_line: (Point, Point),
        lc_line: (Point, Point),
        rc_line: (Point, Point),
    ) -> Self {
        Self {
            h_line,
            v_line,
            lc_line,
            rc_line,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(self.v_line.0, self.v_line.1)?;
        canvas.draw_line(self.h_line.0, self.h_line.1)?;
        canvas.draw_line(self.lc_line.0, self.lc_line.1)?;
        canvas.draw_line(self.rc_line.0, self.rc_line.1)?;
        Ok(())
    }
}
