extern crate ignore;

use std::fs::File;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};

use self::ignore::{Error, Match};
use self::ignore::gitignore::{Gitignore, GitignoreBuilder};

pub fn load(cwd: &Path) -> Result<Ignore, Error> {
    use std::io::Read;

    let mut gitignore = None;
    if let Some(info) = get_gitinfo(cwd) {
        let mut builder = GitignoreBuilder::new(info.root);

        for path in &info.ignore_paths {
            debug!("Found gitignore file: {:?}", path);

            let mut file = try!(File::open(path));
            let mut contents = String::new();
            try!(file.read_to_string(&mut contents));

            for l in contents.lines() {
                if l.is_empty() {
                    continue;
                }
                if l.starts_with("#") {
                    continue;
                }

                try!(builder.add_line(Some(path.to_owned()), l));

                // HACK: add all child entries
                let all_children = format!("{}{}**", l, MAIN_SEPARATOR);
                try!(builder.add_line(Some(path.to_owned()), &all_children));
            }
        }

        let matcher = try!(builder.build());
        gitignore = Some(matcher)
    }

    Ok(Ignore::new(gitignore))
}

pub struct Ignore {
    gitignore: Option<Gitignore>
}

impl Ignore {
    fn new(gitignore: Option<Gitignore>) -> Ignore {
        Ignore {
            gitignore: gitignore
        }
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        if let Some(ref gitignore) = self.gitignore {
            match gitignore.matched(path, false) {
                Match::None => false,
                Match::Whitelist(_) => false,
                Match::Ignore(_) => true
            }
        }
        else {
            false
        }
    }
}

struct GitInfo {
    root: PathBuf,
    ignore_paths: Vec<PathBuf>
}

fn get_gitinfo(path: &Path) -> Option<GitInfo> {
    let mut root = None;
    let mut paths = vec![];

    let mut p = path.to_owned();

    loop {
        let gitignore_path = p.join(".gitignore");
        if gitignore_path.exists() {
            paths.push(gitignore_path);
        }

        // Stop if we see a .git directory
        if let Ok(metadata) = p.join(".git").metadata() {
            if metadata.is_dir() {
                root = Some(p);
                break;
            }
        }

        if p.parent().is_none() {
            break;
        }

        p.pop();
    }

    root.map(|r| GitInfo {
        root: r,
        ignore_paths: paths
    })
}


