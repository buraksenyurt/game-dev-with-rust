#[derive(Clone, Default)]
pub struct Question {
    pub description: String,
    pub expected_answer: String,
}

impl Question {
    pub fn new(description: String, expected_answer: String) -> Self {
        Self {
            description,
            expected_answer,
        }
    }
}
