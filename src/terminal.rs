use ::libc;
use std::os::unix::io::AsRawFd;
use std::io::Result;
use ::winsize;
use ::termios::*;

use ::pty::prelude as pty;

static mut termios_to_restore: Option<Termios> = None;
pub extern "C" fn restore_termios() {
    match unsafe { termios_to_restore } {
        Some(termios) => { let _ = tcsetattr(libc::STDIN_FILENO, TCSANOW, &termios); }
        None => ()
    }
}

pub fn setup_terminal(pty: pty::Master) -> Result<()> {
    let termios = try!(Termios::from_fd(libc::STDIN_FILENO));

    unsafe {
        termios_to_restore = Some(termios);
        libc::atexit(restore_termios);
    };

    enter_raw_mode(libc::STDIN_FILENO).unwrap();

    let winsize = try!(winsize::from_fd(libc::STDIN_FILENO));
    winsize::set(pty.as_raw_fd(), &winsize);

    Ok(())
}

fn enter_raw_mode(fd: libc::c_int) -> Result<()> {
    let mut new_termios = try!(Termios::from_fd(fd));

    new_termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    new_termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    new_termios.c_cflag &= !(CSIZE | PARENB);
    new_termios.c_cflag |= CS8;
    new_termios.c_oflag &= !(OPOST);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;

    unsafe {
        tcsetattr(libc::STDIN_FILENO, TCSANOW, &new_termios);
    }
    Ok(())
}
