extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct History {
    time: SystemTime,
    choice: usize
}

pub struct Question {
    id: String,
    question: String,
    answers: Vec<String>,
    subsection: String,
    subsubsection: String,
    history: Vec<History>
}

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
    subsubsection: String
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
            history: vec![]
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

    pub fn save_as(&self, path: &Path) {
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

    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
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

    assert_eq!(ds.section(0).unwrap().questions().len(), 1);
    assert_eq!(ds.section(1).unwrap().questions().len(), 0);
    assert_eq!(ds.section(2).unwrap().questions().len(), 0);
    assert_eq!(ds.section(3).unwrap().questions().len(), 0);

    assert_eq!(ds.section(0).unwrap().questions()[0].id(), "TA101");
    assert_eq!(ds.section(0).unwrap().questions()[0].question(), "0,042 A entspricht");
}
