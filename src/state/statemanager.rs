use serenity::model::id::GuildId;
use serenity::prelude::{TypeMapKey};
use std::collections::HashMap;
use std::convert::{AsRef, AsMut};
use anyhow::Error;
use super::ServerState;
use std::fs::{create_dir, read_dir, File, remove_file};
use std::path::{Path, PathBuf};

const STATE_PATH: &str = "State";

#[derive(Debug)]
pub struct StateManager {
    states: HashMap<GuildId, ServerState>
}

impl StateManager {
    pub fn new() -> Result<StateManager, Error> {
        let p = Path::new(STATE_PATH);
        if !p.exists() {
            create_dir(p)?;
        }
        let mut states = HashMap::new();
        for entry in read_dir(p)? {
            let path = entry?.path();

            let full_filename = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            let filename = full_filename
                .split(".")
                .into_iter()
                .next()
                .unwrap();

            let id = GuildId(filename.parse::<u64>()?);

            let f = File::open(&path)?;
            let value = serde_json::from_reader(f)?;

            states.insert(id, value);
        }
        Ok(StateManager { states })
    }

    pub fn get<T>(&mut self, id: &GuildId) -> &T
    where
        T: Default + Into<ServerState>,
        ServerState: AsRef<T>
    {
        if !self.states.contains_key(id) {
            self.states.insert(*id, T::default().into());
        }
        self.states.get(id).unwrap().as_ref()
    }

    pub fn set<T>(&mut self, id: &GuildId, state: T) -> Result<(), Error>
    where
        T: Into<ServerState>
    {
        let path = StateManager::get_path(id);

        let value: ServerState = state.into();

        let f = File::create(path)?;
        serde_json::to_writer(f, &value)?;

        self.states.insert(*id, value);

        Ok(())
    }

    pub fn update<T, F>(&mut self, id: &GuildId, change: F) -> Result<(), Error>
    where
        T: Default + Into<ServerState>,
        ServerState: AsMut<T>,
        F: FnOnce(&mut T)
    {
        let path = StateManager::get_path(id);

        let value = self.states.entry(*id).or_insert_with(|| T::default().into());

        change(value.as_mut());

        let f = File::create(path)?;
        serde_json::to_writer(f, &value)?;

        Ok(())
    }

    pub fn remove(&mut self, id: &GuildId) -> Result<(), Error> {
        let path = StateManager::get_path(id);

        if path.exists() {
            remove_file(path)?;
        }

        self.states.remove(id);
        Ok(())
    }

    fn get_path(id: &GuildId) -> PathBuf {
        let mut path = Path::new(STATE_PATH).to_path_buf();
        let file_name = format!("{:?}.json", id.0);
        path.push(&file_name);
        path
    }
}

impl Default for StateManager {
    fn default() -> Self {
        StateManager::new().unwrap()
    }
}

impl TypeMapKey for StateManager {
    type Value = StateManager;
}