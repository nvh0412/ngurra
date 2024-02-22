use crate::errors::Result;

pub trait Builder {
    type OutputType;
    fn build(&mut self) -> Result<Self::OutputType>;
}
