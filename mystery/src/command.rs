// Oyuncunun terminalden girdiği komutları tanımlayan enum veri yapısı.
// Sola dön, aynaya bak, merdiveni tırman, şurubu iç gibi ifadeleri temsil etmeye çalışacağız.
#[derive(Debug, PartialEq)]
pub enum Command {
    Walk(String),
    Look(String),
    GetUp(String),
    Unknown(String),
    Quit,
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;
    use crate::controller::parse;

    #[test]
    fn should_parse_works() {
        let input = "Ayağa kalk";
        let cmd = parse(input);
        assert_eq!(cmd, Command::GetUp("ayağa".to_string()));

        let input = "kapıya yürü";
        let cmd = parse(input);
        assert_eq!(cmd, Command::Walk("kapıya".to_string()));
    }
}
