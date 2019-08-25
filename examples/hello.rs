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
const HELLO_DIR_INO: u64 = 1;

struct HelloFS {
    // inode相当
    pub dir_attr: FileAttr,
    pub file_attr: Vec::<FileAttr>,
    // Directory Infomation 相当
    pub dir_infos: Vec::<hello_dir_info::DirectoryInfo>,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, _parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("-- lookup --");
        let fname = String::from(name.to_str().unwrap());
        let ino = hello_dir_info::lookup(&self.dir_infos, fname);
        let node_idx = hello_attr::exists_ino(&self.file_attr, ino);
        if node_idx != hello_attr::INVALID_INO {
            reply.entry(&TTL, &(self.file_attr[node_idx]), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("-- getattr for {} --", ino);
        //match ino {
        //    _ => reply.error(ENOENT),
        //}
        if ino == HELLO_DIR_INO {
            reply.attr(&TTL, &(self.dir_attr));
        } else {
            let node_idx = hello_attr::exists_ino(&self.file_attr, ino);
            if node_idx != hello_attr::INVALID_INO {
                reply.attr(&TTL, &(self.file_attr[node_idx]));
            } else {
                reply.error(ENOENT);
            }
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
        println!("-- read --");
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn mknod(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, rdev: u32, reply: ReplyEntry) {
        let fname = name.to_str().unwrap();
        //let fname = "hello2.txt";
        println!("-- mknod --");
        println!("  parent: {}\n  name: {}\n  mode: {}\n  rdev: {}", 
                 parent, fname, mode, rdev);
        if self.mknod_impl(1, fname, 0, 0) {
            reply.entry(&TTL, &(self.file_attr[0]), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("-- readdir --");
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

impl HelloFS {

    fn mknod_impl(&mut self, parent: u64, name: &str, _mode: u32, _rdev: u32) -> bool
    {
        if parent != 1 {
            return false
        }
        let next_info = hello_attr::next_ino(&self.file_attr);
        let ino: u64 = *next_info.get("ino").unwrap() as u64;

        let fname: String = String::from(name);
        self.dir_infos.push(hello_dir_info::DirectoryInfo{name: fname, ino: ino});
        //println!("{}", fname); fnameはdir_infoの中に行くらしい

        self.file_attr.push(hello_attr::file_attr_create(ino));
        true
    }
}

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    //let options = ["-o", "ro", "-o", "fsname=hello"]
    let options = ["-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let hello_fs = HelloFS {
        dir_attr:  hello_attr::dir_attr_get(),
        file_attr: vec![hello_attr::file_attr_create(2)],
        dir_infos: vec![hello_dir_info::DirectoryInfo {
                            name: String::from("hello.txt"),
                            ino: 2,
                        }
                   ],
    };
    //hello_fs.mknod_impl(1, "hello2.txt", 0, 0);

    fuse::mount(hello_fs, mountpoint, &options).unwrap();
    //fuse::spawn_mount(HelloFS, mountpoint, &options).unwrap();
}


