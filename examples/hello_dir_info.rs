//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{FileType};

pub struct DirectoryInfo {
    pub ino:  u64,
    pub name: &'static str,
}

impl DirectoryInfo {

}

pub fn readdir(buf: &mut Vec::<(u64, FileType, &str)>, dir_infos: &Vec<DirectoryInfo>)
{
    //let mut ret : Vec::<(u64, FileType, &'static str)> = Vec::new();
    for d in dir_infos {
        buf.push( (d.ino, FileType::RegularFile, d.name) );
    }
}
