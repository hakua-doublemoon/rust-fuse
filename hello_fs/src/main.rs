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
use hello_block_info::{BlockInfo, INVALID_FH};
mod hello_blocks;

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
    pub block_infos: Vec::<BlockInfo>,
    // block
    pub fs_size: i32,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, _parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("[D] -- lookup --");
        let node_idx = self.nid_get_from_name(name);
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

    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: u32, reply: ReplyOpen) {
        println!("[D] -- open [ino: {} / flag: {}] --", ino, flags);
        let info_idx = hello_inode::info_idx_get(&self.file_inode, ino);
        let fh = self.block_infos[info_idx].open(flags);
        if  fh == INVALID_FH  {
            reply.error(libc::EBUSY);
        } else {
            reply.opened(fh as u64, 0);
        }
    }

    // なくても良い
    fn flush(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        println!("[D] -- flush [ino: {}] ---", ino);
        reply.ok();
    }

    // なくても良い（でもファイルハンドラーの制御をするなら必要）
    fn release(&mut self, _req: &Request<'_>, ino: u64, fh: u64, _flags: u32, _lock_owner: u64, _flush: bool, reply: ReplyEmpty) {
        println!("[D] -- release [ino: {}] --", ino);
        let info_idx = hello_inode::info_idx_get(&self.file_inode, ino);
        self.block_infos[info_idx].close(fh);
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

    // なくても良い
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

    fn unlink(&mut self, _req: &Request<'_>, _parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("[D] -- unlink ---");
        let node_idx = self.nid_get_from_name(name);
        let info_idx = (self.file_inode[node_idx].blocks) as usize;

        let block_num = self.block_infos.len();
        let is_end_block = {info_idx == block_num-1};
        let rm_size = self.block_infos[info_idx].size();

        self.file_inode.remove(node_idx);
        self.block_infos.swap_remove(info_idx);
        // update block ID
        if  block_num > 1  && !is_end_block  {
            let ino = self.block_infos[info_idx].ino;
            let node_idx = hello_inode::exists_ino(&self.file_inode, ino);
            self.file_inode[node_idx].blocks = info_idx as u64;
            self.file_inode[node_idx].size   = self.block_infos[info_idx].size() as u64;
        }
        self.fs_size_update(-(rm_size as i32));
        reply.ok();
    }
}

impl HelloFS {

    fn nid_get_from_name(&self, name: &OsStr) -> usize
    {
        //println!("[D] < {}", name.to_str().unwrap());
        let fname = String::from(name.to_str().unwrap());
        let ino = hello_block_info::lookup(&self.block_infos, fname);
        hello_inode::exists_ino(&self.file_inode, ino)
    }

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

        self.fs_size_update(self.block_infos[info_idx].size() as i32);

        (ino, info_idx) 
    }

    fn fs_size_update(&mut self, size: i32)
    {
        self.fs_size += size;
    }

    fn write_impl(&mut self, info_idx: usize, node_idx: usize, offset: i64, data: &[u8]) -> u32
    {
        let ret = self.block_infos[info_idx].bbox.write(self.fs_size, data, offset);

        debug_assert!(true == ret.0, "write failed");
        self.fs_size_update(ret.1);
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
        block_infos:  vec![hello_block_info::create(String::from("hello.txt"), first_ino)],
        fs_size:      0,
    };
    hello_fs.fs_size_update(hello_fs.block_infos[first_info_idx].size() as i32);
    let written_size = hello_fs.write_impl(first_info_idx, first_node_idx, 0, &HELLO_DATA);
    hello_fs.file_inode[first_node_idx].size = written_size as u64;

    fuse::mount(hello_fs, mountpoint, &options).unwrap();
}


