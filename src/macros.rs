#[macro_export]
macro_rules! parse_number {
    ($coordinate: expr) => ({
        use std::str;
        let index: usize = $coordinate.iter().position(|&c|
            !(b'0'..b'9').contains(c)
        ).unwrap_or(
            $coordinate.len()
        );
        let term: &[u8] = &$coordinate[..index];
        if let Ok(number) = unsafe {
            u16::from_str_radix(str::from_utf8_unchecked(term), 10)
        } {
            let next: &[u8] = &$coordinate[index..];
            Some((next.first(), number, next))
        }
        else {
            None
        }
    });
}
