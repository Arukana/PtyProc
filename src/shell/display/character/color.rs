use ::libc;

pub const Black: [libc::c_uchar; 3] = [0, 0, 0];
pub const Red: [libc::c_uchar; 3] = [255, 0, 0];
pub const Yellow: [libc::c_uchar; 3] = [255, 255, 0];
pub const Green: [libc::c_uchar; 3] = [0, 255, 0];
pub const Cyan: [libc::c_uchar; 3] = [0, 255, 255];
pub const Blue: [libc::c_uchar; 3] = [0, 0, 255];
pub const Magenta: [libc::c_uchar; 3] = [255, 0, 255];
pub const White: [libc::c_uchar; 3] = [255, 255, 255];

pub const DEFAULT_FOREGROUND: [libc::c_uchar; 3] = Black;
pub const DEFAULT_BACKGROUND: [libc::c_uchar; 3] = White;
