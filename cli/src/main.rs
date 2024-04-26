use std::net::SocketAddr;

mod config;
mod rgb_commander;
mod utils;

use clap::{Parser, Subcommand};
use config::Config;
use rgb_commander::RgbCommander;
use utils::Color;

#[derive(Parser, Debug)]
#[clap(
    name = "RGB controller CLI",
    version = "2.0",
    about = "CLI For controlling RGB lights",
    author = "Primož Padar"
)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand, Debug)]
enum CliCommands {
    #[clap(about = "Set color - <r> <g> <b> or <preset-name>")]
    Set {
        #[arg(short, long)]
        channel: Option<u8>,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },

    #[clap(about = "Manage broadcast IP settings")]
    Ip {
        #[command(subcommand)]
        _command: IpCommands,
    },

    #[clap(about = "Manage sender channels")]
    Channel {
        #[command(subcommand)]
        _command: ChannelCommands,
    },

    #[clap(about = "Manage color presets")]
    Preset {
        #[command(subcommand)]
        _command: PresetCommands,
    },

    #[clap(about = "Manage config file")]
    Config {
        #[command(subcommand)]
        _command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
enum IpCommands {
    #[clap(about = "Show broadcast IP")]
    Show,
    #[clap(about = "Set broadcast IP")]
    Set { broadcast_ip: String },
}

#[derive(Subcommand, Debug)]
enum ChannelCommands {
    #[clap(about = "Show sender channel")]
    Show,
    #[clap(about = "Set sender channel")]
    Set { channel: u8 },
}

#[derive(Subcommand, Debug)]
enum PresetCommands {
    #[clap(about = "Add preset")]
    Add {
        preset_name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        _args: Vec<String>,
    },
    #[clap(about = "Remove preset")]
    Remove { preset_name: String },
    #[clap(about = "List existing presets")]
    List,
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    #[clap(about = "Show config path")]
    Path,
    #[clap(about = "Show current config")]
    Show,
}

fn main() {
    let args = Cli::parse();

    let mut config = Config::load_or_create("config.json").unwrap();

    let address: SocketAddr = config.address.parse().expect("Invalid server address!");
    let rgb = RgbCommander::connect(address, config.default_channel).expect("Network error!");

    match args.command {
        CliCommands::Set { _args, channel } => {
            if _args.len() == 1 {
                let color = config.presets.get(&_args[0]).expect("Preset not found!");
                let _ = rgb.set_color(color, channel);
            } else if _args.len() == 3 {
                let color = get_color(&_args);
                let _ = rgb.set_color(&color, channel);
            } else {
                panic!("invalid set command check how to use with help")
            }
        }
        CliCommands::Ip { _command } => match _command {
            IpCommands::Show => {
                println!("Current broadcast ip: {}", config.address)
            }
            IpCommands::Set { broadcast_ip } => config.update_address(&broadcast_ip).unwrap(),
        },
        CliCommands::Channel { _command } => match _command {
            ChannelCommands::Show => {
                println!("Default channel: {}", config.default_channel)
            }
            ChannelCommands::Set { channel } => config.update_default_channel(channel).unwrap(),
        },
        CliCommands::Preset { _command } => match _command {
            PresetCommands::Add { preset_name, _args } => {
                let color = get_color(&_args);
                config.add_preset(&preset_name, color).unwrap();
            }
            PresetCommands::Remove { preset_name } => config.remove_preset(&preset_name).unwrap(),
            PresetCommands::List => {
                for (name, color) in config.presets.iter() {
                    println!("{:>5}: {}", name, color);
                }
            }
        },
        CliCommands::Config { _command } => match _command {
            ConfigCommands::Path => {
                println!("{}", config.path)
            }
            ConfigCommands::Show => {
                println!("{}", config.to_string_pretty())
            }
        },
    }
}

fn get_color(args: &[String]) -> Color {
    let r = args.first().expect("Color red missing!").parse().expect("Invalid color red!");
    let g = args.get(1).expect("Color green missing!").parse().expect("Invalid color green!");
    let b = args.get(2).expect("Color blue missing!").parse().expect("Invalid color blue!");
    Color { r, g, b }
}
