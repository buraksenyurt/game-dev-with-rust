// Oyuncunun terminalden girdiği komutları tanımlayan enum veri yapısı.
// Sola dön, aynaya bak, merdiveni tırman, şurubu iç gibi ifadeleri temsil etmeye çalışacağız.
#[derive(Debug, PartialEq)]
pub enum Command {
    GetUp,
    Jump(String),
    Look(String),
    Unknown(String),
    Quit,
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command;
    use crate::controller::parse;

    #[test]
    fn should_parse_works() {
        let input = "Etrafa bak";
        let cmd = parse(input);
        assert_eq!(cmd, Command::Look("etrafa".to_string()));

        let input = "kapıya sıçra";
        let cmd = parse(input);
        assert_eq!(cmd, Command::Jump("kapıya".to_string()));
    }
}
