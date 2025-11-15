use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultPromptSegment,
    Emacs, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal,
};
use std::error::Error;

mod commands;
mod helper;

fn main() -> Result<(), Box<dyn Error>> {
    let commands = vec![
        "test".into(),
        "hello world".into(),
        "hello world reedline".into(),
        "this is the reedline crate".into(),
    ];

    let completer = Box::new(DefaultCompleter::new(commands.clone()));

    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode);

    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic(String::from("$ ")),
        DefaultPromptSegment::Empty,
    );

    loop {
        match editor.read_line(&prompt) {
            Ok(Signal::Success(command)) => {
                let output = commands::create_command(&command).execute();

                if let Some(out) = output.stdout {
                    println!("{}", out.trim_end_matches("\n"));
                }

                if let Some(out) = output.stderr {
                    println!("{}", out.trim_end_matches("\n"));
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                break;
            }
            x => {
                x?;
            }
        }
    }

    Ok(())
}
