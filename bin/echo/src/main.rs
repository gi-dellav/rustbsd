use std::io::{self, Write};
use std::process;

use puffyrs::{escape, flags::FlagParser};

fn main() {
    let parsed = FlagParser::new()
        .bool('n', false)
        .bool('e', false)
        .bool('E', false)
        .parse(std::env::args());

    let no_newline = parsed.bool('n');
    let escape_enabled = parsed.bool('e') && !parsed.bool('E');

    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut first = true;

    for arg in parsed.args() {
        if !first {
            write!(writer, " ").ok();
        }
        first = false;

        if escape_enabled {
            match escape::write_escaped(&mut writer, arg) {
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
