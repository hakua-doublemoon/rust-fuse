//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{FileType, FileAttr};
use nix::unistd;
//use std::time::{Duration, UNIX_EPOCH};
use std::time::{UNIX_EPOCH};

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
