//! Utility for parsing a course entry in the undergraduate calendar.
//! Course entries follow roughly the following grammar:
//!
//!     course_entry := course_listing | elective_slots
//!     course_listing := dept_code course_number credit_only? title? footnote? notes?
//!     dept_code := uppercase+
//!     course_number := digit+
//!     title := (letters | digits | whitespace | "-" | ":")+
//!     footnote := "*" | "**" | "***" | "+"
//!     credit_only := "CR/NCR"
//!     notes := "(see note" "s"? digits+ ("," digits+)* (","? "and" digits+)? ")"
//!     elective_slots := number_word? etype? "Elective" "s"? notes?
//!     number_word := "One" | "Two" | "Three" | "Four" | "Five" |
//!                    "Six" | "Seven" | "Eight" | "Nine" | "Ten"
//!     etype := "Communication"

use nom::digit;
use nom::types::CompleteStr;

pub use nom::Err as ParseError;

type Input<'a> = CompleteStr<'a>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Footnote {
    /// * = Alternate weeks
    AlternateWeeks,
    /// ** = One hour seminar per week
    OneHourSeminarPerWeek,
    /// *** = Laboratory is not scheduled and students are expected to find time in open hours to complete their work
    LabNotScheduled,
    /// + = Number of contact hours for the tutorial or laboratory are unknown; there may be more components than the class (LEC) section
    TutOrLabUnknown,
}

/// Represents a single course entry in the Academic Curriculum table of an
/// undergraduate calendar
#[derive(Debug, PartialEq, Eq)]
pub enum CourseEntry<'a> {
    Course {
        /// e.g. ECE, SE, etc.
        department_code: &'a str,
        /// e.g. 000, 100, 200, 345, etc.
        course_number: &'a str,
        /// e.g. Chemistry for Engineers
        title: Option<&'a str>,
        /// true if the course is marked CR/NCR on the calendar
        credit_only: bool,
        /// A note specifying some extra information about a course
        footnote: Option<Footnote>,
        /// Represents "(see notes x and y)"
        notes: Vec<u32>,
    },
    Electives {
        /// The number of elective course slots that this represents
        slots: u32,
        /// Whether this represents a specific type of elective
        etype: Option<&'a str>,
        /// Represents "(see notes x and y)"
        notes: Vec<u32>,
    }
}

impl<'a> CourseEntry<'a> {
    pub fn parse(text: &str) -> CourseEntry {
        let input = CompleteStr(text.trim());
        match course_entry(input) {
            Ok((remaining, output)) => {
                assert!(remaining.0.is_empty(),
                    "bug: parser did not completely read input for: `{}`\nRemaining: `{}`", text, remaining.0);
                output
            },
            Err(err) => panic!("bug: parse of `{}` failed. Error: {:?}", input, err),
        }
    }
}

macro_rules! ws (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, whitespace, $($args)*)
    }
  )
);

named!(course_entry(Input) -> CourseEntry, alt!(
    course_listing |
    elective_slots
));

named!(course_listing(Input) -> CourseEntry, ws!(do_parse!(
    department_code: dept_code >>
    course_number: course_number >>
    credit_only: opt!(credit_only) >>
    title: opt!(title) >>
    footnote: opt!(footnote) >>
    notes: opt!(notes) >>
    (CourseEntry::Course {
        department_code,
        course_number,
        title,
        credit_only: credit_only.unwrap_or(false),
        footnote,
        notes: notes.unwrap_or_else(Vec::new),
    })
)));

named!(dept_code(Input) -> &str, map!(
    take_while1!(|ch: char| ch.is_uppercase()),
    |s| s.0
));

named!(course_number(Input) -> &str, map!(
    call!(digit),
    |s| s.0
));

named!(credit_only(Input) -> bool, map!(
    tag!("CR/NCR"),
    |_| true
));

named!(title(Input) -> &str, map!(
    take_while1!(is_title_char),
    |s| s.0.trim()
));

fn is_title_char(ch: char) -> bool {
    match ch {
        _ if ch.is_alphabetic() => true,
        _ if ch.is_digit(10) => true,
        _ if ch.is_whitespace() => true,
        '-' => true,
        ':' => true,
        _ => false,
    }
}

named!(footnote(Input) -> Footnote, alt!(
    tag!("***") => { |_| Footnote::LabNotScheduled } |
    tag!("**") => { |_| Footnote::OneHourSeminarPerWeek } |
    tag!("*") => { |_| Footnote::AlternateWeeks } |
    tag!("+") => { |_| Footnote::TutOrLabUnknown }
));

named!(notes(Input) -> Vec<u32>, ws!(do_parse!(
    // Need to do this in a tuple to prevent ws! from allowing whitespace in between
    tuple!(tag!("(see note"), opt!(char!('s'))) >>
    first: nat >>
    more: many0!(ws!(preceded!(char!(','), nat))) >>
    end: opt!(ws!(do_parse!(
        opt!(char!(',')) >>
        tag!("and") >>
        value: nat >>
        (value)
    ))) >>
    char!(')') >>
    ({
        let mut result = vec![first];
        result.extend(more);
        if let Some(end) = end {
            result.push(end);
        }
        result
    })
)));

/// Parses a natural number 0, 1, 2, etc.
named!(nat(Input) -> u32,
    flat_map!(call!(digit), parse_to!(u32))
);

