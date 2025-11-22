//! todolint provides predicates for scanning for incomplete tasks.

extern crate clean_path;
extern crate lazy_static;
extern crate mimetype_detector;
extern crate regex;
extern crate toml;
extern crate walkdir;

use serde::{Deserialize, Serialize};

use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::path;
use std::process;

lazy_static::lazy_static! {
    /// CONFIGURATION_FILENAME denotes the file path to an optional TOML configuration file,
    /// relative to the current working directory.
    pub static ref CONFIGURATION_FILENAME: String = "todolint.toml".to_string();

    /// DEFAULT_FORMAL_TASK_PATTERN matches standardized, crossreferenced code snippets of the form `pending: <uri>`.
    pub static ref DEFAULT_FORMAL_TASK_PATTERN: String = "(?i)^.*pending: [^:]+:.+$".to_string();

    /// TASK_MARKERS collect common incomplete code snippet indicators.
    pub static ref TASK_MARKERS: Vec<String> = vec![
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
        "rube goldberg".to_string(),
        "rube-goldberg".to_string(),
        "rube goldberg".to_string(),
        "stop-gap".to_string(),
        "stop gap".to_string(),
        "stopgap".to_string(),
        "temporary solution".to_string(),
        "to-do".to_string(),
        "todo".to_string(),
        "waiting on".to_string(),
        "workaround".to_string(),
    ];

    ///
    /// DEFAULT_TASK_PATTERN matches common incomplete code snippets.
    ///
    /// Examples:
    ///
    /// * "hack"
    /// * "fixme"
    /// * "todo"
    /// * etc.
    ///
    /// Note that terms like "todo" may trigger false positives in some non-English files.
    ///
    pub static ref DEFAULT_TASK_PATTERN: String = format!(
        r"(?i)^.*\b({})\b.*$",
        TASK_MARKERS.join("|")
    );

    /// JUNK_PATHS collects common third party file paths.
    pub static ref JUNK_PATHS: Vec<String> = vec![
        // todolint
        CONFIGURATION_FILENAME.clone(),

        // VCS
        ".git".to_string(),

        // Internationalization
        "i18n".to_string(),
        "l10n".to_string(),

        // Third party code
        "node_modules".to_string(),
        "target".to_string(),
        "vendor".to_string(),
    ];

    /// DEFAULT_PATH_EXCLUSION_PATTERN matches common third party file paths.
    pub static ref DEFAULT_PATH_EXCLUSION_PATTERN: String = format!(r"^.*(/|\\)?({})$", JUNK_PATHS.join("|"));

    // TEXT_MIMETYPE_PATTERN matches text mimetypes.
    pub static ref TEXT_MIMETYPE_PATTERN: regex::Regex = regex::Regex::new("^text/.+$").unwrap();
}

#[test]
fn test_default_formal_task_pattern() {
    let pattern = regex::Regex::new(&DEFAULT_FORMAL_TASK_PATTERN).unwrap();
    assert!(pattern.is_match("PENDING: https://ticket.test/123"));
    assert!(pattern.is_match("Pending: https://ticket.test/123"));
    assert!(pattern.is_match("pending: https://ticket.test/123"));
    assert!(!pattern.is_match("pending:"));
}

#[test]
fn test_default_task_pattern() {
    let pattern = regex::Regex::new(&DEFAULT_TASK_PATTERN).unwrap();
    assert!(pattern.is_match("BAND-AID"));
    assert!(pattern.is_match("BAND AID"));
    assert!(pattern.is_match("BANDAID"));
    assert!(!pattern.is_match("BAND"));
    assert!(pattern.is_match("hack"));
    assert!(!pattern.is_match("hacker"));
    assert!(pattern.is_match("this is a hack--it should be rewritten"));
    assert!(pattern.is_match("this is a hack. it should be rewritten"));
    assert!(pattern.is_match("this is a hack and it should be rewritten"));
    assert!(pattern.is_match("TO-DO"));
    assert!(pattern.is_match("TODO"));
    assert!(pattern.is_match("TODO:"));
    assert!(pattern.is_match("TODO: walk the dog"));
    assert!(pattern.is_match("Todo"));
    assert!(pattern.is_match("Todo:"));
    assert!(pattern.is_match("Todo: walk the dog"));
    assert!(pattern.is_match("todo"));
    assert!(pattern.is_match("todo:"));
    assert!(pattern.is_match("todo: walk the dog"));
    assert!(!pattern.is_match("Let's make a big to do out of it!"));
}

