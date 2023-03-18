use std::fmt::{Display, Formatter};

// Oyuncunun terminalden girdiği komutları tanımlayan veri yapısı.
// Sola dön, aynaya bak, merdiveni tırman, şurubu iç gibi ifadeleri temsil etmeye çalışacağız.
pub struct Command {
    pub verb: String,
    pub noun: String,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            verb: String::new(),
            noun: String::new(),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.noun, self.verb)
    }
}

impl Command {
    // girilen metni iki parça olarak ele alır ve command nesnesindeki alanlara yerleştirir.
    // Böylece bir komut ile eylemi ve eylemle ilgili konuyu yakalayabiliriz.
    pub fn parse(&mut self, input: &str) {
        let mut parts = input.trim().split_whitespace();
        self.noun = parts.next().unwrap_or_default().to_string();
        self.verb = parts.next().unwrap_or_default().to_string();
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;

    #[test]
    fn should_parse_works() {
        let mut cmd = Command::default();

        let input = "Kapıyı aç";
        cmd.parse(input);
        assert_eq!(cmd.noun, "Kapıyı");
        assert_eq!(cmd.verb, "aç");

        let input = "ışığı yak";
        cmd.parse(input);
        assert_eq!(cmd.noun, "ışığı");
        assert_eq!(cmd.verb, "yak");
    }
}
