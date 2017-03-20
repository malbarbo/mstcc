// system
use std::str::FromStr;
use std::num::ParseIntError;

// external
use clap::Arg;
use rand::{self, Rng, SeedableRng, XorShiftRng};

pub fn partition<T, F>(xs: &mut [T], mut pred: F) -> usize
    where F: FnMut(&T) -> bool
{
    if xs.is_empty() {
        return 0;
    }

    let mut i = 0;
    let mut j = xs.len() - 1;
    while i < j {
        if pred(&xs[i]) {
            i += 1;
        } else {
            xs.swap(i, j);
            j -= 1;
        }
    }

    i
}


// Log

pub fn init_logger(level: &str) {
    use env_logger::LogBuilder;
    use log::LogLevelFilter;

    if let Ok(filter) = LogLevelFilter::from_str(level) {
        LogBuilder::new()
            .filter(None, filter)
            .init()
            .expect("Init logger");

        debug!("Logging at {:?} level", filter);
    }
}


pub fn log_improvement(target: &str, old: u32, new: u32) {
    debug!(target: target, "{} -> {} ({:.02}%)", old, new, improvement_percentage(old, new));
}

pub fn log_improvement_best(target: &str, old: u32, new: u32) {
    info!(target: target, "best {} -> {} ({:.02}%)", old, new, improvement_percentage(old, new));
}


fn improvement_percentage(old: u32, new: u32) -> f64 {
    if old == 0 {
        0.0
    } else if old > new {
        100.0 * (old - new) as f64 / old as f64
    } else {
        -100.0 * (new - old) as f64 / old as f64
    }
}


// Args

#[derive(Copy, Clone, Debug)]
pub struct Seed(u32);

impl FromStr for Seed {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seed(s.parse()?))
    }
}

impl Seed {
    pub fn new_random() -> Seed {
        Seed(rand::weak_rng().gen())
    }

    pub fn new_xor_shift_rng(&self) -> XorShiftRng {
        let s = self.0;
        XorShiftRng::from_seed([s, s.wrapping_add(1), s.wrapping_add(2), s.wrapping_add(3)])
    }
}


pub fn arg_seed() -> Arg<'static, 'static> {
    Arg::with_name("seed")
        .short("s")
        .long("seed")
        .takes_value(true)
        .help("The seed used in the random number generator. \
              A random value is used none is specified")
}

pub fn arg_log() -> Arg<'static, 'static> {
    Arg::with_name("level")
        .long("log")
        .takes_value(true)
        .possible_values(&["off", "info", "debug"])
        .default_value("off")
        .help("Enable logging to stderr")
}

pub fn arg_input() -> Arg<'static, 'static> {
    Arg::with_name("input")
        .takes_value(true)
        .required(true)
        .help("The input file")
}
