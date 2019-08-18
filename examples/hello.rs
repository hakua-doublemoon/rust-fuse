use std::env;
use std::ffi::OsStr;
//use std::time::{Duration, UNIX_EPOCH};
use std::time::{Duration};
use libc::ENOENT;

use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};

mod hello_attr;
mod hello_dir_info;

const TTL: Duration = Duration::from_secs(1);           // 1 second
const HELLO_TXT_CONTENT: &str = "Hello World!\n";


struct HelloFS {
    // inode相当
    pub dir_attr: FileAttr,
    pub file_attr: Vec::<FileAttr>,
    // Directory Infomation 相当
    pub dir_infos: Vec::<hello_dir_info::DirectoryInfo>,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &(self.file_attr[0]), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        //println!("-- getattr for {} --", ino);
        match ino {
            1 => reply.attr(&TTL, &(self.dir_attr)),
            2 => reply.attr(&TTL, &(self.file_attr[0])),
            _ => reply.error(ENOENT),
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let mut entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            //(2, FileType::RegularFile, "hello.txt"),
        ];
        hello_dir_info::readdir(&mut entries, &(self.dir_infos));
        //println!("{:?}", entries2);

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
        }
        reply.ok();
    }
}

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let mut hello_fs = HelloFS {
        dir_attr:  hello_attr::dir_attr_get(),
        file_attr: vec![hello_attr::file_attr_create(2)],
        dir_infos: vec![hello_dir_info::DirectoryInfo {
                            name: "hello.txt",
                            ino: 2,
                        }
                   ],
    };

    fuse::mount(hello_fs, mountpoint, &options).unwrap();
    //fuse::spawn_mount(HelloFS, mountpoint, &options).unwrap();
}
