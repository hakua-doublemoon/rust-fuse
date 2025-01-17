//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{FileType, FileAttr};
//use std::time::{Duration, UNIX_EPOCH};
use std::time::{UNIX_EPOCH};
use nix::unistd;
use std::collections::HashMap;

pub const INVALID_INO: usize = 0xFFFFFFFF;
pub const INVALID_BLOCK_ID: usize = 0xFFFFFFFF;

pub fn dir_attr_get() -> FileAttr
{
    FileAttr {
        ino: 1,
        size: 0,
        blocks: 0xFFFFFFFF,
        atime:  UNIX_EPOCH,                // 1970-01-01 00:00:00
        mtime:  UNIX_EPOCH,
        ctime:  UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o775,
        nlink: 1,
        uid: libc::uid_t::from(unistd::Uid::current()),
        gid: libc::gid_t::from(unistd::Gid::current()),
        rdev: 0,
        flags: 0,
    }
}

pub fn inode_data_create(ino: u64, size :u64, info_idx: usize) -> FileAttr
{
    FileAttr {
        ino: ino,
        size: size,
        blocks: info_idx as u64,
        atime:  UNIX_EPOCH,                // 1970-01-01 00:00:00
        mtime:  UNIX_EPOCH,
        ctime:  UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::RegularFile,
        perm: 0o644,
        nlink: 1,
        uid: libc::uid_t::from(unistd::Uid::current()),
        gid: libc::gid_t::from(unistd::Gid::current()),
        rdev: 0,
        flags: 0,
    }
}

pub fn exists_ino(inodes: &Vec::<FileAttr>, ino: u64) -> usize
{
    let mut node_idx: usize = INVALID_INO;
    let mut i = 0;
    for inode in inodes {
        if inode.ino == ino {
            node_idx = i as usize;
            break;
        }
        i = i + 1;
    }
    node_idx
}

pub fn next_ino(inodes: &Vec::<FileAttr>) -> HashMap<&str, u32>
{
    let mut inos: Vec<u32> = vec![0,1];
    for inode in inodes {
        inos.push(inode.ino as u32);
    }
    inos.sort();
    let mut next_ino: u32 = 2;
    let mut idx: usize = 0;
    let end_idx = inos.len() - 1;
    for ino in &inos {
        next_ino = *ino + 1;
        if idx < end_idx {
            if inos[idx+1] > next_ino {
                break;
            }
            idx = idx + 1;
        } 

    }
    let mut ret = HashMap::new();
    ret.insert("ino", next_ino);
    ret.insert("index", (inos.len() + 1) as u32);
    ret
}

pub fn info_idx_get(inodes: &Vec::<FileAttr>, ino: u64) -> usize
{
    let mut iid: usize = INVALID_BLOCK_ID;
    for inode in inodes {
        if inode.ino == ino {
            iid = inode.blocks as usize;
            break;
        }
    }
    iid 
}

