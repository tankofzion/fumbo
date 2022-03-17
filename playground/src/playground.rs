use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::collections::HashMap;
use rustyline::{At, Cmd, CompletionType, Config, Context, EditMode, Editor, KeyCode, KeyEvent, Modifiers, Movement, Result as PlaygroundResult, Word};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline_derive::Helper;

use crate::Command;

const PROMPT: &str = ">> ";

#[derive(Helper)]
struct PlaygroundHelper {
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


#[derive(Default)]
pub struct Playground<Context> {
    name: String,
    version: String,
    description: String,
    commands: HashMap<String, Command>,
    completion: bool,
    context: Context,
}

impl <Context> Playground<Context>  {

    /// Build a new interpreter instance, providing its configuration
    pub fn new(context: Context) -> Self {
        Playground {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            commands: HashMap::new(),
            completion: false,
            context,
        }
    }

    /// Set playground name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set playground version
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// Set playground description
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Add a new command
    pub fn command(mut self, command: Command) -> Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    /// Start the playground loop
    pub fn execute(&mut self) -> PlaygroundResult<()> {
        env_logger::init();

        let helper = PlaygroundHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            validator: MatchingBracketValidator::new(),
            prompt: PROMPT.to_owned(),
        };

        let configuration = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();

        // build input line editor
        let mut editor = Editor::<PlaygroundHelper>::with_config(configuration);

        editor.set_helper(Some(helper));
        editor.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
        editor.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
        editor.bind_sequence(
            KeyEvent(KeyCode::Left, Modifiers::CTRL),
            Cmd::Move(Movement::BackwardWord(1, Word::Big)),
        );
        editor.bind_sequence(
            KeyEvent(KeyCode::Right, Modifiers::CTRL),
            Cmd::Move(Movement::ForwardWord(1, At::AfterEnd, Word::Big)),
        );

        // reload editor's history
        if editor.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        // let mut opt_history_file = None;
        // let config_dir = evcxr::config_dir();
        // if let Some(config_dir) = &config_dir {
        //     fs::create_dir_all(config_dir).ok();
        //     let history_file = config_dir.join("history.txt");
        //     editor.load_history(&history_file).ok();
        //     opt_history_file = Some(history_file);
        // }

        println!("Welcome to {} v{}", "Fumbo Playground", self.version);
        println!("Handcrafted since 2022 by Fumbo sorcerers");

        // enter the line processing loop
        loop {
            let prompt = format!("{}", PROMPT);
            editor.helper_mut().expect("No helper").prompt = format!("\x1b[1;32m{}\x1b[0m", prompt);
            let line = editor.readline(&prompt);
            match line {
                Ok(line) => {
                    editor.add_history_entry(line.as_str());
                    if let Err(error) = self.handle_line(&line.to_string()) {
                       // (self.error_handler)(error, self)?;
                        eprintln!("Error {:?} in line: {}", error, line);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Interrupted");
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
        editor.append_history("history.txt")
    }

    // Process given input line
    fn handle_line(&mut self, line: &str) -> PlaygroundResult<()> {
        println!("Line: {}", line);
        Ok(())
    }
}