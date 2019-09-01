//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{FileType};

pub struct DirectoryInfo {
    pub ino:  u64, // これをなくすなら代わりにDirectoryInfoの番号をInode側に持たせる必要がある
    pub name: String,
}

impl DirectoryInfo {

}

//pub fn readdir(buf: &mut Vec::<(u64, FileType, &str)>, dir_infos: &Vec<DirectoryInfo>)
pub fn readdir<'a>(buf: &mut Vec::<(u64, FileType, &'a str)>, dir_infos: &'a Vec<DirectoryInfo>)
{
    for d in dir_infos {
        buf.push( (d.ino, FileType::RegularFile, &(d.name)) );
    }
}

pub fn lookup(dir_infos: &Vec<DirectoryInfo>, name: String) -> u64
{
    for d in dir_infos {
        if d.name == name {
            return d.ino
        }
    }
    0
}

