use std::fs::{self, File};
use std::path::Path;
use std::str::FromStr;
use std::fmt;

use serde_json;
use serde_json::error::Error as SerdeError;

#[derive(Debug)]
pub struct UnknownTemplate<'a>(&'a str);

impl<'a> fmt::Display for UnknownTemplate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown course plan template: {}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Term {
    pub name: String,
    pub courses: Vec<TermCourse>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TermCourse {
    pub name: String,
}

/// Represents a template course plan for a given program
#[derive(Debug, Clone, Deserialize)]
pub struct CoursePlanTemplate {
    pub terms: Vec<Term>,
}

impl CoursePlanTemplate {
    /// Parses a course plan template from the template associated with the given template
    /// identifier.
    pub fn from_template(template_id: &str) -> Result<Self, UnknownTemplate> {
        let template_dir = Path::new("templates");
        // It is very important to validate that this is a recognized course plan template because
        // the template identifier provided may be from an untrusted source. It would be very bad
        // if someone found a way to open an arbitrary file on the file system.
        let template_file = String::from(template_id) + ".json";
        let mut found = false;
        for file in fs::read_dir(template_dir).expect("Unable to read template directory") {
            let filename = file.expect("Unable to read template filename").file_name()
                .into_string().expect("Invalid unicode in template filename");
            if filename == template_file {
                found = true;
            }
        }

        if !found {
            return Err(UnknownTemplate(template_id));
        }

        let mut path = template_dir.to_path_buf();
        path.push(template_file);
        let file = File::open(path).expect("Unable to open template");
        Ok(serde_json::from_reader(file).expect("Unable to deserialize template"))
    }
}

impl FromStr for CoursePlanTemplate {
    type Err = SerdeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(text)
    }
}
