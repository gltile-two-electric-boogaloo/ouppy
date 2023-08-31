use io_uring::{opcode, types, IoUring};
use std::io;
use std::os::unix::io::AsRawFd;

const IO_URING_ENTRIES: u32 = 128;
const BUF_WORD: &[u8; 4] = b"arf ";
const BUF_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut ring = IoUring::new(IO_URING_ENTRIES)?;
    let mut buf = vec![0u8; BUF_SIZE*BUF_WORD.len()];
    let fd = io::stdout();

    for _ in 0 .. BUF_SIZE {
        buf.extend_from_slice(BUF_WORD);
    }
    
    let write = opcode::Write::new(types::Fd(fd.as_raw_fd()), buf.as_ptr(), buf.len() as u32)
        .build();

    loop {
        loop {
            if let Err(_) = unsafe { ring.submission().push(&write) } {
                break
            }
        }
    
        ring.submit()?;
    }
}
