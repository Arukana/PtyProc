#[derive(Debug)]
pub enum Sequence {
    Out(String),
    Goto(u16, u16),
}
