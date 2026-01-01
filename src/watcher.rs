//! File Watching Module
//!
//! Provides real-time file system monitoring for automatic index updates.

use crate::index::CodeIndex;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::time::Duration;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = channel();
        
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;
        
        Ok(Self {
            watcher,
            receiver: rx,
        })
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn process_events(
        &self,
        index: Arc<CodeIndex>,
        extensions: Option<Vec<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for res in &self.receiver {
            match res {
                Ok(event) => {
                    self.handle_event(event, &index, &extensions)?;
                }
                Err(e) => eprintln!("Watch error: {e:?}"),
            }
        }
        Ok(())
    }

    fn handle_event(
        &self,
        event: Event,
        index: &CodeIndex,
        extensions: &Option<Vec<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if self.should_index(&path, extensions) {
                        index.index_file(&path)?;
                    }
                }
            }
            EventKind::Remove(_) => {
                // Handle file removal - could remove from index
            }
            _ => {}
        }
        Ok(())
    }

    fn should_index(&self, path: &Path, extensions: &Option<Vec<String>>) -> bool {
        if !path.is_file() {
            return false;
        }
        
        if let Some(exts) = extensions {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                return exts.iter().any(|e| e == ext);
            }
            return false;
        }
        
        true
    }
}

pub fn start_watching(
    path: PathBuf,
    index: Arc<CodeIndex>,
    extensions: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = FileWatcher::new()?;
    watcher.watch(&path)?;
    
    println!("Watching {} for changes...", path.display());
    
    watcher.process_events(index, extensions)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_should_index() {
        let watcher = FileWatcher::new().unwrap();
        let extensions = Some(vec!["rs".to_string(), "py".to_string()]);
        
        let temp_dir = tempdir().unwrap();
        
        let rs_path = temp_dir.path().join("test.rs");
        fs::File::create(&rs_path).unwrap();
        assert!(watcher.should_index(&rs_path, &extensions));
        
        let py_path = temp_dir.path().join("test.py");
        fs::File::create(&py_path).unwrap();
        assert!(watcher.should_index(&py_path, &extensions));
        
        let txt_path = temp_dir.path().join("test.txt");
        fs::File::create(&txt_path).unwrap();
        assert!(!watcher.should_index(&txt_path, &extensions));
    }
}
