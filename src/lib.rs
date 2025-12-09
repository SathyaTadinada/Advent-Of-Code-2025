pub mod input;
pub mod solutions;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Day not implemented: {0}.")]
    DayNotImplemented(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
    Both,
}

impl Part {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Part::One),
            2 => Some(Part::Two),
            _ => None,
        }
    }
}

pub struct DayRunner {
    pub day: u32,
    pub run_fn: fn(&str, Part),
}

impl DayRunner {
    pub fn run(&self, input: &str, which: Part) {
        (self.run_fn)(input, which);
    }
}

pub fn extract_day_number(path: &str) -> u32 {
    path.split(['/', '\\'])
        .find_map(|comp| {
            if let Some(rest) = comp.strip_prefix("day") {
                let num = rest.split('.').next().unwrap_or(rest);
                num.parse::<u32>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0)
}

pub fn run_day(input_dir: &str, day: u32, part: Part) -> Result<(), Error> {
    let entry = solutions::REGISTRY
        .iter()
        .find(|d| d.day == day)
        .ok_or(Error::DayNotImplemented(day))?;

    let input = input::load(input_dir, day);
    entry.run(&input, part);
    Ok(())
}

pub fn run_all(input_dir: &str) {
    for entry in solutions::REGISTRY {
        let input = input::load(input_dir, entry.day);
        entry.run(&input, Part::Both);
    }
}
