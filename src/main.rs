use std::env;

#[derive(Debug)]
struct Arguments {
    flag: String,
    state: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &str> {
        return parse(args);
    }
}

fn parse(args: &[String]) -> Result<Arguments, &str> {
    // covid19 --help
    if args.len() == 2 {
        if args[1].contains("--help") {
            return Ok(Arguments {
                flag: "--help".to_string(),
                state: "".to_string(),
            });
        } else {
            return Err("unknown flag");
        }
    }

    // covid19 --state ny blah blah
    if args.len() > 4 {
        return Err("too many args");
    }

    return Ok(Arguments {
        flag: args[1].clone(),
        state: args[2].clone(),
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let flags = Arguments::new(&args);

    println!("{:?}", &flags);
}
