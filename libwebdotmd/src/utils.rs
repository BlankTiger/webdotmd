use std::path::{Path, PathBuf};

pub(crate) fn load_files_in_dir_to_string(templates_path: &Path) -> Result<Vec<(PathBuf, String)>, std::io::Error> {
    let template_files = std::fs::read_dir(templates_path)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let templates = template_files
        .iter()
        .map(|path| (path.to_owned(), std::fs::read_to_string(path).unwrap()))
        .collect::<Vec<(PathBuf, String)>>();
    Ok(templates)
}
