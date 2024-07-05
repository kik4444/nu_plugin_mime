mod guess;
mod list;
mod mime_command;

use nu_plugin::Plugin;

use crate::{guess::MimeGuess, list::MimeList, mime_command::MimeCommand};

pub struct Mime;

impl Plugin for Mime {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(MimeCommand),
            Box::new(MimeList),
            Box::new(MimeGuess),
        ]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
