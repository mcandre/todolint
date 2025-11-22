//! todolint provides predicates for scanning for incomplete tasks.

extern crate clean_path;
extern crate fancy_regex;
extern crate lazy_static;
extern crate mimetype_detector;
extern crate walkdir;

use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::path;
use std::process;

lazy_static::lazy_static! {
    /// TODO_MARKERS collects common incomplete code snippet prefixes.
    pub static ref TODO_MARKERS: Vec<String> = vec![
        // English
        "band aid".to_string(),
        "band-aid".to_string(),
        "bandaid".to_string(),
        "bodge".to_string(),
        "cludge".to_string(),
        "duct tape".to_string(),
        "duct-tape".to_string(),
        "ducttape".to_string(),
        "duck tape".to_string(),
        "duck-tape".to_string(),
        "ducktape".to_string(),
        "hack".to_string(),
        "kludge".to_string(),
        "fixme".to_string(),
        "jury rig".to_string(),
        "jury-rig".to_string(),
        "juryrig".to_string(),
        "macgyver".to_string(),
        "makeshift".to_string(),
        "patch".to_string(),
        "pending".to_string(),
        "rube goldberg".to_string(),
        "rube-goldberg".to_string(),
        "rube goldberg".to_string(),
        "stopgap".to_string(),
        "todo".to_string(),
        "waiting for".to_string(),
        "waiting on".to_string(),
        "workaround".to_string(),

        // Iberian ASCII/Unicode
        "apano".to_string(),
        "apaño".to_string(),
        "chapuza".to_string(),
        "engenhoca".to_string(),
        "gambiarra".to_string(),
        "pend".to_string(),
        "pte".to_string(),
        "quebra galho".to_string(),
        "quebra-galho".to_string(),
        "quebragalho".to_string(),
        "remendo".to_string(),
        "truco".to_string(),

        // Japanese
        "ガラクタ".to_string(),
        "ハック".to_string(),
        "後で".to_string(),
        "急ごしらえ".to_string(),
        "裏技".to_string(),
        "間に合わせ".to_string(),

        // Chinese
        "应付".to_string(),
        "粗笨".to_string(),
    ];

    ///
    /// TODO_PATTERN matches common incomplete code snippets.
    ///
    /// Examples:
    ///
    /// * "pend: pasear al perro"
    /// * "pend.: pasear al perro"
    /// * "pte: pasear al perro"
    /// * "todo: walk the dog"
    /// * "裏技: 犬の散歩"
    /// * "粗笨: 遛狗"
    ///
    /// However, TODO markers that cite a URI-like resource,
    /// such as a FAQ or support ticket,
    /// are permitted.
    ///
    /// Exceptions:
    ///
    /// * "pendente: https://support.google.com/chrome"
    /// * "pendiente: https://support.google.com/chrome"
    /// * "pending: https://support.google.com/chrome"
    /// * "保留: https://support.google.com/chrome"
    /// * "待办: https://support.google.com/chrome"
    ///
    pub static ref TODO_PATTERN: fancy_regex::Regex = fancy_regex::Regex::new(
&format!(
            r"(?i)^.*({})(?!\w|\:\s*.+\:.+).*$",
            TODO_MARKERS.join("|")
        )
    ).unwrap();

    /// JUNK_PATHS collects common third party file paths.
    pub static ref JUNK_PATHS: Vec<String> = vec![
        "node_modules".to_string(),
        "target".to_string(),
        "vendor".to_string(),
    ];

    /// DEFAULT_EXCLUSION_PATTERN matches common third party file paths.
    pub static ref DEFAULT_EXCLUSION_PATTERN: fancy_regex::Regex = fancy_regex::Regex::new(
        &format!(r"^.*(/|\\)?({})$", JUNK_PATHS.join("|"))
    ).unwrap();

    // TEXT_MIMETYPE_PATTERN matches text mimetypes.
    pub static ref TEXT_MIMETYPE_PATTERN: fancy_regex::Regex = fancy_regex::Regex::new("^text/.+$").unwrap();
}

#[test]
fn test_todo_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = TODO_PATTERN.clone();
    assert!(pattern.is_match("TODO")?);
    assert!(pattern.is_match("TODO:")?);
    assert!(pattern.is_match("TODO: walk the dog")?);
    assert!(!pattern.is_match("TODO: https://ticket.test/123")?);
    assert!(pattern.is_match("Todo")?);
    assert!(pattern.is_match("Todo:")?);
    assert!(pattern.is_match("Todo: walk the dog")?);
    assert!(!pattern.is_match("Todo: https://ticket.test/123")?);
    assert!(pattern.is_match("todo")?);
    assert!(pattern.is_match("todo:")?);
    assert!(pattern.is_match("todo: walk the dog")?);
    assert!(!pattern.is_match("todo: https://ticket.test/123")?);
    assert!(pattern.is_match("pend.: passear o cão")?);
    assert!(!pattern.is_match("pending: https://ticket.test/123")?);
    assert!(!pattern.is_match("Let's make a big to do out of it!")?);
    assert!(!pattern.is_match("some have-nots and well-to-dos")?);
    assert!(!pattern.is_match("band")?);
    Ok(())
}

#[test]
fn test_text_mimetype_pattern() -> Result<(), fancy_regex::Error> {
    let pattern = TEXT_MIMETYPE_PATTERN.clone();
    assert!(pattern.is_match("text/markdown")?);
    assert!(pattern.is_match("text/plain")?);
    assert!(pattern.is_match("text/x-c")?);
    assert!(pattern.is_match("text/x-c++")?);
    assert!(!pattern.is_match("application/octet-stream")?);
    Ok(())
}

