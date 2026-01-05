// Copyright 2026 Saransh Halwai
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// rush shell entry point
// main.rs

use std::env;
use std::io;
use std::io::IsTerminal;
use std::io::Write;
use std::process::Command as ProcCommand;

struct Style {
    enabled: bool,
}

impl Style {
    fn paint(&self, s: &str, code: &str) -> String {
        if self.enabled {
            format!("\x1b[{}m{}\x1b[0m", code, s)
        } else {
            s.to_string()
        }
    }

    fn red(&self, s: &str) -> String {
        self.paint(s, "31")
    }

    fn blue(&self, s: &str) -> String {
        self.paint(s, "34")
    }

    fn bold(&self, s: &str) -> String {
        self.paint(s, "1")
    }
}

fn expand_path(path: &str) -> String {
    if path == "~" {
        env::var("HOME").unwrap()
    } else if let Some(rest) = path.strip_prefix("~/") {
        format!("{}/{}", env::var("HOME").unwrap(), rest)
    } else {
        path.to_string()
    }
}

fn cd(path: &str, style: &Style) {
    let expanded = expand_path(path);

    if let Err(e) = env::set_current_dir(&expanded) {
        eprintln!("{}", style.red(&format!("cd: {}", e)));
    }
}

fn run_external(parts: &[&str], style: &Style) {
    let child = ProcCommand::new(parts[0]).args(&parts[1..]).spawn();

    match child {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("{}", style.red(&format!("rush: {}", e)));
        }
    }
}

fn main() {
    let use_color = io::stdout().is_terminal();
    let stdin = io::stdin();
    let style = Style { enabled: use_color };
    let mut pwd;
    let mut cmd = String::new();

    loop {
        pwd = env::current_dir().unwrap();
        cmd.clear();
        let pwd_str = pwd.display().to_string();
        let prompt = format!("{} {} ", style.blue(&pwd_str), style.bold("$"));

        print!("{prompt}");

        io::stdout().flush().unwrap();
        stdin.read_line(&mut cmd).expect("No line detected");
        cmd = cmd.trim().to_string();
        if cmd == "exit" {
            println!("{}", style.blue("Goodbye."));
            break;
        }
        let cmd: Vec<&str> = cmd.split(" ").collect();

        if cmd[0] == "cd" {
            if cmd.len() < 2 {
                eprintln!("cd: missing operand");
            } else {
                cd(cmd[1], &style);
            }
            continue;
        } else {
            run_external(&cmd, &style);
        }
    }
}
