use advent_of_code::template::commands;

mod args {
    use std::process;

    pub enum Arguments {
        Download {
            day: u8,
        },
        Read {
            day: u8,
        },
        Scaffold {
            day: u8,
        },
        Solve {
            day: u8,
            release: bool,
            time: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
            time: bool,
        },
    }

    pub fn parse() -> Result<Arguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => Arguments::All {
                release: args.contains("--release"),
                time: args.contains("--time"),
            },
            Some("download") => Arguments::Download {
                day: args.free_from_str()?,
            },
            Some("read") => Arguments::Read {
                day: args.free_from_str()?,
            },
            Some("scaffold") => Arguments::Scaffold {
                day: args.free_from_str()?,
            },
            Some("solve") => Arguments::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                time: args.contains("--time"),
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    match args::parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            args::Arguments::All { release, time } => commands::all::handler(release, time),
            args::Arguments::Download { day } => commands::download::handler(day),
            args::Arguments::Read { day } => commands::read::handler(day),
            args::Arguments::Scaffold { day } => commands::scaffold::handler(day),
            args::Arguments::Solve {
                day,
                release,
                time,
                submit,
            } => commands::solve::handler(day, release, time, submit),
        },
    };
}
