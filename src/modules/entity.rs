#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub pos: usize,
    pub tape: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NextAction {
    GoForward,
    JumpBackward,
    JumpForward,
}
