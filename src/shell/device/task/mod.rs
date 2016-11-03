pub mod err;

use std::str;
use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};

pub use self::err::{ProcError, Result};
use ::libc;

/// The proc directory.
const SPEC_PROC: &'static str = "/proc";

/// The status's sub-directory.
const SPEC_SUBD_STATUS: &'static str = "status";

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_PROC: usize = 255;

#[derive(Debug)]
pub struct Proc {
    /// The first pid for tree.
    fpid: libc::pid_t,
    /// name, status, pid, ppid.
    list: Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)>,
}

impl Proc {

    /// The constructor method `new` returns the list of process.
    pub fn new(fpid: libc::pid_t) -> Result<Self> {
        let mut status: Proc = Proc::default();

        status.with_list_process().and_then(|_| {
            status.fpid = fpid;
            Ok(status)
        })
    }

    fn with_list_process(&mut self) -> Result<()> {
        let fpid: PathBuf = PathBuf::from(self.fpid.to_string());
        match fs::read_dir(Path::new(SPEC_PROC)) {
            Err(why) => Err(ProcError::ReadDir(why)),
            Ok(entry) => {
                entry.filter_map(|entry| entry.ok())
                     .skip_while(|entry| entry.path().ends_with(&fpid))
                     .filter_map(|entry|
                         fs::File::open(entry.path().join(SPEC_SUBD_STATUS)).ok()
                                  .and_then(|file|
                                           file.metadata().ok().and_then(|metadata|
                                               if metadata.is_file() {
                                                   Some(file)
                                               } else {
                                                  None
                                               }
                                           )
                                  )).all(|entry| {
                                      self.push(entry).is_some()
                                  });
                Ok(())
            },
        }
    }

    /// The method `push` adds a new process from a file descriptor.
    fn push(&mut self, entry: fs::File) -> Option<()> {
        let file = io::BufReader::new(entry);
        let mut iter = file.lines();
        match (
            iter.next(), iter.next(), iter.next(),
            iter.skip(2).next()
        ) {
            (Some(Ok(mut name)),
            Some(Ok(state)),
            Some(Ok(mut pid)),
            Some(Ok(mut ppid))) => unsafe {
                let (len_pid, len_ppid, len_name) = (
                    pid.len(), ppid.len(), name.len()
                );
                Some(self.list.push((
                    pid.slice_mut_unchecked(6, len_pid).parse::<libc::pid_t>()
                        .ok().unwrap_or_default(),
                    ppid.slice_mut_unchecked(6, len_ppid).parse::<libc::pid_t>()
                        .ok().unwrap_or_default(),
                    *state.as_bytes().get(7).unwrap_or(&b'\x00'),
                    name.slice_mut_unchecked(6, len_name).parse::<String>()
                        .ok().unwrap_or_default()
                )))
            },
            _ => None,
        }
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

impl Clone for Proc {
    fn clone(&self) -> Self {
        let mut status: Proc = Proc::default();

        status.fpid = self.fpid;
        status.with_list_process();
        status
    }

    fn clone_from(&mut self, source: &Self) {
        self.fpid = source.fpid;
        self.with_list_process();
    }
}

impl Iterator for Proc {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current: Proc = self.clone();

        current.from_pid(None).and_then(|cfpid|
            if cfpid.eq(&self.fpid).not() {
                current.get_name(cfpid)
            } else {
                None
            }
        )
    }
}

impl Default for Proc {
    fn default() -> Proc {
        Proc {
            fpid: 0,
            list: Vec::with_capacity(SPEC_CAPACITY_PROC),
        }
    }
}
