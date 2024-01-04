use std::{
    path::{PathBuf, Path}, 
    io::Write,
    fs::{create_dir_all, create_dir, File}, collections::BTreeSet};
use derive_more::From;
use serde::{Serialize, Deserialize};

use crate::{objects::{directory::DirectoryObjects, Objects}, directory::{Directory, Ignores}, blob::Blob, commit::Commit};


#[derive(Debug, From)]
pub enum Error{
    #[from]
    IO(std::io::Error),
    #[from]
    Serde(serde_json::Error),
    MissingObject(Blob),
}

pub struct DotLog{
    root:PathBuf,
}

impl DotLog{
    pub fn init(root:PathBuf)->Result<Self, Error> {
        if root.exists()
        {
            println!("A repository already exists here!");
            return Ok(DotLog{root});
        }

        create_dir_all(&root)?;

        //set default branch (master)
        let mut file = File::options()
            .create(true)
            .write(true)
            .open(&root.join("branch"))?;
        file.write("master".as_bytes())?;

        // Create the branches directory
        create_dir(&root.join("branches"))?;

        //initial commit
        let mut objects = DirectoryObjects::new(root.join("objects"))?;
        let blob_dir = Directory::default();
        let blob_dir = objects.insert_json(&blob_dir)?;
        let commit = Commit{
            directory:blob_dir,
            message:String::from("first commit"),
            previous:BTreeSet::new()
        };

        let commit_id = objects.insert_json(&commit)?;
        write_json(&commit_id, &root.join("branches").join("master"))?;
        let ignores = Ignores::default();
        write_json(&ignores, &root.join("ignores"))?;

        Ok(DotLog{root})


    }
}

//writing and reading json files from /objects
pub trait JSON {
    fn insert_json<A: Serialize>(&mut self, thing: &A) -> Result<Blob, Error>;
    fn read_json<A: for<'de> Deserialize<'de>>(&mut self, object_id: Blob) -> Result<A, Error>;
}

impl JSON for DirectoryObjects{
    fn insert_json<A: Serialize>(&mut self, thing: &A) -> Result<Blob, Error> {
        Ok(self.push(&serde_json::to_vec_pretty(thing)?)?)
    }

    fn read_json<A: for<'de> Deserialize<'de>>(&mut self, object_id: Blob) -> Result<A, Error> {
        match self.get(object_id)? {
            None => Err(Error::MissingObject(object_id)),
            Some(obj) => Ok(serde_json::from_slice(&obj)?),
        }
    }
}

fn read_json<A: for<'de> Deserialize<'de>>(path: &Path) -> Result<A, Error> {
    Ok(serde_json::from_reader(
        File::options().read(true).open(path)?,
    )?)
}

fn write_json<A: Serialize>(thing: &A, path: &Path) -> Result<(), Error> {
    Ok(serde_json::to_writer_pretty(
        File::options().write(true).create(true).open(path)?,
        thing,
    )?)
}