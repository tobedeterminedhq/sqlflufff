use crate::core::parser::segments::base::PathStep;

/// An element of the stack_positions property of DepthInfo.
#[derive(Debug, Clone)]
pub struct StackPosition {
    idx: usize,
    len: usize,
    position_type: String,
}

impl StackPosition {
    fn _stack_pos_interpreter(path_step: PathStep) -> String {}
}

#[derive(Debug, Clone)]
pub struct DepthMap {}

#[derive(Debug, Clone)]
pub struct DepthInfo {}
