use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

use puffyrs::{flags::FlagParser, io as pio};

fn main() {
    let parsed = FlagParser::new()
        .bool('b', false)
        .bool('e', false)
        .bool('n', false)
        .bool('s', false)
        .bool('t', false)
        .bool('u', false)
        .bool('v', false)
        .parse(env::args());

    let bflag = parsed.bool('b');
    let eflag = parsed.bool('e');
    let nflag = parsed.bool('n') || bflag;
    let sflag = parsed.bool('s');
    let tflag = parsed.bool('t');
    let _uflag = parsed.bool('u');
    let vflag = parsed.bool('v') || eflag || tflag;
    let need_cook = bflag || eflag || nflag || sflag || tflag || vflag;

    let files = parsed.args();
    let mut rval = 0;

    if files.is_empty() {
        if need_cook {
            let stdin = io::stdin();
            rval = cook_buf(
                BufReader::new(stdin.lock()),
                "stdin",
                bflag,
                eflag,
                nflag,
                sflag,
                tflag,
                vflag,
            );
        } else {
            rval = raw_cat(&mut io::stdin().lock(), "stdin");
        }
    } else {
        for file in files {
            let r = if file == "-" {
                if need_cook {
                    let stdin = io::stdin();
                    cook_buf(
                        BufReader::new(stdin.lock()),
                        "stdin",
                        bflag,
                        eflag,
                        nflag,
                        sflag,
                        tflag,
                        vflag,
                    )
                } else {
                    raw_cat(&mut io::stdin().lock(), "stdin")
                }
            } else {
                match File::open(file) {
                    Ok(mut f) => {
                        if need_cook {
                            cook_buf(
                                BufReader::new(f),
                                file,
                                bflag,
                                eflag,
                                nflag,
                                sflag,
                                tflag,
                                vflag,
                            )
                        } else {
                            raw_cat(&mut f, file)
                        }
                    }
                    Err(e) => {
                        pio::warn(&format!("{}: {}", file, e));
                        1
                    }
                }
            };
            if r != 0 {
                rval = 1;
            }
        }
    }

    if io::stdout().flush().is_err() {
        pio::die(1, "stdout");
    }

    process::exit(rval);
}

fn raw_cat(reader: &mut dyn Read, filename: &str) -> i32 {
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut buf = [0u8; 65536];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => return 0,
            Ok(n) => {
                let mut off = 0;
                let mut remaining = n;
                while remaining > 0 {
                    match writer.write(&buf[off..off + remaining]) {
                        Ok(0) => pio::die(1, "stdout"),
                        Ok(w) => {
                            off += w;
                            remaining -= w;
                        }
                        Err(_) => pio::die(1, "stdout"),
                    }
                }
            }
            Err(e) => {
                pio::warn(&format!("{}: {}", filename, e));
                return 1;
            }
        }
    }
}

fn cook_buf(
    reader: impl BufRead,
    filename: &str,
    bflag: bool,
    eflag: bool,
    nflag: bool,
    sflag: bool,
    tflag: bool,
    vflag: bool,
) -> i32 {
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut line: u64 = 0;
    let mut gobble = 0;
    let mut prev = b'\n';
    let mut read_err = None;

    let mut bytes = reader.bytes();
    loop {
        let mut ch = match bytes.next() {
            Some(Ok(b)) => b,
            Some(Err(e)) => {
                read_err = Some(e);
                break;
            }
            None => break,
        };

        if prev == b'\n' {
            if sflag {
                if ch == b'\n' {
                    if gobble != 0 {
                        prev = ch;
                        continue;
                    }
                    gobble = 1;
                } else {
                    gobble = 0;
                }
            }
            if nflag {
                if !bflag || ch != b'\n' {
                    if write!(writer, "{:6}\t", line + 1).is_err() {
                        pio::die(1, "stdout");
                    }
                    line += 1;
                } else if eflag {
                    if write!(writer, "{:6}\t", "").is_err() {
                        pio::die(1, "stdout");
                    }
                }
            }
        }

        if ch == b'\n' {
            if eflag {
                if writer.write_all(b"$").is_err() {
                    pio::die(1, "stdout");
                }
            }
        } else if ch == b'\t' {
            if tflag {
                if writer.write_all(b"^I").is_err() {
                    pio::die(1, "stdout");
                }
                prev = ch;
                continue;
            }
        } else if vflag {
            if !ch.is_ascii() {
                if writer.write_all(b"M-").is_err() {
                    pio::die(1, "stdout");
                }
                ch &= 0x7f;
            }
            if ch < 0x20 || ch == 0x7f {
                let c = if ch == 0x7f { b'?' } else { ch | 0x40 };
                if writer.write_all(b"^").is_err() || writer.write_all(&[c]).is_err() {
                    pio::die(1, "stdout");
                }
                prev = ch;
                continue;
            }
        }

        if writer.write_all(&[ch]).is_err() {
            pio::die(1, "stdout");
        }
        prev = ch;
    }

    if let Some(e) = read_err {
        pio::warn(&format!("{}: {}", filename, e));
        return 1;
    }

    if writer.flush().is_err() {
        pio::die(1, "stdout");
    }

    0
}
