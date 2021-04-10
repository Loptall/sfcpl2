use std::any;

use clap::Clap;

#[derive(Debug, Clap, Eq, PartialEq)]
pub struct Xtask {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

impl Xtask {
    pub fn exec(self) -> anyhow::Result<()> {
        match self.subcommand {
            SubCommand::Add(add) => super::add(add),
            SubCommand::Bench(bench) => super::bench(bench),
            SubCommand::Edit(edit) => super::edit(edit),
            SubCommand::Verify(_) => todo!(),
        }
    }
}

#[derive(Debug, Clap, Eq, PartialEq)]
enum SubCommand {
    Add(Add),
    Bench(Bench),
    Edit(Edit),
    Verify(Verify),
}

#[derive(Debug, Clap, Eq, PartialEq)]
pub struct Add {
    #[clap(index = 1, default_value = ".")]
    path: String,
    #[clap(short, long)]
    all: bool,
}

#[derive(Debug, Clap, Eq, PartialEq)]
pub struct Bench {
    #[clap(index = 1, default_value = ".")]
    path: String,
    #[clap(short, long)]
    all: bool,
}

#[derive(Debug, Clap, Eq, PartialEq)]
pub struct Edit {
    #[clap(index = 1, default_value = ".")]
    path: String,
    #[clap(short, long)]
    all: bool,
}

#[derive(Debug, Clap, Eq, PartialEq)]
pub struct Verify {
    #[clap(index = 1, default_value = ".")]
    path: String,
    #[clap(short, long)]
    all: bool,
}

mod tests {
    #[test]
    fn parse_basic() {
        use super::{Add, SubCommand, Xtask};
        use clap::Clap;

        let app = Xtask::parse_from(&["xtask", "add", "crates", "--all"]);
        assert_eq!(
            app,
            Xtask {
                subcommand: SubCommand::Add(Add {
                    path: "crates".to_owned(),
                    all: true,
                },),
            }
        );
    }
}
