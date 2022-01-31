use std::env;
use std::error::Error;
use std::net::SocketAddr;

mod config;
mod rgb_commander;
mod utils;

use config::Config;
use rgb_commander::RgbCommander;
use utils::Color;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::load_or_create("config.json").unwrap();

    let address: SocketAddr = config.address.parse().expect("Invalid server address!");
    let default_id = config.default_id.to_string();
    let rgb = RgbCommander::connect(address, default_id).expect("Network error!");

    match args.get(1) {
        None => println!("--print help--"),
        Some(command) => match command.as_str() {
            "ip:get" | "ip" => println!("Current broadcast ip: {}", config.address),
            "ip:set" => {
                let new_ip = args.get(2).expect("Ip missing. Usage: rgb ip:set <new-ip>");
                config.update_address(new_ip)?
            }
            "set" => {
                if args.len() == 6 {
                    let color = get_color(&args, 2);
                    let _ = rgb.set_color(&color);
                } else if args.len() == 3 {
                    let preset = args.get(2).unwrap();
                    let color = config.presets.get(preset).expect("Preset not found!");
                    let _ = rgb.set_color(color);
                } else {
                    println!("Usage: rgb set <preset> or rgb set <r> <g> <b>");
                }
            }
            "preset:add" => {
                let preset_name = args
                    .get(2)
                    .expect("Preset name missing. Usage: rgb preset:add <preset-name> <r> <g> <b>");
                let color = get_color(&args, 3);
                config.add_preset(preset_name, color)?
            }
            "preset:remove" => {
                let preset_name = args.get(2).expect("Preset name missing. Usage: rgb preset:remove <preset-name>");
                config.remove_preset(preset_name)?
            }
            "preset:list" => {
                for (name, color) in config.presets.iter() {
                    println!("{:>5}: {}", name, color);
                }
            }
            "default" | "default:show" => println!("Current default id: {}", config.default_id),
            "default:set" => {
                let new_default_id = args.get(2).expect("Default id missing. Usage: rgb default:set <device-id>");
                config.update_default_id(new_default_id)?
            }

            not_supported_command => println!("Command {} does not exist! --print help--", not_supported_command),
        },
    }
    Ok(())
}

fn get_color(args: &Vec<String>, start_index: usize) -> Color {
    let r = args
        .get(start_index)
        .expect("Color red missing!")
        .parse()
        .expect("Invalid color red!");
    let g = args
        .get(start_index + 1)
        .expect("Color green missing!")
        .parse()
        .expect("Invalid color green!");
    let b = args
        .get(start_index + 2)
        .expect("Color blue missing!")
        .parse()
        .expect("Invalid color blue!");
    Color { r, g, b }
}
