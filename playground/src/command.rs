//! Playground command
//!
//! ## Example
//!
//! ```text
//!
//! fn hello(&mut context: Context) {
//!     println!("Hello {}", context.arguments);
//! }
//!
//! // Create sample greeting command
//! let hello = Command::new("hello)
//!     .description("Be polite folks!!!"),
//!     .action(hello_handler)
//!
//! // Create a playground instance
//! let playground = Playground::new();
//!
//! // Add the greeter command to the playground
//! playground.command(hello)
//! ```

use crate::Context;

/// Command action type
pub type Action = fn(&mut Context);

/// Command structure
#[derive(Default)]
pub struct Command<'p> {
    /// Command identifier (or name)
    pub(crate) name: &'p str,

    /// Short description of the command's purpose
    pub(crate) description: Option<&'p str>,

    /// Auto-generated help message
    pub(crate) help: Option<&'p str>,

    /// Command action handling function
    pub(crate) action: Option<Action>,
}

// Implement command functionalities
impl <'p> Command<'p> {

    /// Create a new command instance with a given name and callback
    pub fn new<T: Into<&'p str>>(name: T) -> Self {
        Command {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set command name
    pub fn name<T: Into<&'p str>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    /// Set command description
    pub fn description<T: Into<&'p str>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set command action
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set command help message
    pub fn help<T: Into<&'p str>>(mut self, help: T) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Execute command action
    pub fn execute(&self, context: &mut Context) {
        let action = self.action.unwrap();
        action(context);
    }
}