named!(elective_slots(Input) -> CourseEntry, ws!(do_parse!(
    slots: opt!(number_word) >>
    etype: opt!(etype) >>
    // Need to do this in a tuple to prevent ws! from allowing whitespace in between
    tuple!(tag!("Elective"), opt!(char!('s'))) >>
    notes: opt!(notes) >>
    (CourseEntry::Electives {
        // Just the word "Elective" means one elective
        slots: slots.unwrap_or(1),
        etype,
        notes: notes.unwrap_or_else(Vec::new),
    })
)));

named!(number_word(Input) -> u32, alt!(
    tag!("One") => { |_| 1 } |
    tag!("Two") => { |_| 2 } |
    tag!("Three") => { |_| 3 } |
    tag!("Four") => { |_| 4 } |
    tag!("Five") => { |_| 5 } |
    tag!("Six") => { |_| 6 } |
    tag!("Seven") => { |_| 7 } |
    tag!("Eight") => { |_| 8 } |
    tag!("Nine") => { |_| 9 } |
    tag!("Ten") => { |_| 10 }
));

named!(etype(Input) -> &str, map!(
    tag!("Communication"),
    |s| s.0
));

named!(whitespace(Input) -> Input, take_while!(|ch: char| ch.is_whitespace()));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parser {
        ($parser:ident ( $input:expr ) -> $expected:expr) => {
            let input = CompleteStr($input);
            match $parser(input) {
                Ok((remaining, output)) => {
                    assert!(remaining.0.is_empty(),
                        "fail: parser did not completely read input for: `{}`\nRemaining: `{}`", $input, remaining.0);
                    assert_eq!(output, $expected, "Incorrect result for parse of input: `{}`", $input);
                },
                Err(err) => panic!("parse of `{}` failed. Error: {:?}", $input, err),
            }
        };
    }

    #[test]
    fn course_entries() {
        // \u{a0} is &nbsp; and the calendar has this in some spots, want to make sure it gets
        // normalized away
        test_parser!(course_entry("CS 137\u{a0}Programming Principles") -> CourseEntry::Course {
            department_code: "CS",
            course_number: "137",
            title: Some("Programming Principles"),
            credit_only: false,
            footnote: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("SE 101\u{a0}Introduction to Methods of Software Engineering**") -> CourseEntry::Course {
            department_code: "SE",
            course_number: "101",
            title: Some("Introduction to Methods of Software Engineering"),
            credit_only: false,
            footnote: Some(Footnote::OneHourSeminarPerWeek),
            notes: Vec::new(),
        });
        test_parser!(course_entry("STAT 206 Statistics for Software Engineering (see note 5)") -> CourseEntry::Course {
            department_code: "STAT",
            course_number: "206",
            title: Some("Statistics for Software Engineering"),
            credit_only: false,
            footnote: None,
            notes: vec![5],
        });
        test_parser!(course_entry("Communication Elective (see note 6)") -> CourseEntry::Electives {
            slots: 1,
            etype: Some("Communication"),
            notes: vec![6],
        });
        test_parser!(course_entry("CS 247 Software Engineering Principles ***") -> CourseEntry::Course {
            department_code: "CS",
            course_number: "247",
            title: Some("Software Engineering Principles"),
            credit_only: false,
            footnote: Some(Footnote::LabNotScheduled),
            notes: Vec::new(),
        });
        test_parser!(course_entry("Elective") -> CourseEntry::Electives {
            slots: 1,
            etype: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("Elective (see note 1)") -> CourseEntry::Electives {
            slots: 1,
            etype: None,
            notes: vec![1],
        });
        test_parser!(course_entry("WKRPT 200 Work-term Report") -> CourseEntry::Course {
            department_code: "WKRPT",
            course_number: "200",
            title: Some("Work-term Report"),
            credit_only: false,
            footnote: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("TPM 000 CR/NCR") -> CourseEntry::Course {
            department_code: "TPM",
            course_number: "000",
            title: None,
            credit_only: true,
            footnote: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("CS 349 User Interfaces ***") -> CourseEntry::Course {
            department_code: "CS",
            course_number: "349",
            title: Some("User Interfaces"),
            credit_only: false,
            footnote: Some(Footnote::LabNotScheduled),
            notes: Vec::new(),
        });
        test_parser!(course_entry("MSCI 261 Engineering Economics:  Financial Management for Engineers") -> CourseEntry::Course {
            department_code: "MSCI",
            course_number: "261",
            title: Some("Engineering Economics:  Financial Management for Engineers"),
            credit_only: false,
            footnote: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("Two Electives") -> CourseEntry::Electives {
            slots: 2,
            etype: None,
            notes: Vec::new(),
        });
        test_parser!(course_entry("Two Electives (see notes 1 and 2)") -> CourseEntry::Electives {
            slots: 2,
            etype: None,
            notes: vec![1, 2],
        });
        test_parser!(course_entry("Two Electives (see notes 1, 2, 3 and 4)") -> CourseEntry::Electives {
            slots: 2,
            etype: None,
            notes: vec![1, 2, 3, 4],
        });
        test_parser!(course_entry("Two Electives (see notes 1, 2, 3, and 4)") -> CourseEntry::Electives {
            slots: 2,
            etype: None,
            notes: vec![1, 2, 3, 4],
        });
        test_parser!(course_entry("Five Electives (see notes 1 and 2)") -> CourseEntry::Electives {
            slots: 5,
            etype: None,
            notes: vec![1, 2],
        });
    }
}
