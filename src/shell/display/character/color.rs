use ::libc;

pub const BLACK: [libc::c_uchar; 3] = [0, 0, 0];
pub const RED: [libc::c_uchar; 3] = [255, 0, 0];
pub const YELLOW: [libc::c_uchar; 3] = [255, 255, 0];
pub const GREEN: [libc::c_uchar; 3] = [0, 255, 0];
pub const CYAN: [libc::c_uchar; 3] = [0, 255, 255];
pub const BLUE: [libc::c_uchar; 3] = [0, 0, 255];
pub const MAGENTA: [libc::c_uchar; 3] = [255, 0, 255];
pub const WHITE: [libc::c_uchar; 3] = [255, 255, 255];

pub const DEFAULT_FOREGROUND: [libc::c_uchar; 3] = BLACK;
pub const DEFAULT_BACKGROUND: [libc::c_uchar; 3] = WHITE;
