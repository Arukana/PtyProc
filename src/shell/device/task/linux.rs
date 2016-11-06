use std::fs;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};

use super::{SPEC_PROC, SPEC_SUBD_STATUS, SPEC_CAPACITY_PROC};
use super::err::{ProcError, Result};
use ::libc;

#[derive(Debug)]
pub struct Proc {
    /// The first pid for tree.
    pub fpid: libc::pid_t,
    pub lpid: libc::pid_t,
    /// List by pid, ppid. status and unsized-name.
    pub list: Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)>,
}

impl Proc {

    /// The method `with_list_process` pushes all process after the first pid.
    pub fn with_list_process(&mut self) -> Result<()> {
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
}
