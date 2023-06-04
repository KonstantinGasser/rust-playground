use std::{env, net::IpAddr, str::FromStr};

const FLAG_HELP_SHORT: &str = "-h";
const FLAG_TARGET_SHORT: &str = "-t";
const FLAG_THREAD_COUNT_SHORT: &str = "-p";

const DEFAULT_THREAD_COUNT: i8 = 4;

pub enum Err {
    ErrHelpRequested,
    ErrInvalidIpAddr,
    ErrNaN,
}

impl Err {
    pub fn message(self) -> String {
        match self {
            Err::ErrHelpRequested => "Help!".to_string(),
            Err::ErrInvalidIpAddr => "The provided IP address is not valid".to_string(),
            Err::ErrNaN => "The provided thread count is not a number".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Arguments {
    target: String,
    threads: i8,
}

impl Arguments {
    pub fn new() -> Result<Self, Err> {
        let os_args: Vec<String> = env::args().collect();

        let mut argument = Arguments {
            target: "invalid".to_owned(),
            threads: DEFAULT_THREAD_COUNT,
        };

        for arg in os_args.iter() {
            if arg.eq(FLAG_HELP_SHORT) {
                return Err(Err::ErrHelpRequested);
            }

            if arg.eq(FLAG_TARGET_SHORT) {
                let ip = match parse_ip_addr(&arg) {
                    Ok(ip) => ip.clone(),
                    Err(_) => return Err(Err::ErrInvalidIpAddr),
                };
                argument.target = ip.to_string();
            }

            if arg.eq(FLAG_THREAD_COUNT_SHORT) {
                let count = match arg.parse::<i8>() {
                    Ok(count) => count,
                    Err(_) => return Err(Err::ErrNaN),
                };
                argument.threads = count;
            }
        }

        Ok(argument)
    }
}

fn parse_ip_addr(string: &str) -> Result<IpAddr, std::net::AddrParseError> {
    IpAddr::from_str(string)
}
