/////////////////////////////////////////////////////////////
// rust_byte_record::main.rs - manage byte arrays          //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 05 Sep 2020  //
/////////////////////////////////////////////////////////////
/*
   This crate contains definitions for:
   - ByteRecord type with associated free functions
   - ByteArray type with Boxed array member and methods
     for manipulating bytes and strs
   - Message type with Vec&lt;u8&gt> member and methods
     for loading and unloading byte arrays and strings 
*/
#![allow(dead_code)]
use std::convert::{TryFrom};
use std::str::Utf8Error;

/*---------------------------------------------------------
  byte_record:
  - set of public functions for manipulating byte arrays
*/
pub const BYTE_RECORD_SIZE:usize = 32;
pub type ByteRecord = [u8; BYTE_RECORD_SIZE];

pub fn clear(br: &mut ByteRecord) {
    for item in br {
        *item = 0;
    }
}
pub fn set_field(br: &mut ByteRecord, offset:usize, buff: &[u8]) {
    for i in 0..buff.len() {
        if i + offset < br.len() {
            br[i + offset] = buff[i];
        }
    }
}
pub fn usize_to_bytes(t:usize) -> [u8;8] {
    t.to_be_bytes()
}
pub fn usize_from_bytes(b: &[u8]) -> usize {
    let array = <[u8;8]>::try_from(b).unwrap();
    usize::from_be_bytes(array)
}
pub fn i32_to_bytes(t:i32) -> [u8;4] {
    t.to_be_bytes()
}
pub fn i32_from_bytes(b: &[u8]) -> i32 {
    let array = <[u8;4]>::try_from(b).unwrap();
    i32::from_be_bytes(array)
}
pub fn str_to_bytes(s:&str) -> &[u8] {
    s.as_bytes()
}
pub fn str_from_bytes(b: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(b)
}
pub fn show_record(br: &ByteRecord, fold:usize) {
    let mut foldpoint = 0;
    loop {
        print!("\n  ");
        for i in 0..fold {
            if i + foldpoint < br.len() {
                print!("{:>3} ", br[i + foldpoint]);
            }
            else {
                return;
            }
        }
        foldpoint += fold;
    }
}

/*---------------------------------------------------------
  ByteArray:
  - structure that wraps a boxed array of fixed size
  - defines methods like some of the byte_record functions
*/
#[derive(Default)]
pub struct ByteArray {
    br: Box<[u8; BYTE_RECORD_SIZE]>,
}
impl ByteArray {
    pub fn new() -> Self {
        Self {
            br: Box::new([0;BYTE_RECORD_SIZE]),
        }
    }
    pub fn len(&self) -> usize {
        BYTE_RECORD_SIZE
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn array(&self) -> &[u8; BYTE_RECORD_SIZE] {
        &self.br
    }
    pub fn clear(&mut self) {
        for i in 0..self.br.len() {
            self.br[i] = 0;
        }
    }
    pub fn set_field(&mut self, offset:usize, buff: &[u8]) {
        for (i, item) in buff.iter().enumerate() {
            if i + offset < self.br.len() {
                self.br[i + offset] = *item;
            }
        }
    }
    pub fn get_field(&self, offset:usize, size:usize) -> &[u8] {
        &self.br[offset..offset+size]
    }
    pub fn set_str(&mut self, offset:usize, s:&str) {
        let buff = str_to_bytes(s);
        self.set_field(offset, buff);
    }
    pub fn get_str(&self, offset:usize, size:usize) 
        -> Result<&str, Utf8Error> {
        str_from_bytes(&self.br[offset..offset+size])
    }
    pub fn show_record(&self, fold:usize) {
        let mut foldpoint = 0;
        loop {
            print!("\n  ");
            for i in 0..fold {
                if i + foldpoint < self.br.len() {
                    print!("{:>3} ", self.br[i + foldpoint]);
                }
                else {
                    return;
                }
            }
            foldpoint += fold;
        }
    }
}

/*---------------------------------------------------------
  Message:
  - structure that wraps Vec<u8>, treated as byte array
  - set of public functions for manipulating Message state
*/
pub const TYPE_SIZE:usize = 1;
pub const CONTENT_SIZE:usize = 8;  // max 4096 - 32 - 1 = 4063
pub const MSG_SIZE:usize = 4096;

pub enum MsgType {
    DEFAULT = 0,
    TEXT = 1,
    REPLY = 2,
    END = 4,
    QUIT = 8
}

