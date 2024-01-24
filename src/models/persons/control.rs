use crate::secure_invite::{
    keypair::KeyPair,
    // crypt::Crypt,
};

pub struct PersonControl {
    pub nonce: u64,
    // pub control_sum: [u8; 56], // TODO: SHA-3 (Keccak) size
    // pub parent_control_sum: [u8; 56],
    // pub key_pair: KeyPair, // serde не подходит, надо другим способом
}

// TODO: вынести в отдельный модуль и использовать деленирование из статического метода в PersonControl
pub fn convert_any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
        )
    }
}

pub fn convert_slice_u8_to_ref_struct<T>(buf: &[u8]) -> &T {
    let p: *const T = buf.as_ptr() as *const T;
    unsafe { &*p }
 }