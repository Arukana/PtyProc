pub const DEFAULT_REPEAT: libc::c_long = 1_000i64;
pub const DEFAULT_INTERVAL: libc::c_long = 1_000i64;
mod buf;

use std::fmt;
use std::mem;
use std::io::Write;
use std::ops::BitOr;
use std::ops::{Add, Sub, BitAnd, Not, DerefMut};

use ::libc;
use ::time;

pub use super::device::In;

use super::display::Display;
use super::device::control::Control;

#[cfg(feature = "task")]
pub use super::device::BufProc;
pub use super::device::{Out, DeviceState};
pub use super::device::control::operate::key::Key;
pub use super::device::control::operate::mouse::Mouse;

use self::buf::Buf;

fn catch_numbers<'a>(mut acc: Vec<libc::size_t>, buf: &'a [u8]) -> (Vec<libc::size_t>, &'a [u8])
{ match parse_number!(buf)
  { Some((number, &[b';', ref next..])) =>
      { acc.push(number);
        catch_numbers(acc, next) },
    Some((number, &[ref next..])) =>
      { acc.push(number);
        (acc, next) },
    _ =>
      { (acc, buf) }, }}

#[derive(Default, Copy, Clone)]
pub struct ShellState {
    /// The time limit required for a repetition.
    repeat: libc::c_long,
    /// The time limit required for a repetition.
    interval: libc::c_long,
    /// Update.
    idle: Option<()>,
    /// Signal.
    sig: Option<libc::c_int>,
    /// The pressed character.
    in_down: Option<Control>,
    /// The released character.
    in_up: Option<Control>,
    /// The number of the repetition.
    in_repeat: Option<libc::size_t>,
    /// The segment intervals.
    in_interval: Option<time::Tm>,
    /// The output of last text //printed.
    out_last: Option<(Out, libc::size_t)>,
    #[cfg(feature = "task")] task: Option<BufProc>,
    /// The tmp buffer
    buffer: Buf,
}

impl ShellState {

    /// The constructor method `new` returns a empty ShellState.
    #[cfg(feature = "task")]
    pub fn new (
        repeat: Option<libc::c_long>,
        interval: Option<libc::c_long>,
    ) -> Self {
        ShellState {
            repeat: repeat.unwrap_or(DEFAULT_REPEAT),
            interval: interval.unwrap_or(DEFAULT_INTERVAL),
            idle: None,
            sig: None,
            in_down: None,
            in_up: None,
            in_repeat: None,
            in_interval: None,
            out_last: None,
            buffer: Buf([0; 100], 0),
            task: None,
        }
    }

    /// The constructor method `new` returns a empty ShellState.
    #[cfg(not(feature = "task"))]
    pub fn new (
        repeat: Option<libc::c_long>,
        interval: Option<libc::c_long>,
    ) -> Self {
        ShellState {
            repeat: repeat.unwrap_or(DEFAULT_REPEAT),
            interval: interval.unwrap_or(DEFAULT_INTERVAL),
            idle: None,
            sig: None,
            in_down: None,
            in_up: None,
            in_repeat: None,
            in_interval: None,
            out_last: None,
            buffer: Buf([0; 100], 0),
        }
    }

    pub fn set_input_keyown(&mut self, key: char) {
        let mut buf: In = In::default();
        unsafe {
            let data: [libc::c_uchar; 4] = mem::transmute::<char, [libc::c_uchar; 4]>(key);

            buf[0] = data[0];
            buf[1] = data[1];
            buf[2] = data[2];
            buf[3] = data[3];
            self.in_down = Some(
                Control::new(buf, key.len_utf8())
            );
        }
    }

    /// The mutator method `set_repeat` change the time limit of repetition.
    pub fn set_repeat(&mut self, repeat: libc::c_long) {
        self.repeat = repeat;
    }

    /// The mutator method `set_interval` change the interval.
    pub fn set_interval(&mut self, interval: libc::c_long) {
        self.interval = interval;
    }

    /// The mutator method `set_idle` update the idle event status.
    pub fn set_idle(&mut self, entry: Option<()>) {
        self.idle = entry;
    }

    /// The mutator method `set_signal` update the signal
    /// and can resize the Display interface.
    pub fn set_signal(&mut self, signal: Option<libc::c_int>) {
        self.sig = signal;
    }

    /// The mutator method `set_input` update the `in_text`
    /// and save the old `in_text` to `in_text_past`.
    pub fn set_input(&mut self, out_screen: &mut Display, mut down: Option<Control>) {
        match down 
        { Some(is_mouse) =>
            { match is_mouse.as_slice()
              { &[b'\x1B', b'[', b'<', ref next..] =>
                  { let (bonjour, coucou) =
                    { catch_numbers(Vec::new(), next) };
                    match coucou
                    { &[b'M', ..] =>
                      { if out_screen.mouse().3 == false && bonjour[0] > 2
                        { down = None; }
                        else if out_screen.mouse().3 == false && out_screen.mouse().0 == false && out_screen.mouse().1 == false
                        { down = None; }
                        else if out_screen.mouse().3 == false && (out_screen.mouse().1 == true || out_screen.mouse().0 == true) && bonjour[0] == 0 {
                            let mut buf: In = In::default();

                            buf[0] = b'\x1B';
                            buf[1] = b'[';
                            buf[2] = b'M';
                            buf[3] = b' ';
                            buf[4] = bonjour[1] as u8 + 32;
                            buf[5] = bonjour[2] as u8 + 32;
                            down = Some(Control::new(buf, 6));
                        }
                        else if out_screen.mouse().3 == false && out_screen.mouse().0 == true
                        { down = None; }},
                      &[b'm', ..] =>
                      { if out_screen.mouse().0 == false && out_screen.mouse().1 == false
                        { down = None; }
                        else if out_screen.mouse().1 == true && out_screen.mouse().3 == false && bonjour[0] <= 2 {
                            let mut buf: In = In::default();

                            buf[0] = b'\x1B';
                            buf[1] = b'[';
                            buf[2] = b'M';
                            buf[3] = b'#';
                            buf[4] = bonjour[1] as u8 + 32;
                            buf[5] = bonjour[2] as u8 + 32;
                            down = Some(Control::new(buf, 6));
                        }
                        else if out_screen.mouse().0 == true
                        { down = None; }},
                      _ => {}, }},
               _ => {}, }},
          _ => {}, };
          if out_screen.ss()
          { let ss: libc::c_uchar = match down
            { Some(after) =>
              { match after.as_slice()
                { &[b'\x1B', b'[', b'A', ..] => b'A',
                  &[b'\x1B', b'[', b'B', ..] => b'B',
                  &[b'\x1B', b'[', b'C', ..] => b'C',
                  &[b'\x1B', b'[', b'D', ..] => b'D',
                  _ => 0, }},
              _ => 0, };
            if ss > 0 {
                let mut buf: In = In::default();

                buf[0] = b'\x1B';
                buf[1] = b'O';
                buf[2] = ss;
                down = Some(Control::new(buf, 3));
            }}

        self.in_down = down;
        if let Some(after) = down {
            if let Some(before) = self.in_up {
                if before.eq(&after).bitand(
                    before.as_time().add(
                        time::Duration::milliseconds(self.repeat)
                    ) >= after.as_time()
                ) {
                    self.in_repeat = Some(self.in_repeat.unwrap_or_default().add(&1));
                } else {
                    self.in_repeat = Some(0);
                }
            } else {
                self.in_interval = Some(after.as_time());
            }
            self.in_up = Some(after);
        } else if let Some(before) = self.in_up {
            if before.as_time().add(
               time::Duration::milliseconds(self.repeat)
            ) <= time::now() {
                self.in_repeat = None;
                self.in_interval = None;
            }
        }
    }

    /// The mutator method `set_output` update the both `out_text`
    /// and `out_screen` variable.
    pub fn set_output(&mut self, out_screen: &mut Display, entry: Option<(Out, libc::size_t)>) {
        if let Some((buf, len)) = entry {
            self.out_last = Some((buf, len));

            let mut tmp = [0u8; 596];
            let seeker = self.buffer.1;
            { let hey: &mut [u8] = self.buffer.deref_mut();
              if seeker > 0
              { { let mut buffer: &mut [u8] = &mut tmp[..];
                  buffer.write(&[b'\x1B']).unwrap(); }
                { let mut buffer: &mut [u8] = &mut tmp[1..];
                  buffer.write(&hey[..]).unwrap(); }
                { let mut buffer: &mut [u8] = &mut tmp[seeker..];
                  buffer.write(&buf[..len]).unwrap(); }}
              else
              { let mut buffer: &mut [u8] = &mut tmp[..];
                buffer.write(&buf[..len]).unwrap(); }
            }

            let _ = out_screen.write(&tmp[..len + self.buffer.1]);

            { let hey: &mut [u8] = self.buffer.deref_mut();
              { let mut buffer: &mut [u8] = &mut hey[..];
                let ss = [0u8; 100];
                buffer.write(&ss[..]).unwrap(); }}
            self.buffer.1 = 0;

            if (&buf[..len]).iter().position(|&i| i.eq(&b'\x1B')).is_some()
            { match (&buf[..len]).split(|&at| at == b'\x1B').take(len).last()
              { Some(get) =>
                  { if get.is_empty() || get.iter().position(|&i| i.eq(&b';').not().bitand(i.eq(&b'(').not()).bitand(i.eq(&b'[').not()).bitand(i.lt(&b'0').bitor(i.gt(&b'9')))).is_none()
                    { self.buffer.1 = get.len() + 1;
                      let mut coucou: &mut [u8] = self.buffer.deref_mut();
                      coucou.write(&get[..]).unwrap(); }},
                None => {}, }}

            else if len.ge(&3) && (&buf[..len]).iter().position(|&i| (i & 0b11110000 == 0b11110000) || (i & 0b11100000 == 0b11100000) || (i & 0b11000000 == 0b11000000)).is_some()
            { let hs = &buf[len-3..len];
              self.buffer.1 = 3;

              let mut coucou: &mut [u8] = self.buffer.deref_mut();
              if hs[2] & 0b11000000 == 0b11000000 || hs[2] & 0b11100000 == 0b11100000 || hs[2] & 0b11110000 == 0b11110000
              { coucou.write(&hs[2..]).unwrap(); }
              else if hs[1] & 0b11100000 == 0b11100000 || hs[1] & 0b11110000 == 0b11110000
              { coucou.write(&hs[1..]).unwrap(); }
              else if hs[0] & 0b11110000 == 0b11110000
              { coucou.write(&hs[0..]).unwrap(); }}

        } else {
            self.out_last = None;
        }
    }

    /// The mutator method `set_task` updates the task event.
    #[cfg(feature = "task")]
    pub fn set_task(&mut self, task: Option<BufProc>) {
        self.task = task;
    }

    /// The accessor method `is_idle` returns the Idle event.
    pub fn is_idle(&self) -> Option<()> {
        self.idle
    }

    /// The accessor method `is_signal` returns the Signal event.
    pub fn is_signal(&self) -> Option<libc::c_int> {
        self.sig
    }

    /// The accessor method `is_signal_resized` returns the Option for
    /// the WINCH Signal event.
    pub fn is_signal_resized(&self) -> Option<()> {
        if let Some(libc::SIGWINCH) = self.sig {
            Some(())
        } else {
            None
        }
    }

    /// The accessor method `is_input_keydown` returns the pressed Key event.
    pub fn is_input_keydown(&self) -> Option<Key> {
        if let Some(ref control) = self.in_down {
            control.is_key()
        } else {
            None
        }
    }

    /// The accessor method `is_input_keyrepeat` returns the number's repetition
    /// of the Key.
    pub fn is_input_keyrepeat(&self) -> Option<libc::size_t> {
        if let Some(_) = self.in_up {
            self.in_repeat
        } else {
            None
        }
    }

    /// The accessor method `is_input_keyinterval` returns the number's of repetition
    /// between a range of the interval.
    pub fn is_input_keyinterval(&self) -> Option<i64> {
        if let (Some(first), Some(last)) = (self.in_interval, self.in_down) {
            Some(
                first.sub(last.as_time()).num_milliseconds().abs()/self.interval
            )
        } else {
            None
        }
    }

    /// The accessor method `is_input_mouse` returns the pressed Mouse event.
    pub fn is_input_mouse(&self) -> Option<(Mouse, libc::c_ushort, libc::c_ushort)> {
        if let Some(ref control) = self.in_down {
            control.is_mouse()
        } else {
            None
        }
    }

    /// The accessor method `is_in_slice` returns the bytes for a Input event.
    pub fn is_input_slice(&self) -> Option<&[libc::c_uchar]> {
        if let Some(ref control) = self.in_down {
            Some(control.as_slice())
        } else {
            None
        }
    }

    /// The accessor method `is_output_last` returns the Output text event.
    pub fn is_output_last(&self) -> Option<&[libc::c_uchar]> {
        if let Some((ref out, len)) = self.out_last {
            Some(&out[..len])
        } else {
            None
        }
    }

    /// The accessor method `is_output_screen` returns the Output screen event.
    pub fn is_output_screen(&self) -> Option<()> {
        if self.is_output_last().is_some().bitor(
            self.is_signal_resized().is_some()
        ) {
            Some(())
        } else {
            None
        }
    }

    /// The mutator method `set_task` updates the task event.
    #[cfg(feature = "task")]
    pub fn is_task(&self) -> Option<&BufProc> {
        if let Some(ref task) = self.task {
            Some(task)
        } else {
            None
        }
    }

    /// The method `with_device` updates the state from
    /// the event DeviceState interface.
    #[cfg(feature = "task")]
    pub fn clone_from(&mut self, out_screen: &mut Display, event: DeviceState) {
        self.set_task(event.is_task());
        self.set_idle(event.is_idle());
        self.set_signal(event.is_signal());
        self.set_output(out_screen, event.is_out_text());
        self.set_input(out_screen, event.is_input());
    }

    /// The method `with_device` updates the state from
    /// the event DeviceState interface.
    #[cfg(not(feature = "task"))]
    pub fn clone_from(&mut self, out_screen: &mut Display, event: DeviceState) {
        self.set_idle(event.is_idle());
        self.set_signal(event.is_signal());
        self.set_output(out_screen, event.is_out_text());
        self.set_input(out_screen, event.is_input());
    }
}

impl fmt::Debug for ShellState {

    #[cfg(feature = "task")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "ShellState {{ task: {:?}, idle: {:?}, signal: {:?}, input: {:?}, output: {:?} }}",
               self.task, self.idle, self.sig, self.in_down, self.out_last.is_some()
        )
    }

    #[cfg(not(feature = "task"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
          "ShellState {{ idle: {:?}, signal: {:?}, input: {:?}, output: {:?}}}",
               self.idle, self.sig, self.in_down, self.out_last.is_some()
        )
    }
}
