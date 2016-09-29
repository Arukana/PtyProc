use ::libc;
use ::libc::termios;
use ::libc::winsize;

use std::io::Result;
use std::io::{self, Write};

static mut termios_to_restore: Option<termios> = None;

pub extern "C" fn restore_termios() {
    match unsafe { termios_to_restore } {
        Some(termios) => {
            let _ = unsafe {
                libc::ioctl(libc::STDIN_FILENO,
                            (0x80000000 | (116 << 8) | 20 |
                             (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)),
                            &termios);
            };
        }
        None => (),
    }
}

pub fn setup_terminal(_: libc::c_int) -> Result<termios> {
    let termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(0,
                    (0x40000000 | (116 << 8) | 19 |
                     (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)),
                    &termios);
    }
    unsafe {
        termios_to_restore = Some(termios);
        libc::atexit(restore_termios);
    };
    enter_raw_mode(0).unwrap();
    let winsize: winsize = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(0, libc::TIOCSWINSZ, winsize);
    }
    Ok(termios)
}

fn enter_raw_mode(_: libc::c_int) -> Result<()> {
    let mut new_termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(0,
                    (0x40000000 | (116 << 8) | 19 |
                     (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)),
                    &new_termios);
    }
    new_termios.c_lflag ^= !(libc::ICANON);
    new_termios.c_lflag ^= !(libc::ECHO);
    new_termios.c_cc[libc::VMIN] = 1;
    new_termios.c_cc[libc::VTIME] = 0;
    unsafe {
        libc::ioctl(0,
                    (0x80000000 | (116 << 8) | 20 |
                     (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)),
                    &new_termios);
    }
    io::stdout().write(b"\x1b[?1002h\x1b[?1015h\x1b[?1006h"); // MOUSE ON
    Ok(())
}
