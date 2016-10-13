#[macro_export]
macro_rules! c_i {
    ($coordinate: expr) => ({
        use std::str;
        if let Some(index) = $coordinate.iter().position(|&c|
            !(b'0'..b'9').contains(c)
        ) {
            let next: &[u8] = &$coordinate[{index+1}..];
            let term: &[u8] = &$coordinate[..index];
            if let Ok(i) = unsafe {
                u16::from_str_radix(str::from_utf8_unchecked(term), 10)
            } {
                Some((next, i))
            }
            else {
                None
            }
        } else {
            None
        }
    });
}

#[macro_export]
macro_rules! c_xy {
    ($coordinate: expr) => ({
        use std::str;
        if let Some(index) = $coordinate.iter().position(|&sep| sep.eq(&b';')) {
            let (term_x, term_y): (&[u8], &[u8]) = $coordinate.split_at(index);
            let len_y: usize = term_y.iter().position(|&c| {
                if (b'0'..b'9').contains(c) {
                    false
                } else {
                    true
                }
            }).unwrap_or_default();
            let next: &[u8] = &term_y[{len_y+2}..];
            let term_y: &[u8] = &term_y[1..{len_y+2}];
            if let (Ok(x), Ok(y)) = unsafe {(
                u16::from_str_radix(str::from_utf8_unchecked(term_x), 10),
                u16::from_str_radix(str::from_utf8_unchecked(term_y), 10)
            )} {
                Some((next, (x, y)))
            } else {
                None
            }
        } else {
            None
        }
    });
}