 #[derive(Debug, Default)]
pub struct Message {
    br: Vec<u8>,
} 
impl Message {
    /*-------------------------------------------
      Primary interface
    */
    pub fn new(sz:usize) -> Self {
        Self {
            br: vec![0; sz],
        }
    }
    /*-- load existing heap array with zeros --*/
    pub fn init(&mut self) {
        let sz = self.len();
        self.br = vec![0;sz];
    }
    /*-- return message length --*/
    pub fn len(&self) -> usize {
        self.br.len()
    }
    pub fn is_empty(&self) -> bool {
        self.br.len() == 0
    }
    /*-- set message MsgType --*/
    pub fn set_type(&mut self, t:MsgType) {
        self.br[0] = t as u8;
    }
    pub fn get_type(&self) -> u8 {
        self.br[0]
    }
    /*-------------------------------------------
      Set message content from buff and set
      content size to length of buff
    */
    pub fn set_content_bytes(&mut self, buff: &[u8]) {
        self.set_content_size(buff.len());
        self.set_field(TYPE_SIZE+CONTENT_SIZE, buff);
    }
    pub fn get_content_bytes(&self) -> &[u8] {
        self.get_field(
            TYPE_SIZE + CONTENT_SIZE, 
            self.get_content_size()
        )
    }
    /*-------------------------------------------
      Set message content from str and set
      content size to length of str
    */
    pub fn set_content_str(&mut self, s:&str) {
        self.set_content_size(s.len());
        self.set_content_bytes(s.as_bytes());
    }
    pub fn get_content_str(&self) ->Result<&str, Utf8Error> {
        let sz = self.get_content_size();
        let start = TYPE_SIZE + CONTENT_SIZE;
        let end = start + sz;
        str_from_bytes(&self.br[start..end])
    }
    /*-------------------------------------------
      Display message with folded contents
    */
    pub fn show_message(&self, fold:usize) {
        let mut foldpoint = 0;
        loop {
            print!("\n  ");
            for i in 0..fold {
                if i + foldpoint < self.br.len() {
                    print!("{:>3} ", self.br[i + foldpoint]);
                }
                else {
                    return;
                }
            }
            foldpoint += fold;
        }
    }
    /*-------------------------------------------
      Secondary interface
    */
    pub fn set_field(&mut self, offset:usize, buff: &[u8]) {
        for (i, item) in buff.iter().enumerate() {
            if i + offset < self.br.len() {
                self.br[i + offset] = *item;
            }
        }
    }
    pub fn get_field(&self, offset:usize, size:usize) -> &[u8] {
        &self.br[offset..offset+size]
    }
    /*-- set message content size --*/
    pub fn set_content_size(&mut self, sz:usize) {
        self.set_field(TYPE_SIZE, &sz.to_be_bytes());
    }
    pub fn get_content_size(&self) -> usize {
        let bytes = self.get_field(TYPE_SIZE, CONTENT_SIZE);
        let mut dst = [0u8;8];
        dst.clone_from_slice(bytes); // array from byte slice
        usize::from_be_bytes(dst)    // usize from byte array
    }
    pub fn set_str(&mut self, offset:usize, s:&str) {
        let buff = str_to_bytes(s);
        self.set_field(offset, buff);
    }
    pub fn get_str(&self, offset:usize, size:usize) 
        -> Result<&str, Utf8Error> {
        str_from_bytes(&self.br[offset..offset+size])
    }
    pub fn array(&self) -> &[u8] {
        &self.br[..]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
