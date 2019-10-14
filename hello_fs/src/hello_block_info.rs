use fuse::{FileType};

use rand::Rng;

use super::hello_blocks;
use super::hello_blocks::{BlockBox, BLOCK_UNIT};

pub const INVALID_FH: u16 = 0;

pub struct BlockInfo {
    pub ino:  u64, 
    pub name: String,
    pub bbox: BlockBox,
    pub fh: u16,
    pub read_cnt: u8,
    pub is_writing: bool,
}

impl BlockInfo {
    pub fn size(&self) -> usize
    {
        self.bbox.size()
    }

    pub fn open(&mut self, flags: u32) -> u16
    {
        let fl = flags as i32;
        let is_request_write =  (fl & libc::O_WRONLY != 0) || (fl & libc::O_RDWR != 0);
        if  is_request_write && self.is_writing  {
            INVALID_FH
        } else {
            if  is_request_write  {
                self.is_writing = true;
            }
            self.read_cnt += 1;
            self.fh
        }
    }

    pub fn close(&mut self, fh: u64)
    {
        let is_open = self.is_writing | (self.read_cnt > 0);
        debug_assert!(is_open, "unexpected close");
        debug_assert!(self.fh as u64 == fh, "unexpected fh = {}", fh);
        self.read_cnt -= 1;
        self.is_writing = false;
        println!("[D] read_cnt = {}", self.read_cnt);
    }
}

pub fn create(fname: String, ino: u64) -> BlockInfo
{
    let mut fh = INVALID_FH;
    let mut rng = rand::thread_rng();
    while  fh == INVALID_FH  {
        fh = rng.gen();
    }
    BlockInfo {
        name: fname,
        ino: ino,
        bbox: hello_blocks::create_raw(BLOCK_UNIT),
        fh: fh,
        read_cnt: 0,
        is_writing: false,
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

