mod err;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
mod ffi;

use std::ops::Not;
use std::ops::BitAnd;

pub use self::err::{ProcError, Result};

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;

use ::libc;

/// The proc directory.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_PROC: &'static str = "/proc";

/// The status's sub-directory.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_SUBD_STATUS: &'static str = "status";

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_PROC: usize = 512;

impl Proc {
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
    pub fn get_name(&self, pid: libc::pid_t)-> Option<String> {
        self.list.iter().find(
            |&&(ref cpid, _, _, _)| pid.eq(cpid)
        ).and_then(|&(_, _, _, ref name)|
                Some(name.clone())
        )
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
}

impl Iterator for Proc {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.list.clear();
        self.with_list_process().unwrap();

        self.from_pid(None).and_then(|cfpid| {
            if cfpid.eq(&self.fpid).not().bitand(
               self.lpid.eq(&self.fpid).not()
            ) {
               self.lpid = cfpid;
               self.get_name(cfpid)
            } else {
                None
            }
        })
    }
}

impl Default for Proc {

    /// The constructor method `default` returns a empty list of process.
    fn default() -> Proc {
        Proc {
            fpid: 0,
            lpid: 0,
            list: Vec::with_capacity(SPEC_CAPACITY_PROC),
        }
    }
}
