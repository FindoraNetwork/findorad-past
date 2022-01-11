use std::fmt;

use console::{style, Emoji};

#[derive(Default, Debug)]
pub struct Content {
    pub name: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug)]
pub struct Display {
    contents: Vec<Content>,
}

impl Display {
    fn empty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            Emoji("💡", "!?"),
            style("No changes made").bold().yellow()
        )
    }

    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for content in self.contents.iter() {
            write!(
                f,
                "
{} {}
{} {} {}
            ",
                Emoji("📃", "!!"),
                style(content.name.clone()).bold().white(),
                style(content.before.clone()).bold().magenta(),
                Emoji("➤ ", "->"),
                style(content.after.clone()).bold().green(),
            )?
        }
        Ok(())
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return self.empty(f);
        }

        self.display(f)
    }
}

impl From<Vec<(String, String, String)>> for Display {
    fn from(v: Vec<(String, String, String)>) -> Display {
        Display {
            contents: v.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<(String, String, String)> for Content {
    fn from(c: (String, String, String)) -> Content {
        Content {
            name: c.0,
            before: c.1,
            after: c.2,
        }
    }
}
