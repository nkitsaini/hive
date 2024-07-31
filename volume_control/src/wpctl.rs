use std::{
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::bail;

use crate::volume_control::Volume;

pub const DEFAULT_SINK: &'static str = "@DEFAULT_AUDIO_SINK@";

pub fn set_volume(sink: &str, volume: Volume) -> anyhow::Result<()> {
    let result = Command::new("wpctl")
        .arg("set-volume")
        .arg(sink)
        .arg(volume.0.to_string())
        .status()?;
    if !result.success() {
        bail!("Setting volume failed with status code: {result}")
    }
    return Ok(());
}

pub fn get_volume(sink: &str) -> anyhow::Result<Volume> {
    let result = Command::new("wpctl")
        .arg("get-volume")
        .arg(sink)
        .stderr(Stdio::inherit())
        .output()?;
    let result = String::from_utf8(result.stdout)?;
    if !result.starts_with("Volume: ") {
        bail!("Invalid wpctl output")
    }
    let result = f64::from_str(result.replace("Volume: ", "").trim())?;
    return Ok(Volume(result));
}
