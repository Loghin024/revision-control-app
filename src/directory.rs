use std::{collections::{BTreeMap, BTreeSet}, path::Path, fs::File, io::Read};

use crate::{
    blob::Blob,
    objects::Objects
    };

use serde::{Deserialize, Serialize};

//directory tree, where leaves are blobs
#[derive(Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Directory{
    #[serde(flatten)]
    root:BTreeMap<String, DirectoryEntry>
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum DirectoryEntry{
    File(Blob),
    Directory(Box<Directory>)
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Diff {
    pub deleted: BTreeSet<String>,
    pub added: BTreeMap<String, DirectoryEntry>,
    pub modified: BTreeMap<String, DiffEntry>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffEntry {
    File(Blob),
    Directory(Box<Diff>),
}

impl DirectoryEntry{
    pub fn diff(&self, other: &DirectoryEntry) -> Option<DiffEntry>{
        match (self, other) {
            (DirectoryEntry::File(blob_s), DirectoryEntry::File(blob_o)) => {
                if blob_s != blob_o {
                    Some(DiffEntry::File(*blob_o))
                }
                else {
                    None
                }
            }
            (DirectoryEntry::Directory(_), DirectoryEntry::File(blob_o)) => {
                Some(DiffEntry::File(*blob_o))
            }
            (DirectoryEntry::File(_), DirectoryEntry::Directory(d)) => {
                Some(DiffEntry::Directory(Box::new(Diff{
                    deleted:BTreeSet::new(),
                    added:d.root.clone(),
                    modified:BTreeMap::new()
                })))
            }
            (DirectoryEntry::Directory(d_s), DirectoryEntry::Directory(d_o)) => {
                if d_s == d_o {
                    Some(DiffEntry::Directory(Box::new(d_s.diff(d_o))))
                }
                else {
                    None
                }
            }
        }
    }
}


impl Directory{
    pub fn diff(&self, other:&Directory) -> Diff {

        //collect files that are in other but not in self
        let added: BTreeMap<String, DirectoryEntry> = other
            .root
            .iter()
            .filter(|(file_name, _dir_entry)| !self.root.contains_key(*file_name))
            .map(|(fname, dir_entry)| (fname.clone(), dir_entry.clone()))
            .collect();

        //collect files that are in self but not in other
        let deleted: BTreeSet<String> = self
            .root
            .iter()
            .filter(|(file_name, _dir_entry)| !other.root.contains_key(*file_name))
            .map(|(fname, _dir_entry)| fname.clone())
            .collect();


        //collect changes between files/folders that share the same name
        let modified: BTreeMap<String, DiffEntry> = self
            .root
            .iter()
            .filter_map(|(file_name, dir_entry)| {
                other.root.get(file_name).and_then(|other_dir_entry| {
                    dir_entry
                        .diff(other_dir_entry)
                        .map(|diff| (file_name.clone(), diff))
                })
            })
            .collect();


        Diff {
            added,
            deleted,
            modified,
        }
    }
}

//list of ignored files from all levels
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Ignores {
    pub set: BTreeSet<String>,
}

impl Default for Ignores {
    fn default() -> Self {
        Ignores {
            set: vec![String::from(".log")].into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub enum Error<Store: Objects> {
    ObjectMissing(Blob),
    Store(Store::Error),
    IO(std::io::Error),
}

impl Directory{
    pub fn new<Store: Objects>(
        dir: &Path,
        ignores: &Ignores,
        store: &mut Store,
    ) -> Result<Self, Error<Store>> 
    {
        let mut root = BTreeMap::new();
        for f in std::fs::read_dir(dir).map_err(Error::IO)? {
            let dir_entry = f.map_err(Error::IO)?;
            if ignores
                .set
                .contains(&dir_entry.file_name().into_string().unwrap())
            {
                continue;
            }
            let file_type = dir_entry.file_type().map_err(Error::IO)?;
            if file_type.is_dir() {
                let directory = Directory::new(dir_entry.path().as_path(), ignores, store)?;
                root.insert(
                    dir_entry.file_name().into_string().unwrap(),
                    DirectoryEntry::Directory(Box::new(directory)),
                );
            } else if file_type.is_file() {
                let id = Blob::try_from(dir_entry.path().as_path()).map_err(Error::IO)?;
                root.insert(
                    dir_entry.file_name().into_string().unwrap(),
                    DirectoryEntry::File(id),
                );
                let mut v = Vec::new();
                let mut obj_file = File::options()
                    .read(true)
                    .open(dir_entry.path())
                    .map_err(Error::IO)?;
                obj_file.read_to_end(&mut v).map_err(Error::IO)?;
                store.push(&v).map_err(Error::Store)?;
            } else {
                eprintln!(
                    "TODO support things which aren't files or directories: {:?}",
                    dir_entry.file_name()
                );
            }
        }
        Ok(Directory { root })
    }
}


