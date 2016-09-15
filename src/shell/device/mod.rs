mod state;
mod control;

use std::io::{self, Read};
use std::thread;
use std::os::unix::io::AsRawFd;

use ::chan;
use ::libc;
use ::sig;
use ::pty::prelude as pty;

pub use self::control::Control;
pub use self::state::DeviceState;

pub type In = [libc::c_uchar; 12];
pub type Out = [libc::c_uchar; 4096];
pub type Sig = libc::c_int;

/// The struct `Device` is the input/output terminal interface.

pub struct Device {
  speudo: pty::Master,
  input: chan::Receiver<(In, libc::size_t )>,
  output: chan::Receiver<(Out, libc::size_t)>,
  signal: chan::Receiver<Sig>,
}

impl Device {

  /// The constructor method `new` returns a Device interface iterable.
  fn new (
    speudo: pty::Master,
    input: chan::Receiver<(In, libc::size_t)>,
    output: chan::Receiver<(Out, libc::size_t)>,
    signal: chan::Receiver<libc::c_int>,
  ) -> Self {
    ::terminal::setup_terminal(speudo);
    Device {
      speudo: speudo,
      input: input,
      output: output,
      signal: signal,
    }
  }

  pub fn from_speudo(mut master: pty::Master) -> Self {
    let (tx_out, rx_out) = chan::sync(0);
    let (tx_in, rx_in) = chan::sync(0);
    let (tx_sig, rx_sig) = chan::sync(0);

    thread::spawn(move || {
      let mut bytes: In = [0; 12];

      while let Some(read) = io::stdin().read(&mut bytes).ok() {
        tx_in.send((bytes, read));
      }
    });
    thread::spawn(move || {
      let mut bytes: Out = [0u8; 4096];

      while let Some(read) = master.read(&mut bytes).ok() {
        tx_out.send((bytes, read));
      }
    });
    thread::spawn(move || {
      static mut signal: Option<Sig> = None;

      unsafe extern "C" fn event(sig: Sig) {
        signal = Some(sig);
      }
      signal!(sig::ffi::Sig::WINCH, event);
      signal!(sig::ffi::Sig::CHLD, event);
      unsafe {
        loop {
          if let Some(sig) = signal {
            tx_sig.send(sig);
            signal = None;
          }
        }
      }
    });
    Device::new(master, rx_in, rx_out, rx_sig)
  }
}

impl io::Write for Device {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.speudo.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.speudo.flush()
  }
}

impl Drop for Device {
  fn drop(&mut self) {
    unsafe {
      if libc::close(self.speudo.as_raw_fd()) == -1 {
        unimplemented!()
      }
    }
  }
}

impl Iterator for Device {
  type Item = DeviceState;

  fn next(&mut self) -> Option<DeviceState> {
    let ref input: chan::Receiver<(In, libc::size_t)> = self.input;
    let ref output: chan::Receiver<(Out, libc::size_t)> = self.output;
    let ref signal: chan::Receiver<Sig> = self.signal;

    chan_select! {
      default => return Some(DeviceState::from_idle()),
      signal.recv() -> val => return match val {
        Some(sig) => Some(DeviceState::from_sig(sig)),
        None => None,
      },
      input.recv() -> val => return match val {
        Some((buf, len)) => Some(DeviceState::from_in(buf, len)),
        None => None,
      },
      output.recv() -> val => return match val {
        Some((buf, len @ 1 ... 4096)) => Some(DeviceState::from_out(buf, len)),
        _ => None,
      },
    }
  }
}
