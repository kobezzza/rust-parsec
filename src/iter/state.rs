use std::any::Any;
use std::fmt::Debug;

pub trait ParserState: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn ParserState>;
}

impl Clone for Box<dyn ParserState> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}