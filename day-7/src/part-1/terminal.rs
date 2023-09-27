use std::collections::HashMap;

pub struct Terminal {
    cwd: String,
    folders: HashMap<String, usize>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            cwd: String::from("/"),
            folders: HashMap::new(),
        }
    }

    pub fn update_cwd(&mut self, path: &String) {
        match path.as_str() {
            "/" => {
                // navigate home
                self.cwd = String::from("/");
                self.init_folder();
            }
            ".." => self.navigate_back(),
            _ => {
                self.navigate_to(path);
                self.init_folder();
            }
        }
    }

    pub fn update_size(&mut self, size: usize) {
        let path = &self.cwd;
        let folder_size = self.folders.entry(path.to_owned()).or_insert(0);
        *folder_size += size;

        if path != "/" {
            self.update_parent(size);
        }
    }

    pub fn sum_large_folders(&self) -> usize {
        self.folders
            .iter()
            .filter(|folder| folder.1 < &100000)
            .map(|folder| folder.1)
            .sum()
    }

    fn init_folder(&mut self) {
        if !self.folders.contains_key(&self.cwd) {
            self.folders.insert(self.cwd.to_owned(), 0);
        }
    }

    fn update_parent(&mut self, size: usize) {
        let mut curr_path = self.cwd.as_str();

        while let Some(path) = Self::get_parent_path(curr_path) {
            let parent_path = match path {
                "" => "/",
                _ => path,
            };

            let folder_size = self.folders.entry(parent_path.to_owned()).or_insert(0);
            *folder_size += size;

            curr_path = path;
        }
    }

    fn navigate_back(&mut self) {
        if let Some(pos) = self.cwd.rfind('/') {
            self.cwd.truncate(pos);
            if pos == 0 {
                self.cwd = String::from("/");
            }
        }
    }

    fn get_parent_path(path: &str) -> Option<&str> {
        if let Some(pos) = path.rfind('/') {
            return Some(&path[..pos]);
        }

        None
    }

    fn navigate_to(&mut self, path: &String) {
        if let Some(last_char) = self.cwd.chars().last() {
            if last_char != '/' {
                self.cwd.push('/');
            }
        }

        self.cwd.push_str(path.to_owned().as_str());
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}
