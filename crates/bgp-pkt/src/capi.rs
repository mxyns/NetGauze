use bytes::{BufMut};
use netgauze_parse_utils::WritablePdu;
use crate::*;

/// Make a T_new function callable from C
macro_rules! export_new {
    ($T:ty, $init:block, $($name:ident : $typ:ident),* $(,)?) => {
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [< $T _new >] ( $($name : $typ),* ) -> *mut $T {
                unsafe { export_pointer($init) }
            }
        }
    };

    ($T:ty, $fname:ident, $($name:ident : $typ:ident),* $(,)?) => {
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [< $T _new2 >] ( $($name : $typ),* ) -> *mut $T {
                unsafe { export_pointer($T::$fname($($name),*)) }
            }
        }
    };

    ($($x:expr),+ $(,)?) => {

    }
}

export_new!(BytesMut, { BytesMut(bytes::BytesMut::with_capacity(capacity)) }, capacity: usize);

pub struct BytesMut(bytes::BytesMut);

#[inline]
fn import_pointer<T>(value: *mut T) -> Box<T> {
    unsafe {
        Box::from_raw(value)
    }
}

#[inline]
unsafe fn export_pointer<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

#[no_mangle]
pub extern "C" fn make_bytesmut(capacity: usize) -> *mut BytesMut {
    unsafe {
        export_pointer(BytesMut(bytes::BytesMut::with_capacity(capacity)))
    }
}

#[no_mangle]
pub extern "C" fn free_bytesmut(buf: *mut BytesMut) {
    let _ = import_pointer(buf);
}

#[no_mangle]
pub extern "C" fn make_packet() -> *mut BgpMessage {
    unsafe {
        export_pointer(BgpMessage::KeepAlive)
    }
}

#[no_mangle]
pub unsafe extern "C" fn write_packet(msg: *const BgpMessage, buf: *mut BytesMut) -> bool {
    println!("write_packet {:?}", buf);
    let buf = &mut buf
        .as_mut()
        .expect("bad buf pointer")
        .0;

    let item = msg
        .as_ref()
        .expect("bad msg pointer");

    let mut writer = buf.writer();
    item.write(&mut writer).is_ok()
}

#[no_mangle]
pub extern "C" fn free_packet(msg: *mut BgpMessage) {
    let _ = import_pointer(msg);
}

#[no_mangle]
pub unsafe extern "C" fn print_packet(buf: *const BytesMut) {
    let buf_inner = &(buf
        .as_ref()
        .expect("bad buf pointer"))
        .0;

    println!("{:#?}", buf_inner);
    println!("{:#?}", buf_inner.len());
}

#[no_mangle]
pub extern "C" fn nonce12() {}