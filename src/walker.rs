use std::collections::VecDeque;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct TreeWalker {
    q: VecDeque<PathBuf>,
}

impl Iterator for TreeWalker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        let current = self.q.pop_front();
        let children = current
            .clone()
            .and_then(|current| read_dir(&current).ok());

        if let Some(children) = children {
            self.q.extend(children
                          .into_iter()
                          .filter_map(|c| c.ok())
                          .map(|c| c.path()));
        }

        current
    }
}

impl TreeWalker {
    pub fn new(root: &Path) -> TreeWalker {
        let mut q = VecDeque::with_capacity(1);
        q.push_back(PathBuf::from(root));

        TreeWalker { q }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs::DirEntry;
    use std::io;
    use std::path::PathBuf;
    use walker::TreeWalker;

    #[test]
    fn test_returns_the_input_directory() {
        let mut contents = TreeWalker::new(&PathBuf::from("./fixtures/w"))
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| String::from(p.to_string_lossy())));
        assert_eq!(Some(String::from("w")), contents.next());
    }

    #[test]
    fn test_returns_only_the_input_file() {
        let contents = TreeWalker::new(&PathBuf::from("./fixtures/w/e"))
            .into_iter()
            .collect::<Vec<PathBuf>>();
        assert_eq!(vec![PathBuf::from("./fixtures/w/e")], contents);
    }

    #[test]
    fn test_returns_files_directly_in_input_directory() {
        let contents = TreeWalker::new(&PathBuf::from("./fixtures/w"))
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| String::from(p.to_string_lossy())))
            .collect::<HashSet<String>>();
        assert!(contents.contains("e"));
        assert!(contents.contains("u"));
    }

    #[test]
    fn test_returns_directories_directly_in_input_directories() {
        let contents = TreeWalker::new(&PathBuf::from("./fixtures/w"))
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| String::from(p.to_string_lossy())))
            .collect::<HashSet<String>>();
        assert!(contents.contains("a"));
        assert!(contents.contains("o"));
    }

    #[test]
    fn test_returns_nested_directories() {
        let contents = TreeWalker::new(&PathBuf::from("./fixtures/w"))
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| String::from(p.to_string_lossy())))
            .collect::<HashSet<String>>();
        assert!(contents.contains("a"));
        assert!(contents.contains("l"));
        assert!(contents.contains("o"));
    }

    #[test]
    fn test_returns_nested_files() {
        let contents = TreeWalker::new(&PathBuf::from("./fixtures/w"))
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| String::from(p.to_string_lossy())))
            .collect::<HashSet<String>>();
        assert!(contents.contains("k"));
        assert!(contents.contains("l"));
        assert!(contents.contains("e"));
    }
}
