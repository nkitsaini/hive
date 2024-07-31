mod pactl;
mod volume_control;
mod wpctl;
use clap::{Parser, ValueEnum};
use volume_control::Volume;

use change::Change;
mod change;

const MAX_VOLUME: f64 = 2.0;

#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum VolumeController {
    Wpctl,
    Pactl,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// change in volume (1 means full). like: "+0.5", "-0.5", "+0.5%", "-0.01%"
    #[arg(value_parser = clap::value_parser!(Change))]
    volume_change: Change,

    /// Set the sink name, default sink by wireplumber is used otherwise
    #[arg(short, long, default_value = "@DEFAULT_AUDIO_SINK@")]
    wpctl_sink: String,

    #[arg(short, long, default_value = "@DEFAULT_SINK@")]
    pactl_sink: String,

    /// Tries wpctl than pactl if none is specified
    #[arg(short, long, default_value = Option::None)]
    controller: Option<VolumeController>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("Parsed: {:?}", cli.volume_change);

    let current_volume: Volume;
    let controller: VolumeController;

    match cli.controller {
        Some(VolumeController::Wpctl) => {
            controller = VolumeController::Wpctl;
            current_volume = wpctl::get_volume(&cli.wpctl_sink)?;
        }
        Some(VolumeController::Pactl) => {
            controller = VolumeController::Pactl;
            current_volume = pactl::get_volume(&cli.pactl_sink)?;
        }
        None => {
            current_volume = match wpctl::get_volume(&cli.wpctl_sink) {
                Ok(x) => {
                    controller = VolumeController::Wpctl;
                    x
                }
                Err(_) => {
                    controller = VolumeController::Pactl;
                    pactl::get_volume(&cli.pactl_sink)?
                }
            };
        }
    }

    println!("> Using controller: {controller:?}");
    println!("> Current volume: {}", current_volume.0);

    let new_volume = cli
        .volume_change
        .apply(current_volume.0)
        .min(MAX_VOLUME)
        .max(0.);

    let new_volume = Volume((new_volume * 100.).round() / 100.);

    println!("> Setting new volume: {}", new_volume.0);
    match controller {
        VolumeController::Wpctl => wpctl::set_volume(&cli.wpctl_sink, new_volume)?,
        VolumeController::Pactl => pactl::set_volume(&cli.pactl_sink, new_volume)?,
    };
    println!("> Done");
    Ok(())
}
