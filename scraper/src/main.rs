extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate toml;
extern crate reqwest;
extern crate select;

#[macro_use]
extern crate nom;

mod course_entry;
mod term_entry;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::fmt;

use reqwest::Error as ReqwestError;
use toml::de::Error as TOMLError;
use serde_json::error::Error as SerdeError;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name, Text},
};

use course_entry::CourseEntry;
use term_entry::TermEntry;

#[derive(Debug, Deserialize)]
struct ScraperConfig {
    calendars: Vec<Calendar>,
}

#[derive(Debug, Deserialize)]
struct Calendar {
    /// The program slug, used to construct the output filename
    program: String,
    /// The undergraduate calendar URL
    /// e.g. https://ugradcalendar.uwaterloo.ca/page/ENG-Software-Engineering
    url: String,
    /// The year of the first time listed on the undergraduate calendar
    first_term_year: u32,
    /// The co-op work/study sequences supported for this calendar
    /// https://uwaterloo.ca/engineering/future-undergraduate-students/co-op-experience/co-op-studywork-sequences#Standard%20Streams
    /// A separate course plan template will be generated for each stream specified
    /// Leave empty for non-co-op program
    #[serde(default)]
    streams: Vec<WorkStudyStream>,
    /// PD Courses to insert in co-op terms (in order)
    pd: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
enum WorkStudyStream {
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "8")]
    Eight,
}

impl fmt::Display for WorkStudyStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            WorkStudyStream::Four => "stream-4",
            WorkStudyStream::Eight => "stream-8",
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct CoursePlanTemplate {
    terms: Vec<Term>,
}

#[derive(Debug, Clone, Serialize)]
struct Term {
    name: String,
    courses: Vec<TermCourse>,
}

#[derive(Debug, Clone, Serialize)]
struct TermCourse {
    name: String,
}

macro_rules! error_enum {
    (enum $name:ident {
        $($variant:ident($type:ident),)*
    }) => (
        #[derive(Debug)]
        enum $name {
            $($variant($type)),*
        }

        $(
            impl From<$type> for $name {
                fn from(err: $type) -> Self { $name::$variant(err) }
            }
        )*
    );
}

error_enum! {
    enum ScraperError {
        IOError(IOError),
        TOMLError(TOMLError),
        ReqwestError(ReqwestError),
        SerdeError(SerdeError),
    }
}

macro_rules! expect_one {
    ($expr:expr) => (
        {
            let mut iter = $expr;
            let result = iter.next().expect("Expected at least one result");
            iter.next().map(|_| panic!("Expected at most one result"));
            result
        }
    );
}

macro_rules! advance_while {
    ($iter:ident, $cond:expr) => (
        loop {
            match $iter.next() {
                Some(child) => if $cond(child) {
                    break child;
                },
                None => unreachable!("Reached end of iterator without condition being true"),
            }
        }
    );
}

