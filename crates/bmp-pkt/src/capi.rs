use std::collections::HashMap;
use std::slice;
use netgauze_parse_utils::{ReadablePduWithOneInput, Span};
use crate::BmpMessage;
use libc;
use nom::{AsBytes, Offset};

#[no_mangle]
pub extern "C"  fn netgauze_print_packet(buffer: *const libc::c_char, len: u32) -> u32 {

    let s = unsafe { slice::from_raw_parts(buffer as *const u8, len as usize) };
    let span = Span::new(s);
    if let Ok((end_span, msg)) = BmpMessage::from_wire(span, &HashMap::new()) {
        println!("span: {:#?}", span);
        println!("msg: {:#?}", msg);

        return end_span.offset(span.as_bytes()) as u32;
    }

    0
}

#[no_mangle]
pub extern "C" fn nonce1() {

}