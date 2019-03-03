use std::io;
use std::path::Path;
use std::fs::read_dir;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::cmp::{Ord, Ordering};

use csv::ReaderBuilder;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Eq)]
struct Record {
    user: String,
    quests: u64,
    raids: u64,
    spawns: u64,
}

impl Record {
    pub fn new(user: String) -> Record {
        Record {
            user,
            quests: 0,
            raids: 0,
            spawns: 0,
        }
    }

    pub fn get_key(&self) -> String {
        self.user.clone()
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, other: Record) {
        self.quests += other.quests;
        self.raids += other.raids;
        self.spawns += other.spawns;
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        // spawns are used only to discriminate
        // reverse order
        match (self.quests + self.raids).cmp(&(other.quests + other.raids)) {
            Ordering::Equal => self.spawns.cmp(&other.spawns),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.quests == other.quests && self.raids == other.raids && self.spawns == other.spawns
    }
}

fn read_csv(file: &Path, res: &mut HashMap<String, Record>) -> io::Result<()> {
    let mut rdr = ReaderBuilder::new()
                                .delimiter(b';')
                                .from_path(file)
                                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        *res.entry(record.get_key()).or_insert_with(|| Record::new(record.get_key())) += record;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut res = HashMap::new();
    for entry in read_dir("data")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            read_csv(&path, &mut res)?;
        }
    }
    let mut values: Vec<_> = res.values().collect();
    values.sort_unstable();
    println!("{:#?}", values);
    Ok(())
}
