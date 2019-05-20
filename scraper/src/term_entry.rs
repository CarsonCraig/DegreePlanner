//! Utility for parsing a term entry in the undergraduate calendar
//! Term entries follow roughly the following grammar:
//!
//!     term := term_number season
//!     term_number := "1A" | "1B" | "2A" | "2B" | "3A" | "3B" | "4A" | "4B"
//!     season := "Fall" | "Winter" | "Spring"

use std::fmt;

use nom::types::CompleteStr;

pub use nom::Err as ParseError;

type Input<'a> = CompleteStr<'a>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season {
    Fall,
    Winter,
    Spring,
}

impl Season {
    pub fn short(self) -> &'static str {
        match self {
            Season::Fall => "F",
            Season::Winter => "W",
            Season::Spring => "S",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermNumber {
    T1A,
    T1B,
    T2A,
    T2B,
    T3A,
    T3B,
    T4A,
    T4B,
}

impl fmt::Display for TermNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TermNumber::T1A => "1A",
            TermNumber::T1B => "1B",
            TermNumber::T2A => "2A",
            TermNumber::T2B => "2B",
            TermNumber::T3A => "3A",
            TermNumber::T3B => "3B",
            TermNumber::T4A => "4A",
            TermNumber::T4B => "4B",
        })
    }
}

/// Represents a term entry (e.g. "1A Fall")
#[derive(Debug, PartialEq, Eq)]
pub struct TermEntry {
    number: TermNumber,
    season: Season,
}

impl TermEntry {
    pub fn parse(text: &str) -> TermEntry {
        let input = CompleteStr(text.trim());
        match term_entry(input) {
            Ok((remaining, output)) => {
                assert!(remaining.0.is_empty(),
                    "bug: parser did not completely read input for: `{}`\nRemaining: `{}`", text, remaining.0);
                output
            },
            Err(err) => panic!("bug: parse of `{}` failed. Error: {:?}", input, err),
        }
    }

    pub fn is_calendar_year_start(&self) -> bool {
        self.season == Season::Winter
    }

    pub fn format_with_year(&self, year: u32) -> String {
        // Get last two digits of year (will wrap after 2099)
        let year = year - year / 100 * 100;
        format!("{} {}{}", self.number, self.season.short(), year)
    }
}

named!(term_entry(Input) -> TermEntry, ws!(do_parse!(
    number: term_number >>
    season: season >>
    (TermEntry { number, season })
)));

named!(term_number(Input) -> TermNumber, alt!(
    tag!("1A") => { |_| TermNumber::T1A } |
    tag!("1B") => { |_| TermNumber::T1B } |
    tag!("2A") => { |_| TermNumber::T2A } |
    tag!("2B") => { |_| TermNumber::T2B } |
    tag!("3A") => { |_| TermNumber::T3A } |
    tag!("3B") => { |_| TermNumber::T3B } |
    tag!("4A") => { |_| TermNumber::T4A } |
    tag!("4B") => { |_| TermNumber::T4B }
));

named!(season(Input) -> Season, alt!(
    tag!("Fall") => { |_| Season::Fall } |
    tag!("Winter") => { |_| Season::Winter } |
    tag!("Spring") => { |_| Season::Spring }
));
