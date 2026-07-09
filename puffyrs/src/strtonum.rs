/// OpenBSD `strtonum(3)`: parse a string to a bounded `i64`.
/// Returns `Err("too small")`, `Err("too large")`, or `Err("invalid")`.
pub fn strtonum(s: &str, min: i64, max: i64) -> Result<i64, &'static str> {
    let s = s.trim();
    if s.is_empty() {
        return Err("invalid");
    }
    match s.parse::<i64>() {
        Ok(n) if n < min => Err("too small"),
        Ok(n) if n > max => Err("too large"),
        Ok(n) => Ok(n),
        Err(_) => Err("invalid"),
    }
}
