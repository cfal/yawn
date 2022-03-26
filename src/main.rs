mod to_duration;
mod wait_spec;

use log::debug;

use crate::to_duration::ToDuration;
use crate::wait_spec::WaitSpec;

#[derive(Debug, Clone)]
struct YawnConfig {
    wait_spec: WaitSpec,
    exit_code: i32,
}

fn help_str(command: &str) -> String {
    const HELP_STR: &str = "USAGE:

    {} [OPTIONS] <DURATION or TIMESTAMP>

OPTIONS:

    -e, --exit-code NUM
        Exit code, defaults to 0.

DURATION or TIMESTAMP:

    Durations are of the format:

        any positive integer or float are interpreted as seconds (eg 123, 123.45)

        a single string with 'd', 'h', 'm', 's', 'ms' as units, corresponding to days,
        hours, minutes, seconds, and milliseconds. (eg 1d2h3m4s5ms, 1d15m)

    Exact timestamps are of the format:

        RFC3339                (eg 1996-12-19T16:39:57-08:00)
        RFC2822                (eg Tue, 1 Jul 2003 10:52:37 +0200)
        YYYY-mm-dd HH:MM:SS    [local time] (eg 2005-01-03 12:23:34)
        YYYY-mm-dd HH:MM       (eg. 2005-01-03 12:23)

    Inexact-timestamps are of the format:

        HH:MM                  [24-hour format] (eg 14:23)
        HH:MM:SS               [24-hour format] (eg 14:23:45)
        HH:MM:SS.nnn           [24-hour format] (eg 14:23:45.678)
        h:MM P                 [12-hour format] (eg 8:12 am)

    Inexact timestamps always refer to next upcoming date.

EXAMPLES:

    {} 1d2h3m4s

    Sleep for 1 day, 2 hours, 3 minutes, and 4 seconds.

    {} '8:00 am'

    Sleep until the upcoming 8 am.
";

    HELP_STR.replace("{}", &command)
}

fn main() {
    env_logger::init();

    let argv = std::env::args().collect::<Vec<_>>();
    let (command, args) = argv.split_first().unwrap();

    let config = match create_config(args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            eprintln!("");
            eprintln!("{}", help_str(&command));
            std::process::exit(1);
        }
    };

    debug!("{:?}", config);

    run_wait_spec(config.wait_spec);
    std::process::exit(config.exit_code);
}

fn create_config(args: &[String]) -> std::result::Result<YawnConfig, String> {
    let mut exit_code: Option<i32> = None;
    let mut wait_spec: Option<WaitSpec> = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-e" || arg == "--exit-code" {
            i += 1;
            if i >= args.len() {
                return Err(String::from("Exit code not specified"));
            }
            let code = args[i]
                .parse::<i32>()
                .map_err(|_| format!("Invalid exit code value: {}", args[i]))?;
            exit_code = Some(code);
        } else {
            wait_spec = Some(arg.as_str().try_into()?);
        }
        i += 1;
    }

    if wait_spec.is_none() {
        return Err(String::from("No wait params"));
    }

    Ok(YawnConfig {
        wait_spec: wait_spec.unwrap(),
        exit_code: exit_code.unwrap_or(0),
    })
}

fn run_wait_spec(wait_spec: WaitSpec) {
    let duration = match wait_spec {
        WaitSpec::Duration(d) => d,
        WaitSpec::DateTime(dt) => dt.to_duration(),
        WaitSpec::NaiveTime(nt) => nt.to_duration()
    };
    std::thread::sleep(duration);
}
