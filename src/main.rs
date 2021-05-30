use std::convert::TryInto;

/*
fov -v 16:9 90  # get vertical FOV from 90 degrees horizontal at 16:9
fov -h 4:3 55.5  # get horizontal FOV from 55 degrees vertical at 4:3

-h, --horizontal
-v, --vertical
--version
--help
*/
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // TODO: come up with an error message formatter function
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::FloatingFrees);
    opts.long_only(false);

    opts.optflag("v", "vertical", "Convert horizontal FOV to vertical FOV");
    opts.optflag("h", "horizontal", "Convert vertical FOV to horizontal FOV");
    opts.optflag("", "version", "Display version information");
    opts.optflag("?", "help", "Display usage information");

    let matches = opts.parse(&args[1..])?;
    if matches.opt_present("?") {
        print_help();
        return Ok(());
    } else if matches.opt_present("version") {
        print_version();
        return Ok(());
    }

    let mut config = cfov::Config::default();

    config.output_fov_type = if matches.opt_present("v") {
        if matches.opt_present("h") {
            // stdlib includes `impl From<String> for Box<dyn Error>`
            return Err("Cannot specify both -h and -v. Use --help for usage information".into());
        }
        cfov::FovType::VERTICAL
    } else if matches.opt_present("h") {
        cfov::FovType::HORIZONTAL
    } else {
        return Err("Must specify either -h or -v. Use --help for usage information".into());
    };

    if matches.free.len() != 2 {
        return Err("Must specify aspect ratio and input FOV. Use --help for usage information".into());
    }

    config.aspect_ratio = matches.free[0][..].try_into()?;
    config.fov = matches.free[1][..].try_into()?;

    Ok(cfov::run(&config)?)
}

fn print_help() {
    // TODO: put real text here
    println!("placeholder help text");
}

fn print_version() {
    // TODO: put real version here
    println!("placeholder version text");
}
