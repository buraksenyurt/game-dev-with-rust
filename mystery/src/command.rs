use std::str::FromStr;

// Oyuncunun terminalden girdiği komutları tanımlayan enum veri yapısı.
// Sola dön, aynaya bak, merdiveni tırman, şurubu iç gibi ifadeleri temsil etmeye çalışacağız.
#[derive(Debug, PartialEq)]
pub enum Command {
    GetUp,
    Jump(String),
    Look(String),
    Unknown(String),
    Talk(String),
    Get(String),
    Give(String),
    Drop(String),
    Inventory(String),
    Quit,
}

// Command nesnesinin üretimini bir String'den gerçekleştirmek için eklendi
impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lovercase = input.to_lowercase();
        let mut parts = lovercase.split_whitespace();
        let noun = parts.next().unwrap_or_default().to_string();
        let verb = parts.next().unwrap_or_default().to_string();

        match verb.as_str() {
            "bak" => Ok(Command::Look(noun)),
            "git" => Ok(Command::Jump(noun)),
            "kalk" => Ok(Command::GetUp),
            "sor" => Ok(Command::Talk(noun)),
            "al" => Ok(Command::Get(noun)),
            "ver" => Ok(Command::Give(noun)),
            "bırak" => Ok(Command::Drop(noun)),
            "listele" => Ok(Command::Inventory(noun)),
            "bitir" => Ok(Command::Quit),
            _ => Ok(Command::Unknown(input.trim().to_string())),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;
    use crate::controller::parse;
    use std::str::FromStr;

    #[test]
    fn should_parse_works() {
        let input = "Etrafa bak";
        let cmd = Command::from_str(input).unwrap();
        assert_eq!(cmd, Command::Look("etrafa".to_string()));

        let input = "kapıya git";
        let cmd = Command::from_str(input).unwrap();
        assert_eq!(cmd, Command::Jump("kapıya".to_string()));
    }
}
