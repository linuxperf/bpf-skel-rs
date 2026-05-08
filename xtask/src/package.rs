use anyhow::Result;
use xshell::{Shell, cmd};

pub fn run() -> Result<()> {
    let sh = Shell::new()?;
    cmd!(
        sh,
        "cargo package --workspace --allow-dirty --exclude xtask"
    )
    .run()?;

    Ok(())
}
