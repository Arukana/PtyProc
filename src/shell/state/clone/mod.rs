pub use super::DeviceState;

pub trait Clone {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: DeviceState);
}
