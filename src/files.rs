use std::path::{Path, PathBuf};

/*
 * the shell can be started from any folder, so a plain relative
 * path would break. file!() is this file's full path from when
 * amane built it, so paths are looked up from the project instead
 */
pub fn find(relative: &str) -> PathBuf {
    let this_file = Path::new(file!());

    // this file is in src/, the project is one folder up
    let source = this_file.parent().unwrap_or(Path::new("."));
    let project = source.parent().unwrap_or(Path::new("."));

    // joining a full path just gives that full path back
    project.join(relative)
}
