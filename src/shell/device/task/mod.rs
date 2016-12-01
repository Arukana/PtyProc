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

pub enum BufProc {
  /// Update(created_pids, removed_pids)
  Update(Vec<libc::pid_t>, Vec<libc::pid_t>),
  /// CurrentPid(new_current_pid, [its_path_name])
  CurrentPid(libc::pid_t, [libc::c_uchar; 32]),
}

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
  /// The constructor method `new` returns a tree of processes from `pid`
  pub fn new(pid: libc::pid_t, ppid: libc::pid_t, status: libc::c_uchar, name: String) -> Self
  { let pids = proc_get_pids(pid);
    let mut childs = Vec::new();
    match pids
    { Some(mut vec) =>
        { vec.sort_by(|(a, _, _, _), (b, _, _, _)| a.cmp(b));
          vec.iter().all(|i|
          { childs.push(Proc::new(i.info));
            true });
        Proc { info: (pid, ppid, status, name), childs: childs, }},
      None => Proc { pid: pid, childs: childs, }, }}

    /// The constructor method `new` returns the list of process.
    pub fn new(fpid: libc::pid_t) -> Result<Self> {
        let mut status: Proc = Proc::default();

        status.fpid = fpid;
        status.with_list_process().and_then(|_| {
            Ok(status)
        })
    }


    /// The accessor method `get_name` returns the name of
    /// the process according to the pid.
    pub fn get_name(&self, pid: libc::pid_t)-> Option<BufProc> {
        self.list.iter().find(
            |&&(ref cpid, _, _, _)| pid.eq(cpid)
        ).and_then(|&(_, _, _, ref name): &(_, _, _, String)| {
            let mut source: [libc::c_uchar; 32] = [b'\0'; 32];
            {
                let mut buffer: &mut [libc::c_uchar] = &mut source[..];

                buffer.write(name.as_bytes());
            }
            Some((pid, source))
        })
    }

    /// The method `from_pid` returns the last active child process
    /// from the fpid process argument.
    fn from_pid(&self, fpid: Option<libc::pid_t>) -> Option<libc::pid_t> {
        if let Some(&(cpid, _, _, _)) = self.list.iter().find(
            |&&(_, ref ppid, _, _)| fpid.unwrap_or(self.fpid).eq(ppid)
        ) {
            self.from_pid(Some(cpid))
        }
        else {
            fpid.or(Some(self.fpid))
        }
    }

    fn cmp(&self, baum: Baum) -> (Vec<libc::pid_t>, Vec<libc::pid_t>)
    { fn checker(get: &Baum, baum: &Baum, pids: &mut Vec<libc::pid_t>)
      { if !pid_in_baum(baum.pid, get)
        { pids.push(baum.pid); }
          if !baum.childs.is_empty()
          { baum.childs.iter().map(|x| checker(get, x, pids)); }}
      let mut in_pids: Vec<libc::pid_t> = Vec::new();
      let mut out_pids: Vec<libc::pid_t> = Vec::new();
      checker(self, &baum, &mut out_pids);
      checker(&baum, self, &mut in_pids);
      (in_pids, out_pids) }
}

impl Iterator for Proc {
    type Item = BufProc;

    fn next(&mut self) -> Option<BufProc>
    { self.list.clear();
      self.with_list_process().unwrap();
      self.from_pid(None).and_then(|cfpid|
      { if cfpid.eq(&self.fpid).not().bitand(self.lpid.eq(&self.fpid).not())
        { self.fpid = cfpid;
          self.get_name(cfpid) }
        else
        { None }}) }
}

impl Default for Proc
{ fn default() -> Proc
  { Proc { info:(0, 0, 0, "kernel".to_string()), childs: vec![Proc::new(0)], }}}