/// Warning models a TODO finding.
#[derive(Debug)]
pub struct Warning {
    /// path denotes a file path.
    pub path: String,

    /// line_number denotes a line number.
    pub line_number: u64,

    pub line: String,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.path, self.line_number, self.line)
    }
}

/// KirillError models bad computer states.
#[derive(Debug)]
pub enum TodolintError {
    IOError(String),
    DirectoryTraversalError(walkdir::Error),
    UnsupportedPathError(String),
    PathRenderError(String),
    UnknownMimetypeError(String),
    RegexParseError(String),
}

impl fmt::Display for TodolintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodolintError::IOError(e) => write!(f, "{e}"),
            TodolintError::DirectoryTraversalError(e) => write!(f, "{e}"),
            TodolintError::UnsupportedPathError(e) => write!(f, "{e}"),
            TodolintError::PathRenderError(e) => write!(f, "{e}"),
            TodolintError::UnknownMimetypeError(e) => write!(f, "{e}"),
            TodolintError::RegexParseError(e) => write!(f, "{e}"),
        }
    }
}

impl die::PrintExit for TodolintError {
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(die::DEFAULT_EXIT_CODE);
    }
}

/// Linters conducts code quality scans.
#[derive(Debug, Default)]
pub struct Linter {
    /// debug enables additional logging.
    pub debug: bool,
}

impl Linter {
    /// find_text_paths recursively searches
    /// the given directories and/or file root paths
    /// for non-binary file paths.
    pub fn find_text_paths(&self, roots: Vec<&path::Path>) -> Result<Vec<String>, TodolintError> {
        let mut pth_bufs = Vec::<path::PathBuf>::new();

        for root in roots {
            let metadata = fs::metadata(root).map_err(|_| {
                TodolintError::IOError(format!(
                    "unable to query metadata for path: {}",
                    root.display()
                ))
            })?;

            if metadata.is_dir() {
                let walker = walkdir::WalkDir::new(root).sort_by(
                    |a: &walkdir::DirEntry, b: &walkdir::DirEntry| a.file_name().cmp(b.file_name()),
                );

                for entry_result in walker {
                    let entry = entry_result.map_err(TodolintError::DirectoryTraversalError)?;
                    let child_pth: &path::Path = entry.path();

                    if child_pth.is_dir() || child_pth.is_symlink() {
                        continue;
                    }

                    pth_bufs.push(path::PathBuf::from(child_pth));
                }
            } else if metadata.is_file() {
                pth_bufs.push(path::PathBuf::from(root))
            } else {
                return Err(TodolintError::UnsupportedPathError(format!(
                    "unknown type of path: {}",
                    root.display()
                )));
            }
        }

        let mut text_paths = Vec::<String>::new();

        for pth_buf in pth_bufs {
            let pth = pth_buf.as_path();
            let pth_clean_buf = clean_path::clean(pth);
            let pth_clean = pth_clean_buf.as_path();
            let pth_abs = path::absolute(pth_clean).map_err(|_| {
                TodolintError::IOError(format!("unable to resolve path: {}", &pth_clean.display()))
            })?;
            let pth_abs_str = pth_abs
                .to_str()
                .ok_or(TodolintError::PathRenderError(format!(
                    "unable to process path: {}",
                    pth_abs.display()
                )))?;
            let pth_clean_str =
                pth_clean
                    .to_str()
                    .ok_or(TodolintError::PathRenderError(format!(
                        "unable to process path: {}",
                        pth_clean.display()
                    )))?;

            if DEFAULT_EXCLUSION_PATTERN
                .is_match(pth_abs_str)
                .map_err(|e| TodolintError::RegexParseError(e.to_string()))?
            {
                if self.debug {
                    eprintln!("info: excluding path: {pth_clean_str}");
                }

                continue;
            }

            let mimetype = mimetype_detector::detect_file(pth_abs_str)
                .map_err(|e| {
                    TodolintError::IOError(format!(
                        "unable to analyze mimetype from file: {pth_clean_str}: {e}",
                    ))
                })?
                .mime();

            if !TEXT_MIMETYPE_PATTERN
                .is_match(mimetype)
                .map_err(|e| TodolintError::RegexParseError(e.to_string()))?
            {
                if self.debug {
                    eprintln!("info: skipping mimetype: {mimetype}, path: {pth_clean_str}",);
                }

                continue;
            }

            text_paths.push(pth_clean_str.to_string())
        }

        Ok(text_paths)
    }

    pub fn check(&self, pth: String) -> Result<Vec<Warning>, TodolintError> {
        let file = fs::File::open(&pth)
            .map_err(|_| TodolintError::IOError(format!("unable to open file: {}", &pth)))?;
        let reader = io::BufReader::new(file);
        let mut warnings = Vec::<Warning>::new();
        let mut i = 1u64;

        for line_result in reader.lines() {
            let line = line_result.map_err(|_| {
                TodolintError::IOError(format!("unable to read line from file: {}", &pth))
            })?;

            if TODO_PATTERN
                .is_match(&line)
                .map_err(|e| TodolintError::RegexParseError(e.to_string()))?
            {
                let line_trimmed = line.to_string().trim_start().to_string();

                warnings.push(Warning {
                    path: pth.clone(),
                    line_number: i,
                    line: line_trimmed,
                });
            }

            i += 1;
        }

        Ok(warnings)
    }

    /// scan recursively analyzes the given file path roots for TODO warnings.
    pub fn scan(&self, roots: Vec<&path::Path>) -> Result<Vec<Warning>, TodolintError> {
        let text_paths = self.find_text_paths(roots)?;
        let mut warnings = Vec::<Warning>::new();

        for text_path in text_paths {
            warnings.extend(self.check(text_path)?);
        }

        Ok(warnings)
    }
}
