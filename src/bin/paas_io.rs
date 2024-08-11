// // generics -> code abstraction
// //  ReadStats<R> and WriteStats<W> --> generic structs
//
// // Traits --> define shared behaviour in an abstract way so like INTERFACES
// //
// use std::io::{self, Read, Write, Result};
//
// pub struct ReadStats<R: Read> {
//     wrapped: R,
//     bytes_read: usize,
//     read_operations: usize,
// }
//
// pub struct WriteStats<W: Write> {
//     wrapped: W,
//     bytes_written: usize,
//     write_operations: usize,
// }
//
// impl<R: Read> ReadStats<R> {
//     pub fn new(wrapped: R) -> ReadStats<R> {
//         ReadStats {
//             wrapped,
//             bytes_read: 0,
//             read_operations: 0,
//         }
//     }
//
//     pub fn get_ref(&self) -> &R {
//         &self.wrapped
//     }
//
//     pub fn bytes_through(&self) -> usize {
//         self.bytes_read
//     }
//
//     pub fn reads(&self) -> usize {
//         self.read_operations
//     }
// }
//
// impl<R: Read> Read for ReadStats<R> {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let size = self.wrapped.read(buf)?;
//         self.bytes_read += size;
//         self.read_operations += 1;
//         Ok(size)
//     }
// }
//
// impl<W: Write> WriteStats<W> {
//     pub fn new(wrapped: W) -> WriteStats<W> {
//         WriteStats {
//             wrapped,
//             bytes_written: 0,
//             write_operations: 0,
//         }
//     }
//
//     pub fn get_ref(&self) -> &W {
//         &self.wrapped
//     }
//
//     pub fn bytes_through(&self) -> usize {
//         self.bytes_written
//     }
//
//     pub fn writes(&self) -> usize {
//         self.write_operations
//     }
// }
//
// impl<W: Write> Write for WriteStats<W> {
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         let size = self.wrapped.write(buf)?;
//         self.bytes_written += size;
//         self.write_operations += 1;
//         Ok(size)
//     }
//
//     fn flush(&mut self) -> Result<()> {
//         self.wrapped.flush()
//     }
// }
//
// fn main() {}

use std::io::{self, Read, Write};
use std::net::TcpStream; // network perations

//generic strusct to wrap any type T
struct IoStats<T> {
    inner: T,
    bytes_read: usize,
    read_ops: usize,
    bytes_written: usize,
    write_ops: usize,
}

impl <T> IoStats<T> {
    fn new(inner: T) -> Self {
        // init stats counter
        IoStats {
            inner,
            bytes_read: 0,
            read_ops: 0,
            bytes_written: 0,
            write_ops: 0,
        }
    }

    // return current stats as tuple
    fn stats(&self) -> (usize, usize, usize, usize) {
        (self.bytes_read, self.read_ops , self.bytes_written, self.write_ops)
    }
}

// read
impl<T: Read> Read for IoStats<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.bytes_read += bytes_read;
        self.read_ops += 1;
        Ok(bytes_read)
    }
}

// write
impl<T: Write> Write for IoStats<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.bytes_written += bytes_written;
        self.write_ops += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn main() {}