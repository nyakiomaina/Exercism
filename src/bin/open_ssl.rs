extern crate openssl_sys as ffi;

const EVP_MAX_BLOCK_LENGTH: usize = 16;

extern "C" {
    pub fn EVP_aes_256_cbc() -> *const ffi::EVP_CIPHER;
    pub fn EVP_CIPHER_CTX_new() -> *mut ffi::EVP_CIPHER_CTX;
    pub fn EVP_EncryptInit_ex(ctx: *mut ffi::EVP_CIPHER_CTX, cipher: *const ffi::EVP_CIPHER, impl_: *mut ffi::ENGINE, key: *const u8, iv: *const u8) -> i32;
    pub fn EVP_EncryptUpdate(ctx: *mut ffi::EVP_CIPHER_CTX, out: *mut u8, outl: *mut i32, in_: *const u8, inl: i32) -> i32;
    pub fn EVP_EncryptFinal_ex(ctx: *mut ffi::EVP_CIPHER_CTX, outm: *mut u8, outl: *mut i32) -> i32;
    pub fn EVP_CIPHER_CTX_free(ctx: *mut ffi::EVP_CIPHER_CTX);
}

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    unsafe {
        let cipher = EVP_aes_256_cbc();
        let context = EVP_CIPHER_CTX_new();

        EVP_EncryptInit_ex(context, cipher, std::ptr::null_mut(), key.as_ptr(), iv.as_ptr());

        let mut out_buf = vec![0u8; data.len() + EVP_MAX_BLOCK_LENGTH];
        let mut out_len = 0;

        EVP_EncryptUpdate(context, out_buf.as_mut_ptr(), &mut out_len, data.as_ptr(), data.len() as i32);

        let mut final_out_len = 0;
        EVP_EncryptFinal_ex(context, out_buf.as_mut_ptr().add(out_len.try_into().unwrap()), &mut final_out_len);
        out_buf.truncate((out_len + final_out_len).try_into().unwrap());

        EVP_CIPHER_CTX_free(context);

        out_buf
    }
}
fn main() {
    let key = b"0123456789abcdef0123456789abcdef";
    let iv = b"abcdef9876543210";
    let data = b"Hello, world!";

    let encrypted_data = encrypt(data, key, iv);
    println!("Encrypted data: {:?}", encrypted_data);
}
