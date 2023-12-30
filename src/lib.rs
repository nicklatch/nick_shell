pub mod commands {
    use std::{
        env,
        path::Path,
        process::{Child, Command, Stdio},
        str::SplitAsciiWhitespace,
    };

    pub fn command_cli(input: &String) {
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

                    previous_command = match output {
                        Ok(output) => Some(output),
                        Err(e) => {
                            eprint!("{}", e);
                            return ();
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
