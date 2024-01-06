use std::{collections::{BTreeMap, BTreeSet}, path::Path, fs::File, io::Read};

use crate::{
    blob::Blob,
    objects::Objects
    };

use serde::{Deserialize, Serialize};

//directory tree, where leaves are blobs
#[derive(Clone, PartialEq, Eq, Default, Deserialize, Serialize, Debug)]
pub struct Directory{
    #[serde(flatten)]
    root:BTreeMap<String, DirectoryEntry>
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum DirectoryEntry{
    File(Blob),
    Directory(Box<Directory>)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diff {
    pub deleted: BTreeMap<String, DirectoryEntry>,
    pub added: BTreeMap<String, DirectoryEntry>,
    // pub modified: BTreeMap<String, DiffEntry>,
    pub modified: BTreeMap<String, DirectoryEntry>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
                    deleted:BTreeMap::new(),
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

    fn get_added(&self, main:&Directory, other:&Directory) -> BTreeMap<String, DirectoryEntry>{
        let mut added:BTreeMap<String, DirectoryEntry> = BTreeMap::new();
        for (entry_name, entry_obj) in &other.root{
            if let DirectoryEntry::File(_) = entry_obj {
                if !main.root.contains_key(entry_name){
                    if let Some(entry_value) = other.root.get(entry_name)
                    {
                        added.insert(entry_name.clone(), entry_value.clone());
                    }
                }
            }
            else if let DirectoryEntry::Directory(dir_o) = entry_obj{
                if !main.root.contains_key(entry_name){
                    if let Some(entry_value) = other.root.get(entry_name){
                        added.insert(entry_name.clone(), entry_value.clone());
                    }
                }
                else {
                    if let Some(entry_value) = main.root.get(entry_name)
                    {
                            if let DirectoryEntry::Directory(dir_s) = entry_value {
                                added.extend(self.get_added(&dir_s, &dir_o).into_iter().map(|(k,v)|(k.to_owned(), v)));

                            }
                    }
                }
            }
        }
        added
    }

    fn get_deleted(&self, main:&Directory, other:&Directory) -> BTreeMap<String, DirectoryEntry>{
        let mut deleted:BTreeMap<String, DirectoryEntry> = BTreeMap::new();
        for (entry_name, entry_obj) in &main.root{
            if let DirectoryEntry::File(_) = entry_obj {
                if !other.root.contains_key(entry_name){
                    if let Some(entry_value) = main.root.get(entry_name)
                    {
                        deleted.insert(entry_name.clone(), entry_value.clone());
                    }
                }
            }
            else if let DirectoryEntry::Directory(dir_s) = entry_obj{
                if !other.root.contains_key(entry_name){
                    if let Some(entry_value) = main.root.get(entry_name){
                        deleted.insert(entry_name.clone(), entry_value.clone());
                    }
                }
                else {
                    if let Some(entry_value) = other.root.get(entry_name)
                    {
                            if let DirectoryEntry::Directory(dir_o) = entry_value {
                                deleted.extend(self.get_deleted(&dir_s, &dir_o).into_iter().map(|(k,v)|(k.to_owned(), v)));

                            }
                    }
                }
            }
        }
        deleted
    }

    fn get_changes(&self, main:&Directory, other:&Directory) -> BTreeMap<String, DirectoryEntry>{
        let mut changes:BTreeMap<String, DirectoryEntry> = BTreeMap::new();
        for (entry_name, entry_obj) in &other.root{
            // println!("{:?}\n{:?}\n", entry_name, entry_obj);
            if let DirectoryEntry::File(_) = entry_obj {
                if main.root.contains_key(entry_name){
                    if let Some(entry_value) = main.root.get(entry_name)
                    {
                        if let (DirectoryEntry::File(hash_main), DirectoryEntry::File(hash_other)) = (entry_value, entry_obj){
                            if hash_main != hash_other{
                                changes.insert(entry_name.clone(), entry_value.clone());
                            }
                        }
                    }
                }
            }
            else if let DirectoryEntry::Directory(dir_o) = entry_obj{
                if main.root.contains_key(entry_name){
                    if let Some(entry_value) = main.root.get(entry_name)
                    {
                            if let DirectoryEntry::Directory(dir_s) = entry_value {
                                changes.extend(self.get_changes(&dir_s, &dir_o).into_iter().map(|(k,v)|(k.to_owned(), v)));

                            }
                    }
                }
            }
        }
        changes
    }

    pub fn diff(&self, other:&Directory) -> Diff {
        // println!("###{:?}\n{:?}\n###", self, other);
        let added = self.get_added(&self, &other);
        let deleted = self.get_deleted(&self, &other);
        let modified = self.get_changes(&self, &other);


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


