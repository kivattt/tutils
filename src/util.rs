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

//fn has_suffix_from_list(s: &str, list: &Vec<&str>) -> bool {
fn has_suffix_from_list(s: &str, list: &[&str]) -> bool {
    for e in list.iter() {
        if s.ends_with(e) {
            return true;
        }
    }

    return false;
}

const IMAGE_TYPES: &[&str] = &[
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

const VIDEO_TYPES: &[&str] = &[
    ".mp4",
    ".webm",
    ".mkv",
    ".mov",
    ".avi",
    ".flv",
];

const ARCHIVE_TYPES: &[&str] = &[
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

const CODE_TYPES: &[&str] = &[
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

pub fn ansi_color_from_file_extension(path: &str) -> &str {

    if has_suffix_from_list(path, &IMAGE_TYPES) {
        return "\x1b[01;33m";
    }

    if has_suffix_from_list(path, &VIDEO_TYPES) {
        return "\x1b[01;35m";
    }

    if has_suffix_from_list(path, &ARCHIVE_TYPES) {
        return "\x1b[01;31m";
    }

    if has_suffix_from_list(path, &CODE_TYPES) {
        return "\x1b[01;35m";
    }

    return "";
}
