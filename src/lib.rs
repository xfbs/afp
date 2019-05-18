extern crate serde;
extern crate serde_yaml;
extern crate rand;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use rand::Rng;
use rand::seq::SliceRandom;

pub mod ui;

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    time: SystemTime,
    choice: usize
}

#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    id: String,
    question: String,
    answers: Vec<String>,
    subsection: String,
    subsubsection: String,
    history: Vec<History>
}

#[derive(Debug, PartialEq)]
pub enum QuestionState {
    /// never tried or always wrong
    Red,

    /// correct at least once in last three attempts
    Yellow,

    /// corrent three times last three attampts
    Green
}

//#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    name: String,
    short: String,
    questions: Vec<Question>,
}

pub struct DataStore {
    sections: Vec<Section>,
    filename: PathBuf
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreFile {
    sections: Vec<DataStoreFileSection>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreFileSection {
    name: String,
    short: String,
    questions: Vec<DataStoreQuestion>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreQuestion {
    id: String,
    question: String,
    answers: Vec<String>,
    subsection: String,
    subsubsection: String,
    history: Vec<DataStoreHistory>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataStoreHistory {
    time: u64,
    choice: usize
}


impl DataStoreFileSection {
    fn load(self) -> Result<Section, Box<Error>> {
        Ok(Section {
            name: self.name,
            short: self.short,
            questions: self.questions.into_iter().map(|s| s.load().unwrap()).collect(),
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
            history: self.history.into_iter().map(|s| s.load().unwrap()).collect()
        })
    }
}

impl DataStoreHistory {
    fn load(self) -> Result<History, Box<Error>> {
        Ok(History {
            time: SystemTime::UNIX_EPOCH + Duration::from_secs(self.time),
            choice: self.choice
        })
    }
}


impl DataStore {
    pub fn load(path: &Path) -> Result<DataStore, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let ds: DataStoreFile = serde_yaml::from_reader(reader)?;

        Ok(DataStore {
            // FIXME: error handling.
            sections: ds.sections.into_iter().map(|s| s.load().unwrap()).collect(), 
            filename: path.to_path_buf()
        })
    }

    pub fn save_as(&self, _path: &Path) {
    }

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

    pub fn practise(&self) -> Option<&Question> {
        let mut rng = rand::thread_rng();
        self.questions.choose(&mut rng)
    }
}

impl Question {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answers(&self) -> &Vec<String> {
        &self.answers
    }

    pub fn answer(&mut self, n: usize) {
        self.history.push(History::choose(n))
    }

    pub fn subsection(&self) -> &str {
        &self.subsection
    }

    pub fn subsubsection(&self) -> &str {
        &self.subsubsection
    }

    pub fn state(&self) -> QuestionState {
        let correct_of_last_three = self.history.iter()
            .rev()
            .take(3)
            .map(|h| h.correct())
            .map(|c| if c { 1 } else { 0 })
            .sum();

        match correct_of_last_three {
            3 => QuestionState::Green,
            0 => QuestionState::Red,
            _ => QuestionState::Yellow
        }
    }

    pub fn stale_time(&self) -> Option<Duration> {
        match self.history.last() {
            Some(entry) => entry.time_since(),
            None => None
        }
    }
}

impl History {
    pub fn new(time: SystemTime, choice: usize) -> History {
        History {
            time: time,
            choice: choice
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
            _ => None
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
    assert_eq!(ds.section(0).unwrap().name(), "Technische Kenntnisse der Klasse E");
    assert_eq!(ds.section(1).unwrap().name(), "Technische Kenntnisse der Klasse A");
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
    assert_eq!(ds.section(0).unwrap().questions()[0].question(), "0,042 A entspricht");
    assert_eq!(ds.section(0).unwrap().questions()[0].subsection(), "Allgemeine mathematische Grundkenntnisse und Größen");
    assert_eq!(ds.section(0).unwrap().questions()[0].subsubsection(), "Allgemeine mathematische Grundkenntnisse");
    assert_eq!(ds.section(0).unwrap().questions()[0].answers(), &vec!["40", "41", "42", "43"].into_iter().map(String::from).collect() as &Vec<String>);

    assert_eq!(ds.section(0).unwrap().questions()[0].state(), QuestionState::Red);
    assert_eq!(ds.section(0).unwrap().questions()[1].state(), QuestionState::Red);
    assert_eq!(ds.section(0).unwrap().questions()[2].state(), QuestionState::Yellow);
    assert_eq!(ds.section(0).unwrap().questions()[3].state(), QuestionState::Green);
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
