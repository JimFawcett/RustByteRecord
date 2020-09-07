/////////////////////////////////////////////////////////////
// rust_byte_record::test1.rs - demo rust_byte_record      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 05 Sep 2020  //
/////////////////////////////////////////////////////////////

use rust_byte_record::*;

fn main() {
    print!("\n  -- demo usize_to_bytes --");
    let us = 2555;
    let bytes = usize_to_bytes(us);
    print!("\n  usize: {}\n  bytes: {:?}", us, bytes);
    let us = usize_from_bytes(&bytes);
    print!("\n  usize: {}", us);
    println!();

    print!("\n  -- demo i32_to_bytes --");
    let i = -2555;
    let bytes = i32_to_bytes(i);
    print!("\n  i: {}\n  bytes: {:?}", i, bytes);
    let i = i32_from_bytes(&bytes);
    print!("\n  i: {}", i);
    println!();

    print!("\n  -- demo str_to_bytes --");
    let s = "a string";
    let bytes = str_to_bytes(s);
    print!("\n  s: {:?}\n  bytes: {:?}", s, bytes);
    let s = str_from_bytes(&bytes).unwrap();
    print!("\n  s: {:?}", s);
    println!();

    print!("\n  -- demo ByteRecord --");
    let mut br:ByteRecord = [0;BYTE_RECORD_SIZE];
    let buff:[u8;4] = [1, 2, 3, 4];
    print!("\n  byte_record:\n");
    show_record(&br,8);
    print!("\n  buff: {:?}",buff);
    let offset = 3;
    print!("\n  offset: {:?}",offset);
    set_field(&mut br, 3, &buff);
    show_record(&br,8);
    println!();

    let sz:usize = 255;
    print!("\n  sz:usize {:?}",sz);
    clear(& mut br);
    set_field(&mut br, 2, &usize_to_bytes(sz));
    show_record(&mut br, 8);

    clear(&mut br);
    let s = "a string";
    print!("\n  s: {:?}", s);
    set_field(&mut br, 2, str_to_bytes(s));
    show_record(&mut br, 8);
    let s = str_from_bytes(&br[2..10]).unwrap();
    print!("\n  s: {:?}", s);
    println!();

    print!("\n  -- demo ByteArray --");
    let mut ba = ByteArray::new();
    ba.show_record(8);
    println!();

    let buff:[u8;4] = [254, 255, 0, 1];
    let mut offset = 2;
    print!("\n  buff:   {:?}",buff);
    print!("\n  offset: {:?}",offset);
    ba.set_field(offset, &buff);
    ba.show_record(8);
    let buff = ba.get_field(offset, buff.len());
    print!("\n  buff: {:?}",buff);
    println!();

    let s = "a literal string";
    offset = 4;
    ba.clear();
    print!("\n  s: {:?}",&s);
    print!("\n  offset: {:?}",offset);
    ba.set_str(offset, &s);
    ba.show_record(8);
    let rslt = ba.get_str(offset, s.len());
    if rslt.is_ok() {
        print!("\n  s: {:?}",&rslt.unwrap());
    }
    println!();

    print!("\n  -- demo Message --");
    let mut msg = Message::new(32);
    msg.show_message(8);
    println!();

    let buff:[u8;4] = [254, 255, 0, 1];
    print!("\n  buff: {:?}",buff);
    msg.set_content_bytes(&buff);
    msg.show_message(8);
    let buff = msg.get_content_bytes();
    print!("\n  buff: {:?}",buff);
    println!();

    let s = "a literal string";
    msg.init();
    print!("\n  s: {:?}",&s);
    msg.set_content_str(&s);
    msg.show_message(8);
    let rslt = msg.get_content_str();
    if rslt.is_ok() {
        print!("\n  s: {:?}",&rslt.unwrap());
    }
    println!();
    print!("\n  msg.array():\n{:?}",msg.array());
    println!();

    print!("\n  MsgType TEXT");
    msg.init();
    msg.set_type(MsgType::TEXT);
    msg.show_message(8);
    println!("\n\n  That's all Folks!\n");
}
