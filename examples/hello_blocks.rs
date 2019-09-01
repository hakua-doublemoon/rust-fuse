//use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
//use std::boxed::Box
const BLOCK_UNIT:  usize = 0x10;
const BLOCK_LIMIT: usize = 0x10000000; // 256MB
use std::alloc::{alloc, dealloc, realloc, Layout};

pub struct BlockBox {
    pub data: *mut u8,
    pub layout: Layout,
}

impl Drop for BlockBox {
    fn drop(&mut self)
    {
        println!("[drop]");
        unsafe {
            dealloc(self.data, self.layout);
        }
    }
}

impl BlockBox {
    pub fn read<'a>(&self, offset: i64, size: u32) -> &'a[u8]
    {
        let p = self.data;
        let sz = self.layout.size() as i64;
        let mut l = offset + (size as i64);
        if  l > sz  { l = sz };
        let slice = unsafe { std::slice::from_raw_parts(p, l as usize) };
        println!("  read to : {}", l);
        &slice[(offset as usize)..]
    }

}

pub fn create(size: u64) -> Box<BlockBox>
{
    let ptr :*mut u8;
    let lay: Layout;
    unsafe {
        lay = Layout::from_size_align(size as usize, BLOCK_UNIT).ok().unwrap();
        ptr = alloc(lay);
    }
    Box::new(BlockBox{
                data: ptr,
                layout: lay,
             })
}

pub fn data_write(bboxes: &mut Vec::<Box<BlockBox>>, block_id: u64, fs_size: i32, 
                  data: &[u8], offset: i64)
                  -> (bool, u64, i32)
{
    let mut new_id = block_id;
    let mut inc_mem_size :i32 = 0;
    if  offset < 0  {
        println!("unexpected offset : {}", offset);
        return (false, new_id, inc_mem_size);
    }
    new_id = (bboxes.len() - 1) as u64;
    inc_mem_size = 0;

    let original_size :usize;
    let mut bbx = bboxes.swap_remove(block_id as usize);
    unsafe {
        let bx = Box::leak(bbx);
        original_size = bx.layout.size();
        let mut dst_ptr = bx.data;

        let src_ptr = data.as_ptr() as *const u8;
        let src_len = data.len();
        let end_pos = src_len + (offset as usize);

        let bsize = bx.layout.size();
        if  bsize < end_pos  {
            let new_size = align_up(end_pos, BLOCK_UNIT);
            //println!("[realloc] : {}", new_size);
            if  (fs_size as usize + (new_size - bsize)) > BLOCK_LIMIT  {
                return (false, BLOCK_LIMIT as u64, 0);
            }
            inc_mem_size = (new_size - original_size) as i32;

            let lay = Layout::from_size_align(new_size, BLOCK_UNIT).ok().unwrap();
            bx.data = realloc(dst_ptr, lay, new_size);
            bx.layout = lay;

            dst_ptr = bx.data;
            if  offset > 0  {
                //std::ptr::copy_nonoverlapping(dst_ptr, new_ptr, offset);
                dst_ptr = dst_ptr.offset(offset as isize) as *mut u8;
                println!("[offset]");
            } 
            println!("[realloc]");
        }
        std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
        bbx = Box::from_raw(bx);
    }
    bboxes.push(bbx);
    (true, new_id, inc_mem_size)
}

fn align_up(n: usize, unit: usize) -> usize
{
    let n2 = n + unit-1;
    (n2/unit)*unit
}


