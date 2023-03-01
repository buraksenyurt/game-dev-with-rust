pub struct Pellets {
    pub count: u32,
    pub coordinates: Vec<(usize, usize)>,
}
impl Pellets {
    pub fn new(count: u32, coordinates: Vec<(usize, usize)>) -> Self {
        Self { count, coordinates }
    }
}
