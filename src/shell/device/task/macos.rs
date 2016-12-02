use std::mem;

use super::ffi;
use super::err::{ProcError, Result};
use ::libc;

unsafe fn get_unchecked_str(cp: *mut u8, start: *mut u8) -> String {
    let len = cp as usize - start as usize;
    let part = Vec::from_raw_parts(start, len, len);
    let tmp = String::from_utf8_unchecked(part.clone());

    mem::forget(part);
    tmp
}

#[derive(Debug)]
pub struct Proc {
    /// The first pid of the tree.
    pub first_pid: libc::pid_t,
    /// The current pid.
    pub running_pid: libc::pid_t,
    /// List by pid, ppid. status and unsized-name.
    pub list: Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)>,
}

impl Proc {
    pub fn with_list_process(&mut self) -> Result<()> {
        unsafe {
            let count = ffi::proc_listallpids(::std::ptr::null_mut(), 0);
            if count < 1 {
                return Err(ProcError::ListAllPidLen);
            }
            let mut pids: Vec<libc::pid_t> = Vec::with_capacity(count as usize);
            pids.set_len(count as usize);
            let count = count * mem::size_of::<libc::pid_t>() as i32;
            let x = ffi::proc_listallpids(pids.as_mut_ptr() as *mut libc::c_void, count);

            if x < 1 || x as usize > pids.len() {
                return Err(ProcError::ListAllPidLenUnvalid);
            } else if pids.len() > x as usize {
                pids.set_len(x as usize);
            }

            let taskallinfo_size = mem::size_of::<ffi::proc_taskallinfo>() as i32;
            let mut mib: [libc::c_int; 3] = [ffi::CTL_KERN, ffi::KERN_ARGMAX, 0];
            let mut argmax = 0;
            let mut size = mem::size_of::<libc::c_int>();
            while libc::sysctl(
                mib.as_mut_ptr(),
                2,
                (&mut argmax) as *mut i32 as *mut libc::c_void,
                 &mut size,
                 ::std::ptr::null_mut(),
                 0
            ).eq(&-1) {}
            mib[0] = ffi::CTL_KERN;
            mib[1] = ffi::KERN_PROCARGS2;
            let mut proc_args: Vec<u8> = Vec::with_capacity(argmax as usize);
            pids.iter().all(|&pid| {
                self.push(pid, taskallinfo_size as i32, &mut proc_args, mib, argmax as libc::size_t);
                true
            });
            Ok(())
        }
    }

    fn push(&mut self,
        pid: libc::c_int,
        taskallinfo_size: i32,
        proc_args: &mut Vec<u8>,
        mut mib: [libc::c_int; 3],
        argmax: libc::size_t,
    ) {
        unsafe {
            let mut task_info = mem::zeroed::<ffi::proc_taskallinfo>();
            if ffi::proc_pidinfo(pid,
                                 ffi::PROC_PIDTASKALLINFO,
                                 0,
                                 &mut task_info as *mut ffi::proc_taskallinfo as *mut libc::c_void,
                                 taskallinfo_size) != taskallinfo_size {
                return ;
            }
            let ptr = proc_args.as_mut_slice().as_mut_ptr();
            mib[2] = pid as libc::c_int;
            let mut size = argmax;
            let mut name = String::new();

            if libc::sysctl(mib.as_mut_ptr(), 3, ptr as *mut libc::c_void,
                           &mut size, ::std::ptr::null_mut(), 0) != -1 {
                let mut cp = ptr.offset(mem::size_of::<libc::c_int>() as isize);
                let start = cp;
                if cp < ptr.offset(size as isize) {
                    while cp < ptr.offset(size as isize) && *cp != 0 {
                        cp = cp.offset(1);
                    }
                    let exe = get_unchecked_str(cp, start);
                    if let Some(l) = exe.split("/").last() {
                        name = l.to_owned();
                    }
                }
                self.list.push((pid, task_info.pbsd.ppbi_pid as i32, (task_info.pbsd.pbi_nice as u32 ^ task_info.pbsd.pbi_status) as u8, name));
            }
        }
    }
}
