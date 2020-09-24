use seahorse::Context;
use console::{Term, Style};
use dialoguer::theme::{ColorfulTheme, Theme};
use dialoguer::Select;

pub fn main(c: &Context) {
    let term = Term::stdout();
    term.set_title("CSV mapper");

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(&theme)
            .with_prompt("Choose action")
            .default(0)
            .item("Map")
            .item("Cancel")
            .item("Save and exit")
            .interact().unwrap();

        match next_menu {
            0 => {
                map_view(&term, &theme);
            },
            1 => {
                term.clear_screen();
                std::process::exit(0);
            },
            2 => {
                term.clear_screen();
                std::process::exit(0);
            },
            _ => {
                term.clear_screen();
                std::process::exit(0);
            }
        }
    }
}

fn map_view(term: &Term, theme: &ColorfulTheme) {
    loop {
        term.clear_screen();

        let next_menu = Select::with_theme(theme)
            .with_prompt("Map headers")
            .default(0)
            .item("Back")
            .interact().unwrap();

        match next_menu {
            0 => {
                break
            },
            _ => {

            }
        }
    }
}