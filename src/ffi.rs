use ::libc;

#[link(name = "c")]
extern "C" {
  pub fn write(fd: libc::c_int, buf: *const libc::c_uchar, count: libc::size_t) -> libc::ssize_t;
}
