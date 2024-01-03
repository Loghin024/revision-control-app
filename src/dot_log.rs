use std::{
    path::PathBuf, 
    io::Write,
    fs::{create_dir_all, create_dir, File}};
use derive_more::From;

use crate::objects::directory::DirectoryObjects;


#[derive(Debug, From)]
pub enum Error{
    #[from]
    IO(std::io::Error),
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
        let mut objects = DirectoryObjects::new(root.join("objects"));
        // let blob_dir = Dire

        Ok(DotLog{root})
    }
}