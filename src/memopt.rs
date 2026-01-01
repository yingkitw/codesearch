//! Memory-Optimized File Reading Module
//!
//! Provides memory-efficient file reading for very large files using memory mapping.

use memmap2::Mmap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10 MB

pub enum FileReader {
    Buffered(BufReader<File>),
    Mapped(Mmap),
}

impl FileReader {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        
        if metadata.len() > LARGE_FILE_THRESHOLD {
            let mmap = unsafe { Mmap::map(&file)? };
            Ok(FileReader::Mapped(mmap))
        } else {
            Ok(FileReader::Buffered(BufReader::new(file)))
        }
    }

    pub fn read_lines(&mut self) -> Box<dyn Iterator<Item = String> + '_> {
        match self {
            FileReader::Buffered(reader) => {
                Box::new(std::io::BufRead::lines(reader).filter_map(|line| line.ok()))
            }
            FileReader::Mapped(mmap) => {
                let content = String::from_utf8_lossy(mmap);
                Box::new(content.lines().map(|s| s.to_string()).collect::<Vec<_>>().into_iter())
            }
        }
    }

    pub fn search_pattern(
        &mut self,
        pattern: &regex::Regex,
        max_results: usize,
    ) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        let mut line_number = 0;

        for line in self.read_lines() {
            line_number += 1;
            if pattern.is_match(&line) {
                results.push((line_number, line));
                if results.len() >= max_results {
                    break;
                }
            }
        }

        results
    }

    pub fn search_streaming(
        &mut self,
        pattern: &regex::Regex,
        max_results: usize,
    ) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        let mut line_num = 1;

        for line in self.read_lines() {
            if results.len() >= max_results {
                break;
            }

            if pattern.is_match(&line) {
                results.push((line_num, line));
            }
            line_num += 1;
        }

        results
    }
}

pub fn read_file_chunked<F>(
    path: &Path,
    chunk_size: usize,
    mut processor: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&str) -> bool,
{
    let mut reader = FileReader::new(path)?;
    let mut buffer = String::with_capacity(chunk_size);
    let mut continue_processing = true;

    for line in reader.read_lines() {
        if !continue_processing {
            break;
        }

        buffer.push_str(&line);
        buffer.push('\n');

        if buffer.len() >= chunk_size {
            continue_processing = processor(&buffer);
            buffer.clear();
        }
    }

    if !buffer.is_empty() && continue_processing {
        processor(&buffer);
    }

    Ok(())
}

pub struct StreamingSearcher {
    pattern: regex::Regex,
    max_results: usize,
    results: Vec<(usize, String)>,
}

impl StreamingSearcher {
    pub fn new(pattern: regex::Regex, max_results: usize) -> Self {
        Self {
            pattern,
            max_results,
            results: Vec::new(),
        }
    }

    pub fn search_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = FileReader::new(path)?;
        let mut line_number = 0;

        for line in reader.read_lines() {
            line_number += 1;
            
            if self.results.len() >= self.max_results {
                break;
            }

            if self.pattern.is_match(&line) {
                self.results.push((line_number, line));
            }
        }

        Ok(())
    }

    pub fn get_results(&self) -> &[(usize, String)] {
        &self.results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_reader_small_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1\nline 2\nline 3").unwrap();
        
        let mut reader = FileReader::new(file.path()).unwrap();
        let lines: Vec<String> = reader.read_lines().collect();
        
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_streaming_searcher() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "fn main() {{\n    println!(\"test\");\n}}").unwrap();
        
        let pattern = regex::Regex::new(r"fn\s+\w+").unwrap();
        let mut searcher = StreamingSearcher::new(pattern, 10);
        
        searcher.search_file(file.path()).unwrap();
        let results = searcher.get_results();
        
        assert_eq!(results.len(), 1);
        assert!(results[0].1.contains("fn main"));
    }

    #[test]
    fn test_search_pattern() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "fn test1() {{}}\nfn test2() {{}}\nfn test3() {{}}").unwrap();
        
        let mut reader = FileReader::new(file.path()).unwrap();
        let pattern = regex::Regex::new(r"fn\s+\w+").unwrap();
        let results = reader.search_pattern(&pattern, 2);
        
        assert_eq!(results.len(), 2);
    }
}
