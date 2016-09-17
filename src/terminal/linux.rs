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
                libc::ioctl(libc::STDIN_FILENO, 0x00005402, &termios);
            };
        }
        None => (),
    }
}

pub fn setup_terminal(fd: libc::c_int) -> Result<termios> {
    let termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(fd, 0x00005401, &termios);
    }
    unsafe {
        termios_to_restore = Some(termios);
        libc::atexit(restore_termios);
    };
    enter_raw_mode(fd).unwrap();
    let winsize: winsize = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(fd, libc::TIOCSWINSZ, winsize);
    }
    Ok(termios)
}

fn enter_raw_mode(fd: libc::c_int) -> Result<()> {
    let mut new_termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe {
        libc::ioctl(fd, 0x00005401, &new_termios);
    }
    new_termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
    new_termios.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    new_termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
    new_termios.c_cflag |= libc::CS8;
    new_termios.c_oflag &= !(libc::OPOST);
    new_termios.c_cc[libc::VMIN] = 1;
    new_termios.c_cc[libc::VTIME] = 0;
    unsafe {
        libc::ioctl(fd, 0x00005402, &new_termios);
    }
    io::stdout().write(b"\x1b[?1002h\x1b[?1015h\x1b[?1006h").unwrap(); // MOUSE ON
    Ok(())
}
