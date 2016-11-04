use ::libc;

use super::In;
use super::Out;
use super::Sig;
use super::control::Control;

pub enum DeviceState {
    /// The current task.
    #[cfg(feature = "task")] Proc(String),
    /// Update.
    Idle,
    /// As catched a signal.
    Sig(Sig),
    /// The output of new lines.
    OutText(Out, libc::size_t),
    /// The current character.
    InText(Control),
}

impl DeviceState {

    #[cfg(feature = "task")]
    /// The constructor method `from_task` returns a Process' event.
    pub fn from_task(name: String) -> Self {
        DeviceState::Proc(name)
    }

    /// The constructor method `from_idle` returns a Update's event.
    pub fn from_idle() -> Self {
        DeviceState::Idle
    }

    /// The constructor method `from_out` returns a text Output's event.
    pub fn from_out(buf: Out, len: libc::size_t) -> Self {
        DeviceState::OutText(buf, len)
     }

    /// The constructor method `from_out` returns a key Input's event.
    pub fn from_in(buf: In, len: libc::size_t) -> Self {
        DeviceState::InText(Control::new(buf, len))
    }

    /// The constructor method `from_ig` returns a Signal's event.
    pub fn from_sig(sig: libc::c_int) -> Self {
        DeviceState::Sig(sig)
    }

    #[cfg(feature = "task")]
    /// The accessor method `is_task` returns a Process' event.
    pub fn is_task(self) -> Option<String> {
        match self {
            DeviceState::Proc(name) => Some(name),
            _ => None,
        }
    }

    /// The accessor method `is_idle` returns a Option for Update's event.
    pub fn is_idle(&self) -> Option<()> {
        match *self {
            DeviceState::Idle => Some(()),
            _ => None,
        }
    }

    /// The accessor method `is_out_text` returns a Option for Ouput's event.
    pub fn is_out_text(&self) -> Option<(Out, libc::size_t)> {
        match *self {
            DeviceState::OutText(buf, len) => Some((buf, len)),
            _ => None,
        }
    }

    /// The accessor method `is_input` returns a Option for key
    /// or mouse Input's event.
    pub fn is_input(&self) -> Option<Control> {
        match *self {
            DeviceState::InText(event) => Some(event),
            _ => None,
        }
    }

    /// The accessor method `is_signal` returns a Option for Signal's event.
    pub fn is_signal(&self) -> Option<libc::c_int> {
        match *self {
            DeviceState::Sig(sig) => Some(sig),
            _ => None,
        }
    }
}

impl Clone for DeviceState {
    fn clone(&self) -> DeviceState {
        match *self {
            #[cfg(feature = "task")] DeviceState::Proc(ref name) => DeviceState::Proc(name.clone()),
            DeviceState::Idle => DeviceState::Idle,
            DeviceState::Sig(sig) => DeviceState::Sig(sig),
            DeviceState::OutText(out, len) => DeviceState::OutText(out, len),
            DeviceState::InText(control) => DeviceState::InText(control),
        }
    }
}