#[test]
fn test_default_path_exclusion_pattern() {
    let pattern = regex::Regex::new(&DEFAULT_PATH_EXCLUSION_PATTERN).unwrap();
    assert!(pattern.is_match(&CONFIGURATION_FILENAME));
    assert!(pattern.is_match(".git"));
    assert!(pattern.is_match("./.git"));
    assert!(pattern.is_match("../.git"));
    assert!(pattern.is_match("node_modules"));
    assert!(pattern.is_match("target"));
    assert!(pattern.is_match("vendor"));
}

#[test]
fn test_text_mimetype_pattern() {
    let pattern = TEXT_MIMETYPE_PATTERN.clone();
    assert!(pattern.is_match("text/markdown"));
    assert!(pattern.is_match("text/plain"));
    assert!(pattern.is_match("text/x-c"));
    assert!(pattern.is_match("text/x-c++"));
    assert!(!pattern.is_match("application/octet-stream"));
}

/// Warning models a TODO finding.
#[derive(Debug)]
pub struct Warning {
    /// path denotes a file path.
    pub path: String,

    /// line_number denotes a line number.
    pub line_number: u64,

    /// line denotes a text snippet.
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
    TOMLParseError(String),
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
            TodolintError::TOMLParseError(e) => write!(f, "{e}"),
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
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Linter {
    /// debug enables additional logging.
    pub debug: Option<bool>,

    /// path_exclusion_pattern matches skippable file paths.
    ///
    /// Syntax is Rust [regex](https://crates.io/crates/regex).
    pub path_exclusion_pattern: Option<String>,

    /// formal_task_pattern matches standardized, documented code snippets.
    ///
    /// Syntax is Rust [regex](https://crates.io/crates/regex).
    pub formal_task_pattern: Option<String>,

    /// task_pattern matches incomplete code snippets.
    ///
    /// Syntax is Rust [regex](https://crates.io/crates/regex).
    pub task_pattern: Option<String>,
}

impl Linter {
    /// load generates a Linter.
    pub fn load() -> Result<Self, TodolintError> {
        let pth: &str = &CONFIGURATION_FILENAME;
        let toml_string = fs::read_to_string(pth)
            .map_err(|_| TodolintError::IOError(format!("unable to read file: {pth}")))?;
        let linter: Linter = toml::from_str(&toml_string)
            .map_err(|e| TodolintError::TOMLParseError(e.message().to_string()))?;
        Ok(linter)
    }

    /// find_text_paths recursively searches
    /// the given directories and/or file root paths
    /// for non-binary file paths.
    pub fn find_text_paths(&self, roots: Vec<&path::Path>) -> Result<Vec<String>, TodolintError> {
        let default_path_exclusion_pattern = DEFAULT_PATH_EXCLUSION_PATTERN.clone();
        let path_exclusion_pattern = regex::Regex::new(
            self.path_exclusion_pattern
                .as_ref()
                .unwrap_or(&default_path_exclusion_pattern),
        )
        .map_err(|e| TodolintError::RegexParseError(e.to_string()))?;
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

            if path_exclusion_pattern.is_match(pth_abs_str) {
                if let Some(true) = self.debug {
                    eprintln!("debug: excluding path: {pth_clean_str}");
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

            if !TEXT_MIMETYPE_PATTERN.is_match(mimetype) {
                if let Some(true) = self.debug {
                    eprintln!("debug: skipping mimetype: {mimetype}, path: {pth_clean_str}",);
                }

                continue;
            }

            text_paths.push(pth_clean_str.to_string())
        }

        Ok(text_paths)
    }

    pub fn check(&self, pth: String) -> Result<Vec<Warning>, TodolintError> {
        let default_formal_task_pattern = DEFAULT_FORMAL_TASK_PATTERN.clone();
        let default_task_pattern = DEFAULT_TASK_PATTERN.clone();
        let formal_task_pattern = regex::Regex::new(
            self.formal_task_pattern
                .as_ref()
                .unwrap_or(&default_formal_task_pattern),
        )
        .map_err(|e| TodolintError::RegexParseError(e.to_string()))?;
        let task_pattern =
            regex::Regex::new(self.task_pattern.as_ref().unwrap_or(&default_task_pattern))
                .map_err(|e| TodolintError::RegexParseError(e.to_string()))?;

        let file = fs::File::open(&pth)
            .map_err(|_| TodolintError::IOError(format!("unable to open file: {}", &pth)))?;
        let reader = io::BufReader::new(file);
        let mut warnings = Vec::<Warning>::new();
        let mut i = 1u64;

        for line_result in reader.lines() {
            let line = line_result.map_err(|_| {
                TodolintError::IOError(format!("unable to read line from file: {}", &pth))
            })?;

            if formal_task_pattern.is_match(&line) {
                continue;
            }

            if task_pattern.is_match(&line) {
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
