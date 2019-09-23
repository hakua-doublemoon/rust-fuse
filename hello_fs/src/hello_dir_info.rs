use fuse::{FileType};

use super::hello_blocks;
use super::hello_blocks::{BlockBox, BLOCK_UNIT};

pub struct BlockInfo {
    pub ino:  u64, 
    pub name: String,
    pub bbox: BlockBox,
}

impl BlockInfo {

}

pub fn create(fname: String, ino: u64) -> BlockInfo
{
    BlockInfo {
        name: fname,
        ino: ino,
        bbox: hello_blocks::create_raw(BLOCK_UNIT),
    }
}

pub fn readdir<'a>(buf: &mut Vec::<(u64, FileType, &'a str)>, dir_infos: &'a Vec<BlockInfo>)
{
    for d in dir_infos {
        buf.push( (d.ino, FileType::RegularFile, &(d.name)) );
    }
}

pub fn lookup(dir_infos: &Vec<BlockInfo>, name: String) -> u64
{
    for d in dir_infos {
        if d.name == name {
            return d.ino
        }
    }
    0
}

