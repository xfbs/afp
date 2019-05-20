extern crate rand;
extern crate serde;
extern crate serde_yaml;

use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub mod ui;

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    time: SystemTime,
    choice: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    id: String,
    question: String,
    answers: Vec<String>,
    subsection: usize,
    subsubsection: usize,
    history: Vec<History>,
}

#[derive(Debug, PartialEq)]
pub enum QuestionState {
    /// never tried or always wrong
    Red,

    /// correct at least once in last three attempts
    Yellow,

    /// corrent three times last three attampts
    Green,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    name: String,
    short: String,
    questions: Vec<Question>,
    subsections: Vec<SubSection>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QuestionFilter {
    All,
    SubSection(usize),
    SubSubSection(usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubSection {
    name: String,
    subsubsections: Vec<SubSubSection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubSubSection {
    name: String,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    sections: Vec<Section>,
    filename: PathBuf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreFile {
    sections: Vec<DataStoreFileSection>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreFileSection {
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

impl QuestionFilter {
    pub fn includes(self, other: QuestionFilter) -> bool {
        match self {
            QuestionFilter::All => true,
            QuestionFilter::SubSection(ss) => match other {
                QuestionFilter::SubSection(ssp) => ss == ssp,
                QuestionFilter::SubSubSection(ssp, _) => ss == ssp,
                _ => false,
            },
            QuestionFilter::SubSubSection(ss, sss) => match other {
                QuestionFilter::SubSubSection(ssp, sssp) => ss == ssp && sss == sssp,
                _ => false,
            },
        }
    }
}

impl DataStoreFileSection {
    fn load(self) -> Result<Section, Box<Error>> {
        Ok(Section {
            name: self.name,
            short: self.short,
            questions: self
                .questions
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
            subsections: self
                .subsections
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
        })
    }
}

impl DataStoreSubSection {
    fn load(self) -> Result<SubSection, Box<Error>> {
        Ok(SubSection {
            name: self.name,
            subsubsections: self
                .subsubsections
                .into_iter()
                .map(|s| SubSubSection { name: s })
                .collect(),
        })
    }
}

impl DataStoreQuestion {
    fn load(self) -> Result<Question, Box<Error>> {
        Ok(Question {
            id: self.id,
            question: self.question,
            answers: self.answers,
            subsection: self.subsection,
            subsubsection: self.subsubsection,
            history: self
                .history
                .into_iter()
                .map(|s| s.load().unwrap())
                .collect(),
        })
    }
}

impl DataStoreHistory {
    fn load(self) -> Result<History, Box<Error>> {
        Ok(History {
            time: SystemTime::UNIX_EPOCH + Duration::from_secs(self.time),
            choice: self.choice,
        })
    }
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore {
            sections: Vec::new(),
            filename: PathBuf::new(),
        }
    }

    pub fn load(path: &Path) -> Result<DataStore, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let ds: DataStoreFile = serde_yaml::from_reader(reader)?;

        Ok(DataStore {
            // FIXME: error handling.
            sections: ds.sections.into_iter().map(|s| s.load().unwrap()).collect(),
            filename: path.to_path_buf(),
        })
    }

    pub fn save_as(&self, _path: &Path) {}

    pub fn save(&self) {
        self.save_as(&self.filename)
    }

    pub fn sections(&self) -> &Vec<Section> {
        &self.sections
    }

    pub fn section(&self, n: usize) -> Option<&Section> {
        self.sections.get(n)
    }

    pub fn section_mut(&mut self, n: usize) -> Option<&mut Section> {
        self.sections.get_mut(n)
    }
}

impl Section {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short(&self) -> &str {
        &self.short
    }

    pub fn count(&self) -> usize {
        self.questions.len()
    }

    pub fn count_by_state(&self, state: QuestionState) -> usize {
        self.questions.iter().filter(|q| q.state() == state).count()
    }

    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn question_mut(&mut self, n: usize) -> Option<&mut Question> {
        self.questions.get_mut(n)
    }

    pub fn question(&self, n: usize) -> Option<&Question> {
        self.questions.get(n)
    }

    /// Find a question that might be a good candidate to practise that
    /// matches the filter.
    pub fn practise(&self, filter: QuestionFilter) -> Option<usize> {
        let candidates = self.questions.iter()
            .enumerate()
            .filter(|(_, question)| filter.includes(question.filter()))
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        let mut rng = rand::thread_rng();
        candidates.choose(&mut rng).map(|v| *v)
    }

    pub fn subsection(&self, n: usize) -> Option<&SubSection> {
        if n == 0 {
            None
        } else {
            self.subsections.get(n - 1)
        }
    }

    pub fn subsections(&self) -> &Vec<SubSection> {
        &self.subsections
    }

    pub fn subsubsection(&self, ss: usize, sss: usize) -> Option<&SubSubSection> {
        match self.subsection(ss) {
            Some(ss) => ss.subsubsection(sss),
            _ => None,
        }
    }

    pub fn state(&self, filter: QuestionFilter) -> QuestionState {
        let mut has_non_red = false;
        let mut is_all_green = true;

        self.questions()
            .iter()
            .filter(|question| filter.includes(question.filter()))
            .for_each(|question| match question.state() {
                QuestionState::Green => {
                    has_non_red = true;
                }
                QuestionState::Yellow => {
                    has_non_red = true;
                    is_all_green = false;
                }
                QuestionState::Red => {
                    is_all_green = false;
                }
            });

        match (has_non_red, is_all_green) {
            (true, true) => QuestionState::Green,
            (true, false) => QuestionState::Yellow,
            (false, _) => QuestionState::Red,
        }
    }
}

impl SubSection {
    /// Get the name of this subsection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retrieves a subsubsection.
    pub fn subsubsection(&self, n: usize) -> Option<&SubSubSection> {
        if n == 0 {
            None
        } else {
            self.subsubsections.get(n - 1)
        }
    }

    /// Gets a list of all subsubsections in this subsection.
    pub fn subsubsections(&self) -> &Vec<SubSubSection> {
        &self.subsubsections
    }
}

impl SubSubSection {
    /// Get the name of this subsubsection.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Represents a single question.
impl Question {
    /// Identifier string of question. Ideally unique.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Question string.
    pub fn question(&self) -> &str {
        &self.question
    }

    /// List of answers (as string) the question has. The first one is the
    /// correct one.
    pub fn answers(&self) -> &Vec<String> {
        &self.answers
    }

    /// Record an answer.
    pub fn answer(&mut self, n: usize) {
        self.history.push(History::choose(n))
    }

    /// Subsection ID of this question.
    pub fn subsection(&self) -> usize {
        self.subsection
    }

    /// Subsubsection ID of this question.
    pub fn subsubsection(&self) -> usize {
        self.subsubsection
    }

    /// State (if the questions is considered to be answered correctly).
    pub fn state(&self) -> QuestionState {
        let correct_of_last_three = self
            .history
            .iter()
            .rev()
            .take(3)
            .map(|h| h.correct())
            .map(|c| if c { 1 } else { 0 })
            .sum();

        match correct_of_last_three {
            3 => QuestionState::Green,
            0 => QuestionState::Red,
            _ => QuestionState::Yellow,
        }
    }

    /// Time since the question has been last answered.
    pub fn stale_time(&self) -> Option<Duration> {
        match self.history.last() {
            Some(entry) => entry.time_since(),
            None => None,
        }
    }

    pub fn filter(&self) -> QuestionFilter {
        match (self.subsection, self.subsubsection) {
            (0, 0) => QuestionFilter::All,
            (ss, 0) => QuestionFilter::SubSection(ss - 1),
            (ss, sss) => QuestionFilter::SubSubSection(ss - 1, sss - 1),
        }
    }
}

impl History {
    pub fn new(time: SystemTime, choice: usize) -> History {
        History {
            time: time,
            choice: choice,
        }
    }

    pub fn choose(choice: usize) -> History {
        Self::new(SystemTime::now(), choice)
    }

    pub fn time(&self) -> SystemTime {
        self.time
    }

    pub fn choice(&self) -> usize {
        self.choice
    }

    pub fn correct(&self) -> bool {
        self.choice == 0
    }

    pub fn time_since(&self) -> Option<Duration> {
        match self.time.elapsed() {
            Ok(duration) => Some(duration),
            _ => None,
        }
    }
}

#[test]
fn test_load_file() {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/datastore.yaml");

    let ds = DataStore::load(&d);
    assert!(ds.is_ok());
    let ds = ds.ok().unwrap();
    assert_eq!(&ds.filename, &d);
}

#[test]
fn test_check_sections() {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/datastore.yaml");
    let ds = DataStore::load(&d).ok().unwrap();

    assert_eq!(ds.sections.len(), 4);
    assert_eq!(
        ds.section(0).unwrap().name(),
        "Technische Kenntnisse der Klasse E"
    );
    assert_eq!(
        ds.section(1).unwrap().name(),
        "Technische Kenntnisse der Klasse A"
    );
    assert_eq!(ds.section(2).unwrap().name(), "Betriebliche Kenntnisse");
    assert_eq!(ds.section(3).unwrap().name(), "Kenntnisse von Vorschriften");

    assert_eq!(ds.section(0).unwrap().short(), "Technik E");
    assert_eq!(ds.section(1).unwrap().short(), "Technik A");
    assert_eq!(ds.section(2).unwrap().short(), "Betrieb");
    assert_eq!(ds.section(3).unwrap().short(), "Vorschriften");

    assert_eq!(ds.section(0).unwrap().questions().len(), 4);
    assert_eq!(ds.section(1).unwrap().questions().len(), 0);
    assert_eq!(ds.section(2).unwrap().questions().len(), 0);
    assert_eq!(ds.section(3).unwrap().questions().len(), 0);
}

#[test]
fn test_check_questions() {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/datastore.yaml");
    let ds = DataStore::load(&d).ok().unwrap();

    assert_eq!(ds.section(0).unwrap().questions().len(), 4);
    assert_eq!(ds.section(0).unwrap().questions()[0].id(), "TA101");
    assert_eq!(
        ds.section(0).unwrap().questions()[0].question(),
        "0,042 A entspricht"
    );
    //assert_eq!(ds.section(0).unwrap().questions()[0].subsection(), "Allgemeine mathematische Grundkenntnisse und Größen");
    //assert_eq!(ds.section(0).unwrap().questions()[0].subsubsection(), "Allgemeine mathematische Grundkenntnisse");
    assert_eq!(
        ds.section(0).unwrap().questions()[0].answers(),
        &vec!["40", "41", "42", "43"]
            .into_iter()
            .map(String::from)
            .collect() as &Vec<String>
    );

    assert_eq!(
        ds.section(0).unwrap().questions()[0].state(),
        QuestionState::Red
    );
    assert_eq!(
        ds.section(0).unwrap().questions()[1].state(),
        QuestionState::Red
    );
    assert_eq!(
        ds.section(0).unwrap().questions()[2].state(),
        QuestionState::Yellow
    );
    assert_eq!(
        ds.section(0).unwrap().questions()[3].state(),
        QuestionState::Green
    );
}

#[test]
fn test_mut_question_state() {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/datastore.yaml");
    let mut ds = DataStore::load(&d).ok().unwrap();

    let section = ds.section_mut(0).unwrap();
    let question = section.question_mut(0).unwrap();

    assert!(question.stale_time().is_none());
    assert_eq!(question.state(), QuestionState::Red);

    question.answer(1);
    assert!(question.stale_time().is_some());
    assert_eq!(question.state(), QuestionState::Red);

    question.answer(0);
    assert_eq!(question.state(), QuestionState::Yellow);
    question.answer(2);
    question.answer(1);
    question.answer(2);
    assert_eq!(question.state(), QuestionState::Red);
    question.answer(0);
    assert_eq!(question.state(), QuestionState::Yellow);
    question.answer(0);
    assert_eq!(question.state(), QuestionState::Yellow);
    question.answer(0);
    assert_eq!(question.state(), QuestionState::Green);
}
