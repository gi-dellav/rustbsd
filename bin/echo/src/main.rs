use std::env;
use std::io::{self, Write};
use std::process;

fn escape(s: &str) -> i32 {
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            print!("{}", ch);
            continue;
        }

        match chars.next() {
            None => {
                print!("\\");
                return 0;
            }
            Some('\\') => print!("\\"),
            Some('a') => print!("\x07"),
            Some('b') => print!("\x08"),
            Some('c') => return -1,
            Some('e') => print!("\x1b"),
            Some('f') => print!("\x0c"),
            Some('n') => print!("\n"),
            Some('r') => print!("\r"),
            Some('t') => print!("\t"),
            Some('v') => print!("\x0b"),
            Some('0') => {
                let mut val: u8 = 0;
                for _ in 0..3 {
                    if let Some(&d) = chars.peek() {
                        if d >= '0' && d <= '7' {
                            val = val * 8 + (d as u8 - b'0');
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                print!("{}", val as char);
            }
            Some('x') => {
                if let Some(&d) = chars.peek() {
                    if d.is_ascii_hexdigit() {
                        let mut val: u8 = 0;
                        for _ in 0..2 {
                            if let Some(&h) = chars.peek() {
                                if h.is_ascii_hexdigit() {
                                    val *= 16;
                                    val += match h {
                                        '0'..='9' => h as u8 - b'0',
                                        'a'..='f' => h as u8 - b'a' + 10,
                                        'A'..='F' => h as u8 - b'A' + 10,
                                        _ => unreachable!(),
                                    };
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                        }
                        print!("{}", val as char);
                    } else {
                        print!("\\x");
                    }
                } else {
                    print!("\\x");
                }
            }
            Some(other) => {
                print!("\\{}", other);
            }
        }
    }

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nflag = false;
    let mut eflag = false;
    let mut idx = 1;

    while idx < args.len() {
        let arg = &args[idx];
        if arg.is_empty() || !arg.starts_with('-') {
            break;
        }

        let flags = &arg[1..];
        if flags.is_empty() {
            break;
        }

        let mut parsed = false;
        for ch in flags.chars() {
            match ch {
                'E' => {
                    eflag = false;
                    parsed = true;
                }
                'e' => {
                    eflag = true;
                    parsed = true;
                }
                'n' => {
                    nflag = true;
                    parsed = true;
                }
                _ => {
                    eflag = false;
                    nflag = false;
                    parsed = false;
                    break;
                }
            }
        }
        if !parsed {
            break;
        }
        idx += 1;
    }

    let mut first = true;
    while idx < args.len() {
        if !first {
            print!(" ");
        }
        first = false;

        if eflag {
            if escape(&args[idx]) != 0 {
                let _ = io::stdout().flush();
                process::exit(0);
            }
        } else {
            print!("{}", args[idx]);
        }
        idx += 1;
    }

    if !nflag {
        println!();
    }
    let _ = io::stdout().flush();
}
