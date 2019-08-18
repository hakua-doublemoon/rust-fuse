//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{FileType, FileAttr};
//use std::time::{Duration, UNIX_EPOCH};
use std::time::{UNIX_EPOCH};
use nix::unistd;

pub fn dir_attr_get() -> FileAttr
{
    FileAttr {
        ino: 1,
        size: 0,
        blocks: 0,
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

pub fn file_attr_create(ino: u64) -> FileAttr
{
    FileAttr {
        ino: ino,
        size: 13,
        blocks: 1,
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
