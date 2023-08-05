pub struct Terminal;

impl Terminal {
    pub fn print_intro() {
        let line = "*************************";
        let empty_line = "**                     **";
        let lines = vec![
            line,
            empty_line,
            empty_line,
            "**      MİNİ QUİZ      **",
            "**   Developed By BSŞ  **",
            empty_line,
            "**        2023         **",
            empty_line,
            empty_line,
            line,
        ];
        for line in lines {
            println!("{}", line);
        }
    }
}
