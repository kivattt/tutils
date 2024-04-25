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

const AUDIO_TYPES: &[&str] = &[
    ".wav",
    ".flac",
    ".mp3",
    ".ogg",
    ".m4a",
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
    ".cxx",
    ".hpp",
    ".hxx",
    ".h",
    ".c",
    ".cc",
    ".py",
    ".sh",
    ".bash",
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    ".rs",
    ".lua",
    ".vim",
    ".java",
];

const DOCUMENT_TYPES: &[&str] = &[
    ".md",
    ".pdf",
    ".epub",
    ".docx",
    ".doc",
    ".odg",
    ".fodg",
    ".otg",
];

pub fn ansi_color_from_file_extension(path: &str) -> &str {

    if has_suffix_from_list(path, &IMAGE_TYPES) {
        return "\x1b[0;93m"; // Bright yellow
    }

    if has_suffix_from_list(path, &VIDEO_TYPES) {
        return "\x1b[0;95m"; // Bright purple/pink
    }

    if has_suffix_from_list(path, &ARCHIVE_TYPES) {
        return "\x1b[0;91m"; // Bright red
    }

    if has_suffix_from_list(path, &CODE_TYPES) {
        return "\x1b[0;96m"; // Bright cyan
    }

    if has_suffix_from_list(path, &AUDIO_TYPES) {
        return "\x1b[0;35m" // Dark purple
    }

    if has_suffix_from_list(path, &DOCUMENT_TYPES) {
        return "\x1b[0;37m"; // Gray, because documents are boring
    }

    return "";
}
