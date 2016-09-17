use ::libc;
use ::libc::termios;
use ::libc::winsize;
use std::os::unix::io::AsRawFd;
use std::io::Result;

use ::pty::prelude as pty;

static mut termios_to_restore: Option<termios> = None;
#[cfg(target_os = "macos")]
pub extern "C" fn restore_termios() {
    match unsafe { termios_to_restore } {
        Some(termios) => { let _ = unsafe { libc::ioctl(libc::STDIN_FILENO, (0x80000000 | (116 << 8) | 20 | (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)), &termios); }; }
        None => ()
    }
}

#[cfg(target_os = "macos")]
pub fn setup_terminal(pty: pty::Master) -> Result<termios>
{ let termios: termios = unsafe { ::std::mem::zeroed() };
  unsafe { libc::ioctl(libc::STDIN_FILENO, (0x40000000 | (116 << 8) | 19 | (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)), &termios); }
  unsafe 
  { termios_to_restore = Some(termios);
    libc::atexit(restore_termios); };
  enter_raw_mode(libc::STDIN_FILENO).unwrap();
  let winsize: winsize = unsafe { ::std::mem::zeroed() };
  unsafe { libc::ioctl(libc::STDIN_FILENO, libc::TIOCSWINSZ, winsize); }
  Ok(termios) }

#[cfg(target_os = "macos")]
fn enter_raw_mode(fd: libc::c_int) -> Result<()> {
    let mut new_termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe { libc::ioctl(libc::STDIN_FILENO, (0x40000000 | (116 << 8) | 19 | (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)), &new_termios); }
    new_termios.c_lflag ^= !(libc::ICANON);
    new_termios.c_lflag ^= !(libc::ECHO);
    new_termios.c_cc[libc::VMIN] = 1;
    new_termios.c_cc[libc::VTIME] = 0;
    unsafe { libc::ioctl(libc::STDIN_FILENO, (0x80000000 | (116 << 8) | 20 | (((::std::mem::size_of::<termios>() & 0x1FFF) << 16) as u64)), &new_termios); }
    print!("\x1b[?1002h\x1b[?1015h\x1b[?1006h"); // MOUSE ON
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub extern "C" fn restore_termios() {
    match unsafe { termios_to_restore } {
        Some(termios) => { let _ = unsafe { libc::ioctl(libc::STDIN_FILENO, 0x00005402, &termios); }; }
        None => ()
    }
}

#[cfg(not(target_os = "macos"))]
pub fn setup_terminal(pty: pty::Master) -> Result<termios>
{ let termios: termios = unsafe { ::std::mem::zeroed() };
  unsafe { libc::ioctl(libc::STDIN_FILENO, 0x00005401, &termios); }
  unsafe 
  { termios_to_restore = Some(termios);
    libc::atexit(restore_termios); };
  enter_raw_mode(libc::STDIN_FILENO).unwrap();
  let winsize: winsize = unsafe { ::std::mem::zeroed() };
  unsafe { libc::ioctl(libc::STDIN_FILENO, libc::TIOCSWINSZ, winsize); }
  Ok(termios) }

#[cfg(not(target_os = "macos"))]
fn enter_raw_mode(fd: libc::c_int) -> Result<()> {
    let mut new_termios: termios = unsafe { ::std::mem::zeroed() };
    unsafe { libc::ioctl(libc::STDIN_FILENO, 0x00005401, &new_termios); }
    new_termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
    new_termios.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    new_termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
    new_termios.c_cflag |= libc::CS8;
    new_termios.c_oflag &= !(libc::OPOST);
    new_termios.c_cc[libc::VMIN] = 1;
    new_termios.c_cc[libc::VTIME] = 0;
    unsafe { libc::ioctl(libc::STDIN_FILENO, 0x00005402, &new_termios); }
    print!("\x1b[?1002h\x1b[?1015h\x1b[?1006h"); // MOUSE ON
    Ok(())
}
