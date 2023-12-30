use std::io::{self, stdout, Write};
use nick_shell::commands::command_cli;

fn main() {
    loop {
        print!(" => ");

        let _ = stdout().flush();

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprint!("{}", e);
                continue;
            }
        }

        command_cli(&input);
    }
}
