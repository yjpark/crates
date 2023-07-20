use std::path::PathBuf;

pub type PathArg = Option<PathBuf>;

pub mod path_arg {
    use std::path::PathBuf;

    pub fn unwrap_or_else<F>(arg: &super::PathArg, fallback: F) -> PathBuf
        where F: FnOnce() -> PathBuf
    {
        if let Some(path) = arg {
            return path.clone();
        }
        fallback()
    }

    pub fn unwrap_or_cwd(arg: &super::PathArg) -> PathBuf {
        unwrap_or_else(arg, || -> PathBuf {
            std::env::current_dir().unwrap()
        })
    }

    pub fn unwrap_or_in_cwd(arg: &super::PathArg, segments: &Vec<&str>) -> PathBuf {
        unwrap_or_else(arg, || -> PathBuf {
            let mut path = std::env::current_dir().unwrap();
            for segment in segments {
                path.push(segment);
            }
            path
        })
    }

    pub fn in_unwrap_or_cwd(arg: &super::PathArg, segments: &Vec<&str>) -> PathBuf {
        let mut path = unwrap_or_cwd(arg);
        for segment in segments {
            path.push(segment);
        }
        path
    }
}
