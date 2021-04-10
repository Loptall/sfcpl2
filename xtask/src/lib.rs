pub(crate) mod cli;
pub(crate) mod util;

use cli::{Add, Bench, Edit, Verify, Xtask};

pub fn exec() -> anyhow::Result<()> {
    Xtask::exec(<Xtask as clap::Clap>::parse())
}

pub fn add(option: Add) -> anyhow::Result<()> {
    let cargo_toml = util::find_cargo_toml("");
    Ok(())
}

pub fn edit(option: Edit) -> anyhow::Result<()> {
    todo!()
}

pub fn bench(option: Bench) -> anyhow::Result<()> {
    todo!()
}
