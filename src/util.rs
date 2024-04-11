use std::env;

pub fn working_directory() -> std::path::PathBuf {
    env::current_dir()
        .unwrap_or_else(|_| env::var("PWD").unwrap().into())
}

pub fn path_without_slash_suffix(path: &str) -> &str {
    if path.ends_with("/") {
        return &path[..path.len()-1];
    }
    return &path[..]
}

fn has_suffix_from_list(s: &str, list: &Vec<&str>) -> bool {
    for e in list.iter() {
        if s.ends_with(e) {
            return true;
        }
    }

    return false;
}

pub fn ansi_color_from_file_extension(path: &str) -> &str {
    let image_types = vec![
        ".png",
        ".jpg",
        ".jpeg",
        ".jfif",
        ".flif",
        ".tiff",
        ".gif",
        ".webp",
        ".bmp",
    ];

    let video_types = vec![
        ".mp4",
        ".webm",
        ".mkv",
        ".mov",
        ".avi",
        ".flv",
    ];

    let archive_types = vec![
        ".zip",
        ".jar",
        ".kra",
        
        // https://en.wikipedia.org/wiki/Tar_(computing)
		".tar.bz2", ".tb2", ".tbz", ".tbz2", ".tz2",
		".tar.gz", ".taz", ".tgz",
		".tar.lz",
		".tar.lzma", ".tlz",
		".tar.lzo",
		".tar.xz", ".tZ", ".taZ",
		".tar.zst", ".tzst",
    ];

    let code_types = vec![
        ".go",
        ".cpp",
        ".py",
        ".sh",
        ".bash",
        ".js",
        ".jsx",
        ".ts",
        ".tsx",
        ".c",
        ".rs",
        ".lua",
        ".vim",
        ".java",
    ];

    if has_suffix_from_list(path, &image_types) {
        return "\x1b[01;33m";
    }

    if has_suffix_from_list(path, &video_types) {
        return "\x1b[01;35m";
    }

    if has_suffix_from_list(path, &archive_types) {
        return "\x1b[01;31m";
    }

    if has_suffix_from_list(path, &code_types) {
        return "\x1b[01;35m";
    }

    return "";
}
