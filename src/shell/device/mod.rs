pub mod control;
mod state;

use std::io::{self, Read};
use std::thread;

use ::chan;
use ::libc;
use ::pty::prelude as pty;

pub use self::state::DeviceState;

pub type In = [libc::c_uchar; 12];
pub type Out = [libc::c_uchar; 4096];
pub type Sig = libc::c_int;

/// The struct `Device` is the input/output terminal interface.

#[derive(Clone)]
pub struct Device {
  input: chan::Receiver<(In, libc::size_t)>,
  output: chan::Receiver<(Out, libc::size_t)>,
  signal: chan::Receiver<Sig>,
}

impl Device {

  /// The constructor method `new` returns a Device interface iterable.
  fn new (
    input: chan::Receiver<(In, libc::size_t)>,
    output: chan::Receiver<(Out, libc::size_t)>,
    signal: chan::Receiver<libc::c_int>,
  ) -> Self {
    Device {
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
      use std::sync::atomic::{AtomicI32, ATOMIC_I32_INIT, Ordering};
      use std::time::Duration;
      use std::ops::Div;

      static GOT_SIGNAL: AtomicI32 = ATOMIC_I32_INIT;

      unsafe fn event(sig: Sig) {
        GOT_SIGNAL.store(sig, Ordering::Release);
      }
      let duration: Duration = Duration::from_secs(1).div(2);
      unsafe {
        libc::signal(libc::SIGWINCH, event as libc::sighandler_t);
        loop {
          match GOT_SIGNAL.load(Ordering::Relaxed) {
            0 => thread::sleep(duration),
            sig => {
              GOT_SIGNAL.store(0, Ordering::Release);
              tx_sig.send(sig);
            },
          }
        }
      }
    });
    Device::new(rx_in, rx_out, rx_sig)
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
