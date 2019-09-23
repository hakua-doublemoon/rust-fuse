use std::alloc::{alloc, dealloc, realloc, Layout};

pub const BLOCK_UNIT:  usize = 0x100;
pub const BLOCK_LIMIT: usize = 0x10000000; // 256MB

pub struct BlockBox {
    pub data: *mut u8,
    pub layout: Layout,
}

impl Drop for BlockBox {
    fn drop(&mut self)
    {
        println!("[D] drop");
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
        //println!("[D]  read to : {}", l);
        &slice[(offset as usize)..]
    }

    pub fn write(&mut self, fs_size: i32, data: &[u8], offset: i64)
    -> (bool, i32, u32)
    {
        let mut inc_mem_size :i32 = 0;
        let eos_pos :usize;
        if  offset < 0  {
            println!("[E] unexpected offset : {}", offset);
            return (false, inc_mem_size, 0);
        }
        inc_mem_size = 0;

        let original_size :usize;
        unsafe {
            let bx = self;
            original_size = bx.layout.size();
            println!("[D] current size : {}", original_size);
            let mut dst_ptr = bx.data;

            let src_ptr = data.as_ptr() as *const u8;
            let src_len = data.len();
            println!("[D] src_len : {}", src_len);
            println!("[D] offset  : {}", offset);
            let end_pos = src_len + (offset as usize);
            println!("[D] end position : {}", end_pos);
            let mut cur_pos = 0;

            let bsize = original_size;
            if  bsize < end_pos  {
                let new_size = align_up(end_pos, BLOCK_UNIT);
                //println!("[realloc] : {}", new_size);
                if  (fs_size as usize + (new_size - bsize)) > BLOCK_LIMIT  {
                    return (false, 0, 0);
                }
                inc_mem_size = (new_size - original_size) as i32;

                let lay = Layout::from_size_align(new_size, BLOCK_UNIT).ok().unwrap();
                bx.data = realloc(dst_ptr, lay, new_size);
                bx.layout = lay;

                dst_ptr = bx.data;
                println!("[D] realloc");
            }
            if  offset > 0  {
                dst_ptr = dst_ptr.offset(offset as isize) as *mut u8;
                //println!("[offset]");
                cur_pos = offset as usize;
            } 
            std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
            eos_pos = cur_pos + src_len;
            let eof_pos = original_size + (inc_mem_size as usize);
            if  eos_pos < eof_pos  {
                let remain_bytes = eof_pos - eos_pos;
                dst_ptr = dst_ptr.offset(src_len as isize) as *mut u8;
                std::ptr::write_bytes(dst_ptr, 0, remain_bytes);
            }
        }
        (true, inc_mem_size, eos_pos as u32)
    }

    pub fn size(&self) -> usize
    {
        self.layout.size()
    }
}

pub fn create_raw(size: usize) -> BlockBox
{
    let ptr :*mut u8;
    let lay: Layout;
    unsafe {
        lay = Layout::from_size_align(size, BLOCK_UNIT).ok().unwrap();
        ptr = alloc(lay);
        std::ptr::write(ptr, 0);
    }
    BlockBox {
       data: ptr,
       layout: lay,
    }
}

fn align_up(n: usize, unit: usize) -> usize
{
    let n2 = n + unit-1;
    (n2/unit)*unit
}


