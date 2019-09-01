use std::env;
use std::ffi::OsStr;
//use std::time::{Duration, UNIX_EPOCH};
use std::time::{Duration};
use libc::{ENOENT, ENOSYS};

use fuse::{FileType, FileAttr, Filesystem, Request};
use fuse::{ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{ReplyWrite};

mod hello_inode;
use hello_inode::{INVALID_INO};
mod hello_dir_info;
use hello_dir_info::{DirectoryInfo};
mod hello_blocks;
use hello_blocks::{BlockBox};

const TTL: Duration = Duration::from_secs(1);           // 1 second
const HELLO_DATA: [u8; 18] = [
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
                0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a];
const HELLO_DIR_INO: u64 = 1;

struct HelloFS {
    // inode相当
    pub dir_inode: FileAttr,
    pub file_inode: Vec::<FileAttr>,
    // Directory Infomation 相当
    pub dir_infos: Vec::<DirectoryInfo>,
    // block
    pub bbox: Vec::<Box<BlockBox>>,
    pub fs_size: i32,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, _parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("-- lookup --");
        let fname = String::from(name.to_str().unwrap());
        let ino = hello_dir_info::lookup(&self.dir_infos, fname);
        let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
        if node_idx != INVALID_INO {
            reply.entry(&TTL, &(self.file_inode[node_idx]), 0);
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
            reply.attr(&TTL, &(self.dir_inode));
        } else {
            let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
            if node_idx != INVALID_INO {
                reply.attr(&TTL, &(self.file_inode[node_idx]));
            } else {
                reply.error(ENOENT);
            }
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, size: u32, reply: ReplyData) {
        println!("-- read --");
        if  ino == 2 && offset >= 0  {
            let bid = hello_inode::block_id_get(&self.file_inode, ino);
            reply.data(self.bbox[bid as usize].read(offset, size));
            //reply.data(&HELLO_DATA);
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
            reply.entry(&TTL, &(self.file_inode[0]), 0);
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

    fn write(&mut self, _req: &Request<'_>, _ino: u64, _fh: u64, _offset: i64, _data: &[u8],
                       _flags: u32, reply: ReplyWrite)
    {
        println!("-- write --");
        reply.error(ENOSYS);
    }
}

impl HelloFS {

    fn mknod_impl(&mut self, parent: u64, name: &str, _mode: u32, _rdev: u32) -> bool
    {
        if parent != 1 {
            return false
        }
        let next_info = hello_inode::next_ino(&self.file_inode);
        let ino: u64 = *next_info.get("ino").unwrap() as u64;

        let fname: String = String::from(name);
        self.dir_infos.push(DirectoryInfo{name: fname, ino: ino});
        let new_bid = self.bbox.len() as u64;
        //println!("{}", fname); fnameはdir_infoの中に行くらしい

        self.file_inode.push(hello_inode::file_attr_create(ino, 1, new_bid));
        true
    }

    fn update_fs_size(&mut self, size: i32)
    {
        self.fs_size += size;
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

    let first_size = 0x10 as u64;
    let mut hello_fs = HelloFS {
        dir_inode:  hello_inode::dir_attr_get(),
        file_inode: vec![hello_inode::file_attr_create(2, first_size, 0)],
        dir_infos:  vec![DirectoryInfo {
                            name: String::from("hello.txt"),
                            ino: 2,
                        }
                    ],
        bbox:       vec![hello_blocks::create(first_size)
                    ],
        fs_size:    first_size as i32,
    };
    let ret = hello_blocks::data_write(&mut hello_fs.bbox, 0, hello_fs.fs_size, 
                                       &HELLO_DATA, 0);
    hello_fs.update_fs_size(ret.2);
    let old_size = hello_fs.file_inode[0].size as i32;
    hello_fs.file_inode[0].size = (old_size + ret.2) as u64;

    fuse::mount(hello_fs, mountpoint, &options).unwrap();
}


