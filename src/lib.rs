extern crate serde;
extern crate serde_yaml;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

pub struct Question {
    id: String,
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
    questions: String,
    progress: String,
}

impl DataStoreFileSection {
    fn load(self) -> Result<Section, Box<Error>> {
        Ok(Section {
            name: self.name,
            short: self.short,
            questions: vec![]
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

    pub fn count_green(&self) -> usize {
        0
    }

    pub fn count_red(&self) -> usize {
        0
    }

    pub fn count_yellow(&self) -> usize {
        0
    }
}

#[test]
fn test_load_file() {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test/datastore.yaml");
    println!("{:?}", &d);

    let ds = DataStore::load(&d);
    assert!(ds.is_ok());
    let ds = ds.ok().unwrap();
    assert_eq!(&ds.filename, &d);
}
