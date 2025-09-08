use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub type Map = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Default)]
pub struct Config {
    inner: Map,
}

impl From<Map> for Config {
    fn from(value: Map) -> Self {
        Self { inner: value }
    }
}

impl From<Config> for Map {
    fn from(value: Config) -> Self {
        value.inner
    }
}

impl AsRef<Map> for Config {
    fn as_ref(&self) -> &Map {
        &self.inner
    }
}

impl Config {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn try_parse(&mut self, reader: impl BufRead) -> Result<(), io::Error> {
        let mut cursor = None::<String>;
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("[") {
                let cursor_next = line[1..line.len() - 1].trim();
                let next = cursor_next.replace("][", "->");
                cursor = Some(next.clone());
                if !self.inner.contains_key(&next) {
                    self.inner.insert(next, Default::default());
                }
                continue;
            }

            let mut split = line.split("=");
            match (split.next(), &cursor) {
                (Some(key), Some(cursor)) => {
                    let value = split.next().unwrap_or("");
                    self.inner
                        .get_mut(cursor)
                        .unwrap()
                        .insert(key.to_owned(), value.to_owned());
                }
                _ => continue,
            }
        }

        Ok(())
    }

    pub fn merge(&mut self, other: Config) {
        for (key, value) in other.inner {
            if let Some(it) = self.inner.get_mut(&key) {
                it.extend(value);
            } else {
                self.inner.insert(key, value);
            }
        }
    }

    pub fn get(&self, namespace: impl AsRef<str>, name: impl AsRef<str>) -> Option<&str> {
        match self.inner.get(namespace.as_ref()) {
            Some(it) => it.get(name.as_ref()).map(|it| it.as_str()),
            None => None,
        }
    }

    #[inline]
    pub fn as_map(&self) -> &Map {
        self.as_ref()
    }
}
