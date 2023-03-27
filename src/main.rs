use std::{ffi::OsStr, fmt, path::Path, process};

use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use fk_lib::kill;

struct ListItem {
    pid: u32,
    comm: String,
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comm = Path::new(&self.comm)
            .file_name()
            .unwrap_or(OsStr::new(&self.comm))
            .to_str()
            .unwrap_or("");
        write!(f, "{}\t{}", self.pid, comm)
    }
}

fn main() {
    let output = process::Command::new("ps")
        .arg("wwxo")
        .arg("pid,comm")
        .output()
        .expect("failed to run process status (ps) utility command");

    let mut items = Vec::new();
    for process_line in String::from_utf8_lossy(&output.stdout).split("\n").skip(1) {
        items.push(ListItem {
            pid: process_line.split_whitespace().next().unwrap_or("0").parse::<u32>().unwrap_or(0),
            comm: String::from(process_line),
        })
    }

    let selection_idx = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap_or(None);

    match selection_idx {
        Some(idx) => kill(items[idx].pid),
        None => process::exit(0),
    }
}
