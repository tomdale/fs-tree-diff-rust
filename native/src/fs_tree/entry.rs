#[derive(Clone, Debug)]
pub struct Entry {
  pub relative_path: String,
  pub is_directory: bool
}

impl Entry {
    pub fn new(path: String) -> Entry {
        let is_directory = path.chars().last().unwrap() == '/';

        Entry {
            relative_path: path,
            is_directory: is_directory
        }
    }
}
