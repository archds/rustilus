use std::fmt::Display;

mod list_shuffle;

use inquire::Select;

pub enum Action {
    ListShuffle,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListShuffle => write!(f, "List shuffle"),
        }
    }
}

fn main() {
    let options: Vec<Action> = vec![Action::ListShuffle];

    let result = Select::new("Choose command", options)
        .prompt()
        .map(|cmd| match cmd {
            Action::ListShuffle => list_shuffle::entrypoint(),
        });

    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
