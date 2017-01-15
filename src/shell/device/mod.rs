pub mod control;
mod state;
mod input;
mod output;
mod spawn;

use ::libc;

pub use self::state::DeviceState;

pub use self::input::In;
pub use self::output::Out;

#[cfg(feature = "signal")]
pub type Sig = libc::c_int;

#[cfg(all(not(feature = "task"), not(feature = "idle"), not(feature = "signal")))]
mod notask_noidle_nosig;
#[cfg(all(not(feature = "task"), not(feature = "idle"), not(feature = "signal")))]
pub use self::notask_noidle_nosig::Device;

#[cfg(all(not(feature = "task"), not(feature = "idle"), feature = "signal"))]
mod notask_noidle_sig;
#[cfg(all(not(feature = "task"), not(feature = "idle"), feature = "signal"))]
pub use self::notask_noidle_sig::Device;

#[cfg(all(not(feature = "task"), feature = "idle", not(feature = "signal")))]
mod notask_idle_nosig;
#[cfg(all(not(feature = "task"), feature = "idle", not(feature = "signal")))]
pub use self::notask_idle_nosig::Device;

#[cfg(all(not(feature = "task"), feature = "idle", feature = "signal"))]
mod notask_idle_sig;
#[cfg(all(not(feature = "task"), feature = "idle", feature = "signal"))]
pub use self::notask_idle_sig::Device;

#[cfg(all(feature = "task", not(feature = "idle"), not(feature = "signal")))]
mod task_noidle_nosig;
#[cfg(all(feature = "task", not(feature = "idle"), not(feature = "signal")))]
pub use self::task_noidle_nosig::Device;

#[cfg(all(feature = "task", not(feature = "idle"), feature = "signal"))]
mod task_noidle_sig;
#[cfg(all(feature = "task", not(feature = "idle"), feature = "signal"))]
pub use self::task_noidle_sig::Device;

#[cfg(all(feature = "task", feature = "idle", not(feature = "signal")))]
mod task_idle_nosig;
#[cfg(all(feature = "task", feature = "idle", not(feature = "signal")))]
pub use self::task_idle_nosig::Device;

#[cfg(all(feature = "task", feature = "idle", feature = "signal"))]
mod task_idle_sig;
#[cfg(all(feature = "task", feature = "idle", feature = "signal"))]
pub use self::task_idle_sig::Device;

#[cfg(any(feature = "task"))]
pub mod procs;
#[cfg(any(feature = "task"))]
pub use self::procs::{Proc, BufProc};
