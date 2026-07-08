use std::io::{self, Write};

pub fn write_escaped(writer: &mut impl Write, s: &str) -> io::Result<bool> {
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

pub fn unescape(s: &str) -> (String, bool) {
    let mut buf = Vec::with_capacity(s.len());
    match write_escaped(&mut buf, s) {
        Ok(true) => {
            let result = String::from_utf8(buf)
                .unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned());
            (result, true)
        }
        Ok(false) => {
            let result = String::from_utf8(buf)
                .unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned());
            (result, false)
        }
        Err(_) => (String::new(), true),
    }
}
