extern crate glob;

use std::path::{Path,PathBuf};

use self::glob::{Pattern,PatternError};

pub struct Filter {
    cwd: PathBuf,
    filters: Vec<Pattern>,
    ignores: Vec<Pattern>
}

impl Filter {
    pub fn new(current_dir: &Path) -> Filter {
        Filter {
            cwd: current_dir.to_path_buf(),
            filters: vec![],
            ignores: vec![]
        }
    }

    pub fn add_filter(&mut self, pattern: &str) -> Result<Pattern, PatternError> {
        let compiled = try!(self.pattern_for(pattern));
        self.filters.push(compiled.clone());

        Ok(compiled)
    }

    pub fn add_ignore(&mut self, pattern: &str) -> Result<Pattern, PatternError> {
        let compiled = try!(self.pattern_for(pattern));
        self.ignores.push(compiled.clone());

        Ok(compiled)
    }

    fn pattern_for(&self, p: &str) -> Result<Pattern, PatternError> {
        let mut path = PathBuf::from(p);
        if path.is_relative() {
            path = self.cwd.join(path.as_path());
        }

        if let Ok(metadata) = path.metadata() {
            if metadata.is_dir() {
                path = path.join("*");
            }
        }

        Pattern::new(path.to_str().unwrap())
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        let path_as_str = path.to_str().unwrap();

        for pattern in &self.ignores {
            if pattern.matches(path_as_str) {
                return true;
            }
        }

        for pattern in &self.filters {
            if pattern.matches(path_as_str) {
                return false;
            }
        }

        self.filters.len() > 0
    }
}