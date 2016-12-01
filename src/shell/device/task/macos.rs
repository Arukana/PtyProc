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
pub enum Status
{ ///Processus sur lequel on est
  Current,
  ///Processus actif
  Active,
  ///Processus suspendu > Ctrl+Z
  Suspend,
  ///Processus zombie > yes 1&>/dev/null &
  Zombie,
  ///Etat non implémenté
  Other, }

#[derive(Debug)]
pub struct Proc
{ /// Process pid, ppid. status and unsized-name.
  pub info: (libc::pid_t, libc::pid_t, libc::c_uchar, String),
  /// List of its childs
  pub childs: Vec<Proc>, }

pub fn proc_get_pids(pid: libc::pid_t) -> Option<Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)>>
{ unsafe 
  { let nb_childs = ffi::proc_listchildpids(0, ::std::ptr::null_mut(), 0);
    if nb_childs < 1
    { return None; }
    let mut child_pids: Vec<libc::pid_t> = Vec::with_capacity(nb_childs as usize);
    child_pids.set_len(nb_childs as usize);
    let count = nb_childs * mem::size_of::<libc::pid_t>() as i32;
    let x = ffi::proc_listchildpids(child_pids.as_mut_ptr() as *mut libc::c_void, count);
    if x < 1 || x as usize > child_pids.len()
    { return None; }
    else if child_pids.len() > x as usize
    { child_pids.set_len(x as usize); }
    let taskallinfo_size = mem::size_of::<ffi::proc_taskallinfo>() as i32;
    let mut mib: [libc::c_int; 3] = [ffi::CTL_KERN, ffi::KERN_ARGMAX, 0];
    let mut argmax = 0;
    let mut size = mem::size_of::<libc::c_int>();
    while libc::sysctl(mib.as_mut_ptr(), 2, (&mut argmax) as *mut i32 as *mut libc::c_void, &mut size, ::std::ptr::null_mut(), 0).eq(&-1) {}
    mib[0] = ffi::CTL_KERN;
    mib[1] = ffi::KERN_PROCARGS2;
    let mut proc_args: Vec<u8> = Vec::with_capacity(argmax as usize);
    let mut childs_with_info: Vec<(libc::pid_t, libc::pid_t, libc::c_uchar, String)> = Vec::new();
    child_pids.iter().all(|&pid|
    { childs_with_info.push(pousse(pid, taskallinfo_size as i32, &mut proc_args, mib, argmax as libc::size_t));
      true });
    Some(childs_with_info) }}

fn pousse(pid: libc::c_int, taskallinfo_size: i32, proc_args: &mut Vec<u8>, mut mib: [libc::c_int; 3], argmax: libc::size_t) -> (libc::pid_t, libc::pid_t, libc::c_uchar, String)
{ unsafe
  { let mut task_info = mem::zeroed::<ffi::proc_taskallinfo>();
    if ffi::proc_pidinfo(pid, ffi::PROC_PIDTASKALLINFO, 0, &mut task_info as *mut ffi::proc_taskallinfo as *mut libc::c_void, taskallinfo_size) != taskallinfo_size
    { return ; }
    let ptr = proc_args.as_mut_slice().as_mut_ptr();
    mib[2] = pid as libc::c_int;
    let mut size = argmax;
    let mut name = String::new();

    if libc::sysctl(mib.as_mut_ptr(), 3, ptr as *mut libc::c_void, &mut size, ::std::ptr::null_mut(), 0) != -1
    { let mut cp = ptr.offset(mem::size_of::<libc::c_int>() as isize);
      let start = cp;
      if cp < ptr.offset(size as isize)
      { while cp < ptr.offset(size as isize) && *cp != 0
        { cp = cp.offset(1); }
        let exe = get_unchecked_str(cp, start);
        if let Some(l) = exe.split("/").last()
        { name = l.to_owned(); }}
      (pid, task_info.pbsd.ppbi_pid as i32, task_info.pbsd.pbi_nice & task_info.pbsd.pbi_status, name) }}}

/// Retuns the current pid
pub fn current_pid() -> libc::pid_t
{ unsafe
  { fn currpid(baum: &Proc, pid: &mut libc::pid_t)
    { match status(baum.info.0)
      { Status::Current => { *pid = baum.info.0; },
        _ => {}, }
      if baum.childs.is_empty().not()
      { baum.childs.iter().all(|i|
        { currpid(i, pid);
          true }); }}
    let mut pid: libc::pid_t = 0;
    currpid(&Proc::new(libc::getpid(), 0, 0, ), &mut pid);
    pid }}

fn status(pid: libc::pid_t) -> Status
{ let info: Option<ProcBsdInfo> = proc_get_info(pid);
  match info
  { Some(proz) =>
    { match (proz.pbi_status, proz.pbi_nice)
      { (2, 0) =>
        { unsafe
          { let cur_pid = libc::getpid();
            let cur_baum = Baum::new(cur_pid);
            let baum = Baum::new(pid);
            if !pid_in_baum(pid, &cur_baum) || !baum.childs.is_empty()
            { Status::Active }
            else
            { Status::Current }}},
          (4, 0) => Status::Suspend,
          (2, 5) => Status::Zombie,
          (_, _) => Status::Other, }},
      None => Status::Other, }}     

/// Ask if `pid` is in a given tree `baum`
pub fn pid_in_baum(pid: libc::pid_t, baum: &Baum) -> bool
{ if baum.info.0.eq(&pid)
  { true }
  else if baum.childs.is_empty().not()
  { baum.childs.iter().find(|&b| pid_in_baum(pid, b)).is_some() }
  else
  { false }}

impl Proc {
  pub fn with_list_process(&mut self) -> Result<()>
  { 
    }
}

impl PartialEq for Proc
{ fn eq(&self, baum: &Proc) -> bool
  { if self.info.0 != baum.info.0
    { false }
    else if baum.childs.is_empty().not()
    { baum.childs.iter().find(|b| self.eq(b)).is_none() }
    else
    { true }}}
