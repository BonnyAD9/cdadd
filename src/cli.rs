use log::warn;
use pareg::Pareg;

use crate::err::{Error, Result};

pub enum Action {
    Help,
    Encode(String),
}

#[derive(Default)]
pub struct Args {
    action: Option<Action>,
    output: Option<String>,
    pub interactive: bool,
}

impl Args {
    pub fn parse(args: Pareg) -> Result<Self> {
        let mut res = Self::default();
        res.parse_base(args)?;
        res.validate()?;
        Ok(res)
    }

    pub fn output(&self) -> &str {
        self.output.as_ref().map_or(".", |o| o.as_ref())
    }

    pub fn action(&self) -> &Action {
        self.action.as_ref().unwrap()
    }

    fn parse_base(&mut self, mut args: Pareg) -> Result<()> {
        while let Some(arg) = args.next() {
            match arg {
                "-h" | "--help" | "-?" => self.set_help()?,
                "-e" | "--encode" => self.set_encode(args.next_arg()?)?,
                "-o" | "--output" => self.output = Some(args.next_arg()?),
                "-i" | "--interactive" => self.interactive = true,
                arg if !arg.starts_with('-') => {
                    self.output = Some(args.cur_arg()?)
                }
                _ => {
                    Err(args.err_unknown_argument())?;
                }
            }
        }

        Ok(())
    }

    fn set_encode(&mut self, path: String) -> Result<()> {
        if self.action.is_some() {
            Err(Error::InvalidUsage("Multiple actions specified.".into()))
        } else {
            self.action = Some(Action::Encode(path));
            Ok(())
        }
    }

    fn set_help(&mut self) -> Result<()> {
        if self.action.is_some() {
            Err(Error::InvalidUsage("Multiple actions specified.".into()))
        } else {
            self.action = Some(Action::Help);
            Ok(())
        }
    }

    fn validate(&self) -> Result<()> {
        match &self.action {
            None => return Err(Error::InvalidUsage("Missing action.".into())),
            Some(Action::Help) => {
                if self.interactive {
                    warn!("Useless argument '-i'");
                }
                if self.output.is_some() {
                    warn!("Useless argument '-o'");
                }
            }
            _ => {}
        }

        Ok(())
    }
}
