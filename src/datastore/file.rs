//! Data structures and methods for loading and storing a DataStore
//! to and from a file.

extern crate serde;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, SystemTime};

use crate::datastore::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreFile {
    pub sections: Vec<DataStoreFileSection>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreFileSection {
    name: String,
    short: String,
    questions: Vec<DataStoreQuestion>,
    subsections: Vec<DataStoreSubSection>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreSubSection {
    name: String,
    subsubsections: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreQuestion {
    id: String,
    question: String,
    answers: Vec<String>,
    subsection: usize,
    subsubsection: usize,
    history: Vec<DataStoreHistory>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreHistory {
    time: u64,
    choice: usize,
}

impl From<&DataStore> for DataStoreFile {
    fn from(datastore: &DataStore) -> DataStoreFile {
        DataStoreFile {
            sections: datastore
                .sections()
                .iter()
                .map(|section| section.into())
                .collect(),
        }
    }
}

impl DataStoreFileSection {
    pub fn load(self) -> Result<Section, Box<Error>> {
        Ok(Section::new(
            self.name,
            self.short,
            self.questions
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
            self.subsections
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
        ))
    }
}

impl From<&Section> for DataStoreFileSection {
    fn from(section: &Section) -> DataStoreFileSection {
        DataStoreFileSection {
            name: section.name().into(),
            short: section.short().into(),
            questions: section
                .questions()
                .iter()
                .map(|question| question.into())
                .collect(),
            subsections: section
                .subsections()
                .iter()
                .map(|subsection| subsection.into())
                .collect(),
        }
    }
}

impl From<&Question> for DataStoreQuestion {
    fn from(question: &Question) -> Self {
        DataStoreQuestion {
            id: question.id().into(),
            question: question.question().into(),
            answers: question.answers().clone(),
            subsection: question.subsection(),
            subsubsection: question.subsubsection(),
            history: question
                .history()
                .iter()
                .map(|history| history.into())
                .collect(),
        }
    }
}

impl From<&History> for DataStoreHistory {
    fn from(history: &History) -> Self {
        DataStoreHistory {
            time: history
                .time()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|time| time.as_secs())
                .unwrap_or(0),
            choice: history.choice(),
        }
    }
}

impl DataStoreSubSection {
    pub fn load(self) -> Result<SubSection, Box<Error>> {
        Ok(SubSection::new(
            self.name,
            self.subsubsections
                .into_iter()
                .map(|s| SubSubSection::new(s))
                .collect(),
        ))
    }
}

impl From<&SubSection> for DataStoreSubSection {
    fn from(subsection: &SubSection) -> Self {
        DataStoreSubSection {
            name: subsection.name().into(),
            subsubsections: subsection
                .subsubsections()
                .iter()
                .map(|subsubsection| subsubsection.into())
                .collect(),
        }
    }
}

impl From<&SubSubSection> for String {
    fn from(subsubsection: &SubSubSection) -> String {
        subsubsection.name().into()
    }
}

impl DataStoreQuestion {
    pub fn load(self) -> Result<Question, Box<Error>> {
        Ok(Question::new(
            self.id,
            self.question,
            self.answers,
            self.subsection,
            self.subsubsection,
            self.history
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
        ))
    }
}

impl DataStoreHistory {
    pub fn load(self) -> Result<History, Box<Error>> {
        Ok(History::new(
            SystemTime::UNIX_EPOCH + Duration::from_secs(self.time),
            self.choice,
        ))
    }
}
