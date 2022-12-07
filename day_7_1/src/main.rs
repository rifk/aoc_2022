use eyre::Result;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Dir {
    name: String,
    files: HashMap<String, i32>,
    dirs: HashMap<String, Box<Dir>>,
}
impl Dir {
    /// get the size of the dir, and the size of all nested dirs
    fn dir_sizes(&self) -> (i32, Vec<i32>) {
        let files_size = self.files.values().sum::<i32>();
        let (direct_dirs, nested_dirs): (Vec<i32>, Vec<Vec<i32>>) =
            self.dirs.values().map(|d| d.dir_sizes()).unzip();
        let mut nested_dirs = nested_dirs.into_iter().flatten().collect::<Vec<i32>>();
        nested_dirs.append(&mut direct_dirs.clone());
        (
            files_size + direct_dirs.into_iter().sum::<i32>(),
            nested_dirs,
        )
    }
}

fn main() -> Result<()> {
    let filesystem = {
        let input = utils::get_input(7)?;
        let mut iter = input.lines();
        assert!(iter.next() == Some("$ cd /"));

        let root = Dir {
            name: "/".to_string(),
            files: HashMap::new(),
            dirs: HashMap::new(),
        };
        build_filesystem(&mut iter, root)
    };

    let (root, nested) = filesystem.dir_sizes();
    let mut sum = nested.into_iter().filter(|s| *s <= 100_000).sum::<i32>();
    if root < 100_000 {
        sum += root;
    }
    println!("{}", sum);

    Ok(())
}

fn build_filesystem(lines: &mut Lines, mut dir: Dir) -> Dir {
    while let Some(l) = lines.next() {
        if l == "$ cd /" {
            panic!("doest support jumping to root");
        } else if l == "$ ls" || l.starts_with("dir ") {
            continue;
        } else if l == "$ cd .." {
            return dir;
        } else if l.starts_with("$ cd ") {
            let name = l.strip_prefix("$ cd ").unwrap().to_string();
            let inner = Dir {
                name: name.clone(),
                files: HashMap::new(),
                dirs: HashMap::new(),
            };
            dir.dirs
                .insert(name, Box::new(build_filesystem(lines, inner)));
        } else {
            let mut file = l.split_whitespace();
            let size = file.next().unwrap().parse::<i32>().unwrap();
            dir.files.insert(file.next().unwrap().to_string(), size);
        }
    }
    dir
}
