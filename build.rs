use embuild::{
    self,
    build::{CfgArgs, LinkArgs},
};

fn main() -> Result<(), String> {
    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF").map_err(|err| err.to_string())?;

    let cfg = CfgArgs::try_from_env("ESP_IDF").map_err(|err| err.to_string())?;

    cfg.output();

    Ok(())
}
