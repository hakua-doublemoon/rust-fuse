use std::env;
use std::ffi::OsStr;
use std::time::{Duration, SystemTime};
use libc::{ENOENT, ENOSYS};

use fuse::{FileType, FileAttr, Filesystem, Request};
use fuse::{ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use fuse::{ReplyWrite, ReplyOpen, ReplyEmpty, ReplyXattr};

mod hello_inode;
use hello_inode::{INVALID_INO, INVALID_BLOCK_ID};
mod hello_block_info;
use hello_block_info::{BlockInfo};
mod hello_blocks;

const TTL: Duration = Duration::from_secs(1);           // 1 second
const HELLO_DATA: [u8; 18] = [
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
                0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
                0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a];
const HELLO_DIR_INO: u64 = 1;
const FILE_HANDLER: u64 = 1;

struct HelloFS {
    // inode相当
    pub dir_inode: FileAttr,
    pub file_inode: Vec::<FileAttr>,
    // Directory Infomation 相当
    pub block_infos: Vec::<BlockInfo>,
    // block
    pub fs_size: i32,
    // fh
    pub is_file_open: bool,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, _parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("[D] -- lookup --");
        //println!("[D] < {}", name.to_str().unwrap());
        let fname = String::from(name.to_str().unwrap());
        let ino = hello_block_info::lookup(&self.block_infos, fname);
        let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
        if node_idx != INVALID_INO {
            reply.entry(&TTL, &(self.file_inode[node_idx]), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("[D] -- getattr for {} --", ino);
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
        println!("[D] -- read [ino: {}]--", ino);
        if  ino >= 2 && offset >= 0  {
            let info_idx = hello_inode::info_idx_get(&self.file_inode, ino);
            println!("[D] < {} of {}", info_idx, ino);
            reply.data(self.block_infos[info_idx].bbox.read(offset, size));
        } else {
            reply.error(ENOENT);
        }
    }

    fn mknod(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, rdev: u32, reply: ReplyEntry) {
        let fname = name.to_str().unwrap();
        println!("[D] -- mknod --");
        println!("[D]  parent: {}\n  name: {}\n  mode: {}\n  rdev: {}", 
                 parent, fname, mode, rdev);
        let ret = self.mknod_impl(1, fname, 0, 0);
        if  ret.0 != HELLO_DIR_INO  {
            reply.entry(&TTL, &(self.file_inode[ret.1]), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("[D] -- readdir [ino: {}] --", ino);
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let mut entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            //(2, FileType::RegularFile, "hello.txt"),
        ];
        hello_block_info::readdir(&mut entries, &(self.block_infos));
        //println!("{:?}", entries2);

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
        }
        reply.ok();
    }

    fn open(&mut self, _req: &Request<'_>, ino: u64, _flags: u32, reply: ReplyOpen) {
        println!("[D] -- open [ino: {}] --", ino);
        if  self.is_file_open  {
            reply.opened(0, 0);
        } else {
            self.is_file_open = true;
            reply.opened(FILE_HANDLER, 0);
        }
    }

    fn flush(&mut self, _req: &Request<'_>, ino: u64, fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        println!("[D] -- flush [ino: {}] ---", ino);
        debug_assert!(fh == FILE_HANDLER, "unknown fh {} / ino = {}", fh, ino);
        self.is_file_open = false;
        reply.ok();
    }

    fn release(&mut self, _req: &Request<'_>, ino: u64, fh: u64, _flags: u32, _lock_owner: u64, _flush: bool, reply: ReplyEmpty) {
        println!("[D] -- release [ino: {}] --", ino);
        debug_assert!(fh == FILE_HANDLER, "unknown fh {} / ino = {}", fh, ino);
        self.is_file_open = false;
        reply.ok();
    }

    fn write(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, offset: i64, data: &[u8],
                       _flags: u32, reply: ReplyWrite)
    {
        println!("[D] -- write [ino: {}] --", ino);
        println!("[D] offset = {}", offset);
        if  ino >= 2 && offset >= 0  {
            let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
            let info_idx = (self.file_inode[node_idx].blocks) as usize;
            debug_assert!(info_idx != INVALID_BLOCK_ID, "ino={} is invalid !?", ino);
            let fsize = self.write_impl(info_idx, node_idx, offset, data);
            reply.written(fsize);
        } else {
            reply.error(ENOSYS);
        }
    }

    fn getxattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, _size: u32, reply: ReplyXattr)
    {
        println!("[D] -- getxattr (not support) ---");
        reply.error(ENOSYS);
    }

    fn setattr(&mut self, _req: &Request<'_>, ino: u64, _mode: Option<u32>,
               _uid: Option<u32>, _gid: Option<u32>, _size: Option<u64>, 
               _atime: Option<SystemTime>, _mtime: Option<SystemTime>, 
               _fh: Option<u64>, _crtime: Option<SystemTime>, _chgtime: Option<SystemTime>, _bkuptime: Option<SystemTime>, 
               _flags: Option<u32>, reply: ReplyAttr)
    {
        println!("[D] -- setattr [ino: {}] --", ino);
        let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
        reply.attr(&TTL, &(self.file_inode[node_idx]));
    }
}

impl HelloFS {

    fn mknod_impl(&mut self, parent: u64, name: &str, _mode: u32, _rdev: u32) 
    -> (u64, usize)
    {
        if  parent != HELLO_DIR_INO  {
            return (HELLO_DIR_INO, 0)
        }
        let next_info = hello_inode::next_ino(&self.file_inode);
        let ino: u64 = *next_info.get("ino").unwrap() as u64;
        println!("new ino = {}", ino);

        let fname: String = String::from(name);
        self.block_infos.push(hello_block_info::create(fname, ino));
        let info_idx = self.block_infos.len() - 1;
        self.file_inode.push(hello_inode::inode_data_create(ino, 1, info_idx));

        self.update_fs_size(self.block_infos[info_idx].bbox.size() as i32);

        (ino, info_idx) 
    }

    fn update_fs_size(&mut self, size: i32)
    {
        self.fs_size += size;
    }

    fn write_impl(&mut self, info_idx: usize, node_idx: usize, offset: i64, data: &[u8]) -> u32
    {
        let ret = self.block_infos[info_idx].bbox.write(self.fs_size, data, offset);

        debug_assert!(true == ret.0, "write failed");
        self.update_fs_size(ret.1);
        self.file_inode[node_idx].size = ret.2 as u64;
        ret.2
    }
}

fn main() {
    //env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    //let options = ["-o", "ro", "-o", "fsname=hello"]
    let options = ["-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let first_ino :u64 = HELLO_DIR_INO + 1;
    let first_info_idx :usize = 0;
    let first_node_idx :usize = 0;
    let mut hello_fs = HelloFS {
        dir_inode:  hello_inode::dir_attr_get(),
        file_inode:   vec![hello_inode::inode_data_create(first_ino, 1, first_info_idx)],
        block_infos:  vec![BlockInfo {
                                name: String::from("hello.txt"),
                                ino:  first_ino,
                                bbox: hello_blocks::create_raw(1 as usize),
                            }
                          ],
        fs_size:      0,
        is_file_open: false,
    };
    hello_fs.update_fs_size(hello_fs.block_infos[first_info_idx].bbox.size() as i32);
    let written_size = hello_fs.write_impl(first_info_idx, first_node_idx, 0, &HELLO_DATA);
    hello_fs.file_inode[first_node_idx].size = written_size as u64;

    fuse::mount(hello_fs, mountpoint, &options).unwrap();
}


