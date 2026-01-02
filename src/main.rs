use std::io;
use std::io::Write;
use std::env;
use std::process::Command as ProcCommand;

fn expand_path(path: &str) -> String {
    if path == "~" {
        env::var("HOME").unwrap()
    } else if let Some(rest) = path.strip_prefix("~/") {
        format!("{}/{}", env::var("HOME").unwrap(), rest)
    } else {
        path.to_string()
    }
}

fn cd(path: &str) {
    let expanded = expand_path(path);

    if let Err(e) = env::set_current_dir(&expanded) {
        eprintln!("cd: {}", e);
    }
}

fn run_external(parts: &[&str]) {
    let child = ProcCommand::new(parts[0])
        .args(&parts[1..])
        .spawn();

    match child {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("rush: {}", e);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut pwd;
    let mut cmd= String::new();

    loop{
        pwd = env::current_dir().unwrap();
        cmd.clear();
        print!("{} $ ", pwd.display());
        io::stdout().flush().unwrap();
        stdin.read_line(&mut cmd).expect("No line detected");
        cmd = cmd.trim().to_string();
        if cmd == "exit" {
            println!("Good bye!");
            break;
        }
        let cmd : Vec<&str> = cmd.split(" ").collect();

        if cmd[0] == "cd" {
            if cmd.len() < 2 {
                eprintln!("cd: missing operand");
            } else {
                cd(cmd[1]);
            }
            continue;
        }
        else {
            run_external(&cmd);
        }
    }

}