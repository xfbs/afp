
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
}

impl DataStore {
    pub fn load(_: &str) -> DataStore {
        DataStore {
            sections: vec![
                Section {
                    name: "Technische Kenntnisse der Klasse E".to_string(),
                    short: "Technik E".to_string(),
                    questions: vec![
                        Question {
                            id: "TA101".to_string()
                        }
                    ]
                }
            ]
        }
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
