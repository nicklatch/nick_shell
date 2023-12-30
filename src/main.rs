use std::{
    env,
    io::{self, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
    str::SplitAsciiWhitespace,
};

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

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(_command) = commands.next() {
            let mut parts: SplitAsciiWhitespace<'_> = input.trim().split_ascii_whitespace();
            let command: &str = match parts.next() {
                Some(s) => s,
                None => {
                    eprintln!("Oops...");
                    continue;
                }
            };
            let args: std::str::SplitAsciiWhitespace<'_> = parts;

            match command {
                "cd" => {
                    let new_dir: &str = args.peekable().peek().map_or("/", |x| x);
                    let root: &Path = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e)
                    }
                }
                "exit" | "EXIT" | "Exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprint!("{}", e);
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }
    }
}
