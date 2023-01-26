use std::{
    env,
    fs::{self, Metadata},
    path::Path,
};

use treeleaf::Tree;

/// Returns the final component of the `Path`, if there is one.
///
/// If the path is a normal file, this is the file name. If it's the path of a directory, this
/// is the directory name.
///
/// Returns [`None`] if the path terminates in `..`.
fn label<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    path.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
}

// * `fold()` - fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
// * `canonicalize` - Returns the canonical, absolute form of the path with all intermediate
//   components normalized and symbolic links resolved.
// * `metadata` - Returns the metadata for the file that this entry points at.
fn tree<P>(path: P) -> std::io::Result<Tree<String>>
where
    P: AsRef<Path>,
{
    let result = fs::read_dir(&path)?.filter_map(|e| e.ok()).fold(
        Tree::new(label(path.as_ref().canonicalize()?)),
        |mut root, entry| {
            let dir: Metadata = entry.metadata().unwrap();
            if dir.is_dir() {
                root.push(tree(entry.path()).unwrap());
            } else {
                root.push(Tree::new(label(entry.path())));
            }
            root
        },
    );
    Ok(result)
}

fn main() {
    let dir: String = env::args().nth(1).unwrap_or_else(|| String::from("."));
    match tree(dir) {
        Ok(tree) => println!("{}", tree),
        Err(e) => eprintln!("error: {}", e),
    }
}
