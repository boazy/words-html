use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::ProjectDirs;
use tap::Tap;
use url::Url;

static DICTIONARY_CSS: &[u8] = include_bytes!("../assets/dictionary.css");
static DICTIONARY_JS: &[u8] = include_bytes!("../assets/dictionary.js");

static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("me", "boazy", "latin-words-html")
        .expect("Unable to determine user's home directory")
});

pub fn resource_url(relative_path: &str) -> Url {
    let path = get_absolute_resource_path_buf(relative_path);
    return Url::from_file_path(path)
        .expect("Unable to convert resource path to URL");
}

pub fn rebuild_cache() {
    let cache_dir = PROJECT_DIRS.cache_dir();
    fs::remove_dir_all(cache_dir).unwrap_or_else(|err| {
        panic!("Unable to remove cache directory {cache_dir:?}: {err:?}")
    });
}

fn resource_bytes(name: &str) -> Option<&'static [u8]> {
    match name {
        "dictionary.css" => Some(DICTIONARY_CSS),
        "dictionary.js" => Some(DICTIONARY_JS),
        _ => None,
    }
}

fn get_absolute_resource_path_buf(relative_path: &str) -> PathBuf {
    let path = PROJECT_DIRS.cache_dir()
        .tap(|dir| {
            fs::create_dir_all(dir.join("assets"))
                .unwrap_or_else(|err| {
                    panic!("Unable to create cache directory {dir:?}: {err:?}")
                })
        })
        .join(relative_path);

    let is_file = fs::metadata(&path)
        .ok()
        .map(|metadata| metadata.is_file())
        .unwrap_or(false);

    if is_file {
        return path;
    }

    let name = relative_path.strip_prefix("assets/")
        .expect("Resource path must start with 'assets/'");

    let Some(bytes) = resource_bytes(name) else {
        panic!("Unknown resource: ${name}");
    };

    fs::write(&path, bytes)
        .unwrap_or_else(|err| {
            panic!("Unable to create resource file in cache directory {path:?}: {err:?}")
        });

    path
}