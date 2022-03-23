use std::borrow::{BorrowMut, Cow};
use std::borrow::Cow::{Borrowed, Owned};
use std::collections::HashMap;
use rustyline::config::Configurer;
use rustyline::{At, Cmd, CompletionType, Config, Context, EditMode, Editor, KeyCode, KeyEvent, Modifiers, Movement, Word};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline_derive::Helper;

use crate::command::Command;
use crate::context::Context as PlaygroundContext;
use crate::error::{PlaygroundError, PlaygroundResult};

// Default shell prompt
const PROMPT: &str = "Woodoo >> ";

#[derive(Helper)]
pub struct PlaygroundHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    prompt: String,
}

impl Completer for PlaygroundHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for PlaygroundHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for PlaygroundHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for PlaygroundHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

/// Playground structure
pub struct Playground<'p> {
    /// Application name
    name: &'p str,

    /// Application's version number (using semantic versioning format, namely 'major.minor.patch')
    version: &'p str,

    /// Short application description
    description: &'p str,

    /// Application commands
    commands: HashMap<&'p str, Command<'p>>,

    /// Application's command-line arguments
    arguments: &'p [String],

    // Reference to readline editor
    editor: Editor<PlaygroundHelper>,

    /// Auto-completion (using tab character, usually) support flag
    completion: bool,
}

// Implement playground methods
impl <'p> Playground<'p>  {

    /// Build and configure a new playground instance
    pub fn new() -> Self {

        let playground = Self {
            name: Default::default(),
            version: Default::default(),
            description: Default::default(),
            commands: Default::default(),
            arguments: &[],
            editor: Editor::<PlaygroundHelper>::new(),
            completion: true
        };

        playground.configure()
    }

    // Configure playground
    fn configure(mut self) -> Self {

        // Set line editor helpers
        let helper = PlaygroundHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            validator: MatchingBracketValidator::new(),
            prompt: PROMPT.to_owned(),
        };

        self.editor.set_helper(Some(helper));

        // Set basic line editor settings
        self.editor.set_history_ignore_space(true);
        self.editor.set_completion_type(CompletionType::List);
        self.editor.set_edit_mode(EditMode::Emacs);

        // Bind keyboard sequences to some line editor functions
        self.editor.bind_sequence(
            KeyEvent::alt('n'),
            Cmd::HistorySearchForward);
        self.editor.bind_sequence(
            KeyEvent::alt('p'),
            Cmd::HistorySearchBackward);
        self.editor.bind_sequence(
            KeyEvent(KeyCode::Left, Modifiers::CTRL),
            Cmd::Move(Movement::BackwardWord(1, Word::Big)),
        );
        self.editor.bind_sequence(
            KeyEvent(KeyCode::Right, Modifiers::CTRL),
            Cmd::Move(Movement::ForwardWord(1, At::AfterEnd, Word::Big)),
        );

        // Load line editor history (from a file)
        if self.editor.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        self
    }

    /// Set playground name
    pub fn name<T: Into<&'p str>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    /// Set playground version
    pub fn version<T: Into<&'p str>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    /// Set a short playground description
    pub fn description<T: Into<&'p str>>(mut self, description: T) -> Self {
        self.description = description.into();
        self
    }

    /// Add a new command to the playground
    pub fn command(mut self, command: Command<'p>) -> Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    /// Returns a mutable reference to the inner readline editor object
    pub fn editor(&mut self) -> &mut Editor<PlaygroundHelper> {
         &mut self.editor
    }

    /// Start the playground loop, providing command-line arguments
    pub fn run(&mut self, arguments: &'p [String]) -> rustyline::Result<()> {
        env_logger::init();

        // Save command-line arguments
        self.arguments = arguments;

        // let mut opt_history_file = None;
        // let config_dir = playground::config_dir();
        // if let Some(config_dir) = &config_dir {
        //     fs::create_dir_all(config_dir).ok();
        //     let history_file = config_dir.join("history.txt");
        //     editor.load_history(&history_file).ok();
        //     opt_history_file = Some(history_file);
        // }

        // Display a welcome message
        self.welcome();

        // enter the line processing loop
        loop {
            let prompt = format!("{}", PROMPT);
            self.editor.helper_mut().expect("No helper").prompt = format!("\x1b[1;32m{}\x1b[0m", prompt);

            let line = self.editor.readline(&prompt);
            match line {
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str());
                    if let Err(error) = self.process_line(&line.to_string()) {
                        eprintln!("Error {:?} in line: {}", error, line);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("\nInterrupted!");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Encountered Eof");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }

        }

        // save history of commands
        self.editor.append_history("history.txt")
    }

    // Process given input line content and execute command action
    fn process_line(&mut self, line: &str) -> PlaygroundResult<()> {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            let content = regex::Regex::new(r#"("[^"\n]+"|[\S]+)"#).unwrap();
            let arguments = content
                .captures_iter(trimmed_line)
                .map(|a| a[0].to_string().replace("\"", ""))
                .collect::<Vec<String>>();

            let mut arguments = arguments.iter().fold(vec![], |mut state, a| {
                state.push(a.as_str());
                state
            });

            let command: String = arguments.drain(..1).collect();

            self.execute_command(&command, &arguments)?;
        }

        Ok(())
    }

    // Execute a given command
    fn execute_command(&mut self, name: &str, arguments: &[&str]) -> PlaygroundResult<()> {

        // Process basic buit-in commands first
        match self.commands.get(name) {
            Some(command) => {
                command.execute(&mut PlaygroundContext::new(self, arguments));
            }
            None => {
                return Err(PlaygroundError::UnknownCommand(name.to_string()));
            }
        }

        Ok(())
    }

    // Display welcome message
    fn welcome(&self) {
        println!("\nWelcome to {} v{}", self.name, self.version);
        println!("Handcrafted since 2022 by Fumbo sorcerers\n");
        println!("For help, enter 'help'");
    }

    /// Display basic help message
    pub fn display_help(&self) {
        println!("Show help");
    }

    /// Display version
    pub fn display_version(&self) {
        println!("v{}", self.version);
    }

    /// Display the list of commands stored in history cache
    pub fn display_history(&self) {
        let history= self.editor.history();
        if ! history.is_empty() {
            let mut count = 0;
            for entry in history {
                println!("{}: {}", count, entry);
                count += 1;
            }
        }
    }
}