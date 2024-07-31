use std::{
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::{bail, Context};

use crate::volume_control::Volume;

pub const DEFAULT_SINK: &'static str = "@DEFAULT_SINK@";

pub fn set_volume(sink: &str, volume: Volume) -> anyhow::Result<()> {
    let result = Command::new("pactl")
        .arg("set-sink-volume")
        .arg(sink)
        .arg((volume.0 * 100.0).to_string() + "%")
        .status()?;
    if !result.success() {
        bail!("Setting volume failed with status code: {result}")
    }
    return Ok(());
}

pub fn get_volume(sink: &str) -> anyhow::Result<Volume> {
    let result = Command::new("pactl")
        .arg("get-sink-volume")
        .arg(sink)
        .stderr(Stdio::inherit())
        .output()?;
    let result = String::from_utf8(result.stdout)?;
    if !result.starts_with("Volume: ") {
        bail!("Invalid pactl output")
    }
    let pat = regex::Regex::new(r#"(?<number>\d+(.\d+)?)%"#).unwrap();
    let result = pat
        .captures(&result)
        .context("Can't find volume in pactl output")?;
    Ok(Volume(f64::from_str(&result["number"])? / 100.))
}
