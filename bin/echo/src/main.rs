use std::env;
use std::io::{self, Write};
use std::process;

fn write_escaped(writer: &mut impl Write, s: &str) -> io::Result<bool> {
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            write!(writer, "{ch}")?;
            continue;
        }

        let Some(next) = chars.next() else {
            write!(writer, "\\")?;
            return Ok(true);
        };

        match next {
            '\\' => write!(writer, "\\")?,
            'a' => write!(writer, "\x07")?,
            'b' => write!(writer, "\x08")?,
            'c' => return Ok(false),
            'e' => write!(writer, "\x1b")?,
            'f' => write!(writer, "\x0c")?,
            'n' => write!(writer, "\n")?,
            'r' => write!(writer, "\r")?,
            't' => write!(writer, "\t")?,
            'v' => write!(writer, "\x0b")?,
            '0' => {
                let mut oct = String::new();
                for _ in 0..3 {
                    match chars.as_str().as_bytes().first() {
                        Some(&b) if matches!(b, b'0'..=b'7') => {
                            oct.push(b as char);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if oct.is_empty() {
                    write!(writer, "\0")?;
                } else if let Ok(val) = u8::from_str_radix(&oct, 8) {
                    write!(writer, "{}", val as char)?;
                }
            }
            'x' => {
                let mut hex = String::new();
                for _ in 0..2 {
                    match chars.as_str().as_bytes().first() {
                        Some(&b) if b.is_ascii_hexdigit() => {
                            hex.push(b as char);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if hex.is_empty() {
                    write!(writer, "\\x")?;
                } else if let Ok(val) = u8::from_str_radix(&hex, 16) {
                    write!(writer, "{}", val as char)?;
                }
            }
            other => write!(writer, "\\{other}")?,
        }
    }

    Ok(true)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut no_newline = false;
    let mut escape = false;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg.is_empty() || !arg.starts_with('-') || arg == "-" {
            break;
        }

        let flags = &arg[1..];
        if flags.is_empty() {
            break;
        }

        let mut valid = true;
        for ch in flags.chars() {
            match ch {
                'E' => escape = false,
                'e' => escape = true,
                'n' => no_newline = true,
                _ => {
                    no_newline = false;
                    escape = false;
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            break;
        }
        i += 1;
    }

    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut first = true;

    for arg in &args[i..] {
        if !first {
            write!(writer, " ").ok();
        }
        first = false;

        if escape {
            match write_escaped(&mut writer, arg) {
                Ok(true) => {}
                Ok(false) => {
                    writer.flush().ok();
                    process::exit(0);
                }
                Err(_) => process::exit(1),
            }
        } else {
            write!(writer, "{arg}").ok();
        }
    }

    if !no_newline {
        writeln!(writer).ok();
    } else {
        writer.flush().ok();
    }
}
