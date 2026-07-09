use std::io::{self, Write};

/// Writes the visual representation of `byte` to `writer` following
/// BSD `vis(3)` semantics (the default mode used by `cat -v`).
///
/// - Non-ASCII bytes (0x80-0xff) are prefixed with `M-`.
/// - Control characters (0x00-0x1f) are rendered as `^@`..`^_`.
/// - DEL (0x7f) is rendered as `^?`.
/// - All other bytes pass through unchanged.
///
/// Returns `Ok(())` on success or a write error.
pub fn write_vis_byte(writer: &mut impl Write, byte: u8) -> io::Result<()> {
    if !byte.is_ascii() {
        writer.write_all(b"M-")?;
        let stripped = byte & 0x7f;
        if stripped < 0x20 || stripped == 0x7f {
            let c = if stripped == 0x7f {
                b'?'
            } else {
                stripped | 0x40
            };
            writer.write_all(b"^")?;
            writer.write_all(&[c])?;
        } else {
            writer.write_all(&[stripped])?;
        }
    } else if byte < 0x20 || byte == 0x7f {
        let c = if byte == 0x7f { b'?' } else { byte | 0x40 };
        writer.write_all(b"^")?;
        writer.write_all(&[c])?;
    } else {
        writer.write_all(&[byte])?;
    }
    Ok(())
}
