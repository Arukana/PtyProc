mod err;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
mod ffi;

#[cfg(feature = "task")]
use std::io::Write;
use std::ops::{Not, BitAnd};

pub use self::err::{ProcError, Result};

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;

pub type BufProc = (libc::pid_t, [libc::c_uchar; 32]);

use ::libc;

/// The proc directory.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_PROC: &'static str = "/proc";

/// The status's sub-directory.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_SUBD_STATUS: &'static str = "status";

/// The default capacity of proc dictionary.
const SPEC_CAPACITY_PROC: usize = 512;

impl Proc {
    /// The constructor method `new` returns the list of process.
    pub fn new(first_pid: libc::pid_t) -> Result<Self> {
        let mut status: Proc = Proc::default();

        status.first_pid = first_pid;
        status.running_pid = first_pid;
        status.with_list_process().and_then(|_| {
            Ok(status)
        })
    }


    /// The accessor method `get_name` returns the name of
    /// the process according to the pid.
    pub fn get_name(&self, pid: libc::pid_t)-> Option<BufProc> {
        self.list.iter().find(
            |&&(ref current_pid, _, _, _)| pid.eq(current_pid)
        ).and_then(|&(_, _, _, ref name): &(_, _, _, String)| {
            let mut source: [libc::c_uchar; 32] = [b'\0'; 32];
            {
                let mut buffer: &mut [libc::c_uchar] = &mut source[..];

                buffer.write(name.as_bytes());
            }
            Some((pid, source))
        })
    }

  /// The method `current_pid` returns the pid which the process is on
  pub fn current_pid(&self) -> libc::pid_t
  { fn currpid(list: &Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)>, from_pid: libc::pid_t, cur_pid: &mut libc::pid_t)
    { let mut childs: Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)> = Vec::with_capacity(list.len());
      list.into_iter().filter_map(|&(a, parent_pid, b, ref c)|
      { if from_pid.eq(&parent_pid)
        { Some((a, parent_pid, b, c.to_string())) }
        else
        { None }}).all(|k|
        { childs.push(k);
          true });
      if childs.is_empty().not()
      { childs.into_iter().all(|x|
        { if cfg!(target_os = "macos")
          { match x
            { (current, _, 2, _) =>
                { if list.into_iter().find(|&&(_, x, _, _)| x == current).is_some()
                  { currpid(list, current, cur_pid); }
                  else
                  { *cur_pid = current; }},
              (new_pid, _, _, _) => { currpid(list, new_pid, cur_pid); }, }}

          else if cfg!(target_os = "linux")
          { match x
            { (current, _, b'R', _) =>
                { if list.into_iter().find(|&&(_, x, _, _)| x == current).is_some()
                  { currpid(list, current, cur_pid); }
                  else
                  { *cur_pid = current; }},
              (new_pid, _, _, _) => { currpid(list, new_pid, cur_pid); }, }}

          true }); }}
    let mut pid: libc::pid_t = 0;
    currpid(&self.list, self.first_pid, &mut pid);
    pid }
}

impl Iterator for Proc
{ type Item = BufProc;

  fn next(&mut self) -> Option<BufProc>
  { self.list.clear();
    self.with_list_process().unwrap();
    let new_pid = self.current_pid();
    if self.running_pid != new_pid && new_pid > 1
{ println!("PID! OLD::{}, NEW::{}", self.running_pid, new_pid);
    	self.running_pid = new_pid;
      self.get_name(self.running_pid) }
    else
    { None }}}
    


impl Default for Proc {

    /// The constructor method `default` returns a empty list of process.
    fn default() -> Proc {
        Proc {
            first_pid: 0,
            running_pid: 0,
            list: Vec::with_capacity(SPEC_CAPACITY_PROC),
        }
    }
}
