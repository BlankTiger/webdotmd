use std::path::{Path, PathBuf};

pub fn load_files_in_dir_to_string(
    templates_path: &Path,
    extension: Option<&str>,
) -> Result<Vec<(PathBuf, String)>, std::io::Error> {
    let template_files = find_all_files_recursively_in_dir(templates_path, extension)?;
    let templates = template_files
        .iter()
        .map(|path| (path.to_owned(), std::fs::read_to_string(path).unwrap()))
        .collect::<Vec<(PathBuf, String)>>();
    Ok(templates)
}

fn find_all_files_recursively_in_dir(dir: &Path, extension: Option<&str>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_all_files_recursively_in_dir(&path, extension)?);
        } else {
            if let Some(extension) = extension {
                if path.extension().unwrap() != extension {
                    continue;
                }
            }
            files.push(path);
        }
    }
    Ok(files)
}
