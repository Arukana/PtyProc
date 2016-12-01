use ::libc;

const MAXCOMLEN: usize = 16; // MAXCOMLEN.

pub const PROC_PIDTASKALLINFO: i32 = 2;
pub const CTL_KERN: libc::c_int = 1;
pub const KERN_ARGMAX: libc::c_int = 8;
pub const KERN_PROCARGS2: libc::c_int = 49;

#[repr(C)]
pub struct proc_taskinfo {
    pub pti_virtual_size: u64, /* virtual memory size (bytes) */
    pub pti_resident_size: u64, /* resident memory size (bytes) */
    pub pti_total_user: u64, /* total time */
    pub pti_total_system: u64,
    pub pti_threads_user: u64, /* existing threads only */
    pub pti_threads_system: u64,
    pub pti_policy: i32, /* default policy for new threads */
    pub pti_faults: i32, /* number of page faults */
    pub pti_pageins: i32, /* number of actual pageins */
    pub pti_cow_faults: i32, /* number of copy-on-write faults */
    pub pti_messages_sent: i32, /* number of messages sent */
    pub pti_messages_received: i32, /* number of messages received */
    pub pti_syscalls_mach: i32, /* number of mach system calls */
    pub pti_syscalls_unix: i32, /* number of unix system calls */
    pub pti_csw: i32, /* number of context switches */
    pub pti_threadnum: i32, /* number of threads in the task */
    pub pti_numrunning: i32, /* number of running threads */
    pub pti_priority: i32, /* task priority */
}

#[repr(C)]
pub struct proc_bsdinfo {
    pub pbi_flags: u32,
    pub pbi_status: u32,
    pub pbi_xstatus: u32,
    pub pbi_pid: u32,
    pub ppbi_pid: u32,
    pub pbi_uid: libc::uid_t,
    pub pbi_gid: libc::gid_t,
    pub pbi_ruid: libc::uid_t,
    pub pbi_rgid: libc::gid_t,
    pub pbi_svuid: libc::uid_t,
    pub pbi_svgid: libc::gid_t,
    pub rfu_1: u32,
    pub pbi_comm: [u8; MAXCOMLEN],
    pub pbi_name: [u8; 2 * MAXCOMLEN],
    pub pbi_nfiles: u32,
    pub pbi_pgid: u32,
    pub pbi_pjobc: u32,
    pub e_tdev: u32,
    pub e_tpgid: u32,
    pub pbi_nice: i32,
    pub pbi_start_tvsec: u64,
    pub pbi_start_tvusec: u64,
}

#[repr(C)]
pub struct proc_taskallinfo {
    pub pbsd: proc_bsdinfo,
    pub ptinfo: proc_taskinfo,
}

extern "C" {
    pub fn proc_pidinfo(
        pid: libc::c_int,
        flavor: libc::c_int,
        arg: u64,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int
    ) -> libc::c_int;

    pub fn proc_listchildpids(_ppid: libc::pid_t, _buff: *mut libc::c_void, _size: libc::c_int) -> libc::c_int;
}
