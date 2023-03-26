pub struct Noun {
    pub name: String,
    pub real: String,
}

impl Noun {
    pub fn new(name: String, real: String) -> Self {
        Self { name, real }
    }
}
