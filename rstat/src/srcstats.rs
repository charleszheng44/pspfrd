use std::path::Path;
use std::fs;
use std::fmt;
use std::io::{self, BufRead};
use super::errors::StatsError;

#[derive(Default, Debug)]
pub struct SrcStats {
    pub abs_path: String,
    pub number_of_files: u32,
    pub loc: u32,
    pub comments: u32,
    pub blanks: u32,
}

impl fmt::Display for SrcStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{absolute file path: {}, \
            number of files: {}, \
            lines of code: {}, \
            number of comments: {}, \
            number of blanks{}}}", 
            self.abs_path, self.number_of_files, 
            self.loc, self.comments, self.blanks)
    }
}

pub fn get_summary_src_stats(inp_dir: &Path) -> Result<SrcStats, StatsError>{
    let mut dirs = vec!{inp_dir.to_path_buf()};
    let mut file_stats = vec!{};
    while let Some(dir) = dirs.pop() {
        for entry in dir.read_dir().expect("TODO") {
            let entry = entry.expect("TODO");
            // push the dir into the dirs
            if entry.path().is_dir() {
                dirs.push(entry.path());
                continue;
            }
            // ignore the symbolic link
            if is_symlink(&entry) {
                continue;
            }
            // get the stats of the file
            file_stats.push(get_src_stats_for_file(entry.path().as_path())?);
        }
    }
    let mut dir_stats = SrcStats {
        abs_path: inp_dir.to_str().expect("").into(),
        ..Default::default()
    };
    // merge all file stats
    for fs in file_stats {
        dir_stats.number_of_files += 1;
        dir_stats.loc += fs.loc;
        dir_stats.comments += fs.comments;
        dir_stats.blanks += fs.blanks;
    }

    Ok(dir_stats)
}

fn get_src_stats_for_file(file_path: &Path) -> Result<SrcStats, StatsError> {
    let mut file_stats = SrcStats {
        abs_path: file_path.to_str().expect("TODO").into(),
        ..Default::default()
    };
    let file = fs::File::open(file_path)?;
    for line in io::BufReader::new(file).lines() {
        let content = line?; 
        if content.trim_start().starts_with("//") {
            file_stats.comments += 1;
            continue;
        }
        if content.is_empty() {
            file_stats.blanks += 1;
            continue;
        }
        file_stats.loc += 1;
    }
    Ok(file_stats)
}

fn is_symlink(dir_entry: &fs::DirEntry) -> bool {
    dir_entry.metadata().expect("TODO").file_type().is_symlink()
}
