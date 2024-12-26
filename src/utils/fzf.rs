use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::Write,
};

#[derive(Debug)]
pub enum SpawnError {
    IOError(std::io::Error),
}

pub struct Fzf {
    pub executable: String,
    pub args: Vec<String>,
}

impl Fzf {
    pub fn new() -> Self {
        Self {
            executable: "fzf".to_string(),
            args: vec![],
        }
    }
}

#[derive(Default)]
pub struct FzfArgs {
    pub print_query: Option<String>,
    pub header: Option<String>,
    pub reverse: bool,
    pub preview: Option<String>,
    pub with_nth: Option<usize>,
    pub ignore_case: bool,
    pub query: Option<String>,
    pub cycle: bool,
    pub delimiter: Option<String>,
    pub preview_window: Option<String>,
}

impl Error for SpawnError {}
impl Display for SpawnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

pub trait FzfSpawn {
    fn spawn(&mut self, args: FzfArgs) -> Result<std::process::Child, SpawnError>;
}

impl FzfSpawn for Fzf {
    fn spawn(&mut self, args: FzfArgs) -> Result<std::process::Child, SpawnError> {
        let mut temp_args = self.args.clone();

        if let Some(header) = args.header {
            temp_args.push(format!("--header={}", header));
        }

        if args.reverse {
            temp_args.push("--reverse".to_string());
        }

        if let Some(preview) = args.preview {
            temp_args.push(format!("--preview={}", preview));
        }

        if let Some(with_nth) = args.with_nth {
            temp_args.push(format!("--with-nth={}", with_nth));
        }

        if args.ignore_case {
            temp_args.push("--ignore-case".to_string());
        }

        if let Some(query) = args.query {
            temp_args.push(format!("--query={}", query));
        }

        if args.cycle {
            temp_args.push("--cycle".to_string());
        }

        if let Some(delimiter) = args.delimiter {
            temp_args.push(format!("--delimiter={}", delimiter));
        }

        if let Some(preview_window) = args.preview_window {
            temp_args.push(format!("--preview-window={}", preview_window));
        }

        let mut command = std::process::Command::new(&self.executable);
        command.args(&temp_args);

        if let Some(print_query) = args.print_query {
            command.stdin(std::process::Stdio::piped());
            let mut child = command.spawn().map_err(SpawnError::IOError)?;

            if let Some(stdin) = child.stdin.as_mut() {
                writeln!(stdin, "{}", print_query).map_err(SpawnError::IOError)?;
            }

            Ok(child)
        } else {
            command.spawn().map_err(SpawnError::IOError)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::fzf::{Fzf, FzfArgs, FzfSpawn};

    #[test]
    fn test_fzf_spawn() {
        let args = FzfArgs {
            print_query: Some("Hello\nWorld".to_string()),
            delimiter: Some(String::from("\t")),
            ..Default::default()
        };

        let mut fzf = Fzf::new();
        let mut child = fzf.spawn(args).unwrap();
        assert_eq!(
            child
                .wait()
                .expect("Failed to spawn child process for fzf")
                .code(),
            Some(0)
        )
    }
}
