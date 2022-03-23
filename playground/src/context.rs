use crate::Playground;

pub struct Context<'c>
{
    /// Reference to the parent playground
    pub playground: &'c Playground<'c>,

    /// Command arguments
    pub arguments: &'c [&'c str],
}


impl<'c> Context<'c> {
    pub fn new(playground: &'c Playground, arguments: &'c [&'c str]) -> Self {
        Context {
            playground,
            arguments,
        }
    }
}