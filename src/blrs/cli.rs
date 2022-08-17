/**
 * cli.rs -- Handles command-line parsing, contains entry-point "parse_args()"
 */

use super::Err;
use super::BLRS;

use clap::{
    arg,
    Command
};

/**
 * Organizes the commands into an enum for decoding
 */
pub enum BlrsCommand {
    Get, Inc, Dec, Smooth, Set
}

impl From<&str> for BlrsCommand {
    fn from(s: &str) -> Self {
        match s {
            "get" => BlrsCommand::Get,
            "inc" => BlrsCommand::Inc,
            "dec" => BlrsCommand::Dec,
            "smooth" => BlrsCommand::Smooth,
            "set" => BlrsCommand::Set,
            _ => panic!("blrs:cli.rs:20: undefined command \"{}\" encountered", s)
        }
    }
}

impl Into<String> for BlrsCommand {
    fn into(self) -> String {
        String::from(match self {
            BlrsCommand::Get => "get",
            BlrsCommand::Inc => "inc",
            BlrsCommand::Dec => "dec",
            BlrsCommand::Smooth => "smooth",
            BlrsCommand::Set => "set"
        })
    }
}

/**
 * Uses clap crate to decode args and begin evaluation
 */
pub fn parse_args() -> Result<(), Err> {
    let matches = Command::new("blrs")
        .version("0.1.0")
        .author("wikki")
        .about("CLI for backlight control")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(BlrsCommand::Get)
                .about("Gets current backlight value")
        ).subcommand(
            Command::new(BlrsCommand::Inc)
                .about("Increments backlight")
                .arg(arg!([value] "Increment value of backlight"))
        ).subcommand(
            Command::new(BlrsCommand::Dec)
                .about("Decrements backlight")
                .arg(arg!([value] "Decrement value of backlight"))
        ).subcommand(
            Command::new(BlrsCommand::Smooth)
                .about("Smoothly transitions to given value")  
                .arg(arg!([value] "Target backlight intensity"))
                .arg(arg!([time] "Time to complete animation"))
        ).subcommand(
            Command::new(BlrsCommand::Set)
                .about("Sets backlight value")
                .arg(arg!([value] "Target backlight intensity"))
        )
        .get_matches();

    // Getting subcommand
    let (sub_str, sub_matches) = match matches.subcommand() {
        Some(s) => s,
        _ => unreachable!("Should never occur as subcommand_required=true")
    };

    // Creating BLRS object
    let blrs = BLRS::new()?;

    // Checking if a 'get' call, will not need an arg
    if sub_str == "get" {
        return blrs.sc_get();
    }

    // Parsing Value into a string
    let value_str = sub_matches.get_one::<String>("value")
        .ok_or(Err::UsageSubcommand(sub_str.to_owned()))?;

    // Parsing to u8
    let value = value_str.parse::<usize>()
        .map_err(|_| Err::ValueParse)?;
    
    // Decoding Command
    match sub_str.into() {
        BlrsCommand::Get => {blrs.sc_get()}, // Called above
        BlrsCommand::Inc => {blrs.sc_inc(value)},
        BlrsCommand::Dec => {blrs.sc_dec(value)},
        BlrsCommand::Smooth => {
            // Parsing <time> argument
            let time = sub_matches.get_one::<String>("time")
                .ok_or(Err::UsageSubcommand(sub_str.to_owned()))?
                .parse::<usize>()
                .map_err(|_| Err::ValueParse)?;

            blrs.sc_smooth(value, time)
        },
        BlrsCommand::Set => {blrs.sc_set(value)},
    }
}