fn main() -> Result<(), ScraperError> {
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, concat!(module_path!(), "=info"));
    env_logger::Builder::from_env(env).init();

    let mut config_file = File::open("scaper.toml")?;
    let mut config_text = String::new();
    config_file.read_to_string(&mut config_text)?;

    let config: ScraperConfig = toml::from_str(&config_text)?;

    for calendar in &config.calendars {
        info!("Fetching '{}'", calendar.url);
        let body = reqwest::get(&calendar.url)?;
        let html = Document::from_read(body)?;

        info!("Extracting information...");
        // .MainContent is arranged as a long list of all of the content in the page.
        // There is no hierarchy so the only option is to search linearly.
        let main_content = expect_one!(html.find(Class("MainContent")));
        let mut children = main_content.children();

        // Advance until we find the Academic Curriculum heading
        advance_while!(children,
            |child: Node| child.name() == Some("h3") && child.text() == "Academic Curriculum");

        // Advance to the second table past that
        let mut table_count = 0;
        let curriculum_table = advance_while!(children, |child: Node| match child.name() {
            Some("table") => {
                table_count += 1;
                table_count == 2
            },
            _ => false,
        });

        let table_body = expect_one!(curriculum_table.find(Name("tbody")));

        let mut plan = CoursePlanTemplate { terms: Vec::new() };
        let mut current_term = None;
        let mut year = calendar.first_term_year;
        for child in table_body.children() {
            // Skip extraneous text (usually whitespace) in between rows
            if child.is(Text) {
                continue;
            }

            let cols: Vec<_> = child.children().filter(|e| e.is(Name("td"))).collect();
            match &cols[..] {
                [term, course, _, _, _] => {
                    if let Some(term) = current_term {
                        plan.terms.push(term);
                    }
                    let term_entry = TermEntry::parse(&term.text());
                    if term_entry.is_calendar_year_start() {
                        year += 1;
                    }
                    current_term = Some(Term {
                        name: term_entry.format_with_year(year),
                        courses: extract_courses(course.text())
                            .into_iter()
                            .map(|name| TermCourse { name })
                            .collect(),
                    });
                },
                [course, _, _, _] |
                [course] => current_term.as_mut()
                    .expect("Expected current_term to be set")
                    .courses
                    .extend(extract_courses(course.text())
                        .into_iter()
                        .map(|name| TermCourse { name })
                        .collect::<Vec<_>>()),
                _ => unreachable!(),
            }
        }
        // Push the last term into the plan
        if let Some(term) = current_term {
            plan.terms.push(term);
        }

        // For non-co-op programs
        if calendar.streams.is_empty() {
            let output_filename = format!("{}_{}-{}.json",
                calendar.program, calendar.first_term_year, calendar.first_term_year + 1);
            info!("Writing output to {}...", output_filename);
            let mut output = File::create(output_filename)?;
            serde_json::to_writer_pretty(output, &plan)?;
        }

        for stream in &calendar.streams {
            let plan = insert_coop_terms(&plan, *stream, &calendar.pd);
            let output_filename = format!("{}_{}-{}_{}.json",
                calendar.program, calendar.first_term_year, calendar.first_term_year + 1, stream);
            info!("Writing output to {}...", output_filename);
            let mut output = File::create(output_filename)?;
            serde_json::to_writer_pretty(output, &plan)?;
        }
    }

    info!("Done.");
    Ok(())
}

fn extract_courses(text: String) -> Vec<String> {
    match CourseEntry::parse(&text) {
        CourseEntry::Course {department_code, course_number, ..} => vec![
            format!("{} {}", department_code, course_number)
        ],
        CourseEntry::Electives {slots, etype, ..} => (1..slots+1).map(|i| {
            format!("{}Elective{}", match etype {
                // Only include the type of the elective if it was provided
                None => "".to_string(),
                Some(s) => format!("{} ", s),
            }, match slots {
                // Do not include an elective number if there is only one
                1 => "".to_string(),
                _ => format!(" {}", i),
            })
        }).collect()
    }
}

/// Inserts co-op terms according to the specification of the stream here:
/// https://uwaterloo.ca/engineering/future-undergraduate-students/co-op-experience/co-op-studywork-sequences#Standard%20Streams
//TODO: All of the per-program exceptions listed further down on that page.
fn insert_coop_terms(plan: &CoursePlanTemplate, stream: WorkStudyStream, pd: &Vec<String>) -> CoursePlanTemplate {
    let terms = &plan.terms;
    assert_eq!(terms.len(), 8, "insert_coop_terms assumes that there are 8 terms");

    let terms = match stream {
        WorkStudyStream::Four => vec![
            terms[0].clone(),
            coop_term(1, pd),
            terms[1].clone(),
            coop_term(2, pd),
            terms[2].clone(),
            coop_term(3, pd),
            terms[3].clone(),
            coop_term(4, pd),
            terms[4].clone(),
            coop_term(5, pd),
            terms[5].clone(),
            coop_term(6, pd),
            terms[6].clone(),
            terms[7].clone(),
        ],
        WorkStudyStream::Eight => vec![
            terms[0].clone(),
            terms[1].clone(),
            coop_term(1, pd),
            terms[2].clone(),
            coop_term(2, pd),
            terms[3].clone(),
            coop_term(3, pd),
            terms[4].clone(),
            coop_term(4, pd),
            terms[5].clone(),
            coop_term(5, pd),
            terms[6].clone(),
            coop_term(6, pd),
            terms[7].clone(),
        ],
    };

    CoursePlanTemplate {terms}
}

fn coop_term(num: u32, pd: &Vec<String>) -> Term {
    let mut courses = vec![TermCourse { name: format!("COOP {}", num) }];
    match pd.get(num as usize - 1) {
        Some(course) => courses.push(TermCourse { name: course.to_string() }),
        None => {},
    }

    //TODO: Would really like these to be "Co-op 1 W19" but don't have code setup for that yet
    Term {
        name: format!("Co-op {}", num),
        courses,
    }
}
