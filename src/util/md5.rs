use md5::{Md5, Digest};
use std::cmp::min;

use crate::Error;

const CRYPT_HASH64: &[u8] = b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const BCRYPT_HASH64_ENC_MAP: &[u8] = b"\x40\x40\x40\x40\x40\x40\x40\x40\x40\x40\x40\x40\x40\x40\x00\x01\
				       \x36\x37\x38\x39\x3a\x3b\x3c\x3d\x3e\x3f\x40\x40\x40\x40\x40\x40\
				       \x40\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\
				       \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x40\x40\x40\x40\x40\
				       \x40\x1c\x1d\x1e\x1f\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2a\
				       \x2b\x2c\x2d\x2e\x2f\x30\x31\x32\x33\x34\x35\x40\x40\x40\x40\x40";

pub fn bcrypt_hash64_decode(enc: &str, decbuf: &mut [u8]) -> Result<(), Error> {
    let mut cbuild = 0u8;
    let mut cpos = 0;
    let mut dec_idx = 0;
    for b in enc.chars() {
	let b = b as u32 - 0x20;
	if b > 0x60 {
	    return Err(Error::EncodingError);
	}
	let dec = BCRYPT_HASH64_ENC_MAP[b as usize];
	if dec == 64 {
	    return Err(Error::EncodingError);
	}
	if cpos == 0 {
	    cbuild = dec;
	} else {
	    cbuild <<= cpos;
	    cbuild |= dec >> (6 - cpos);
	    decbuf[dec_idx] = cbuild;
	    dec_idx += 1;
	    if dec_idx == decbuf.len() {
		break;
	    }
	    cbuild = dec & (0x3F >> cpos);
	}
	cpos += 2;
	if cpos > 6 {
	    cpos = 0;
	}
    }
    Ok(())
}

pub fn md5_sha2_hash64_encode(bs: &[u8]) -> String {
    let ngroups = (bs.len() + 2) / 3;
    let mut out = String::with_capacity(ngroups * 4);
    for g in 0..ngroups {
	let mut g_idx = g * 3;
	let mut enc = 0u32;
	for _ in 0..3 {
	    let b = (if g_idx < bs.len() { bs[g_idx] } else { 0 }) as u32;
	    enc >>= 8;
	    enc |= b << 16;
	    g_idx += 1;
	}
	for _ in 0..4 {
	    out.push(char::from_u32(CRYPT_HASH64[(enc & 0x3F) as usize] as u32).unwrap());
	    enc >>= 6;
	}
    }
    match bs.len() % 3 {
	1 => { out.pop(); out.pop(); },
	2 => { out.pop(); },
	_ => (),
    }
    out
}

const MD5_MAGIC: &str = "$1$";
const MD5_TRANSPOSE: &[u8] = b"\x0c\x06\x00\x0d\x07\x01\x0e\x08\x02\x0f\x09\x03\x05\x0a\x04\x0b";

pub fn do_md5_crypt(pass: &[u8], salt: &str) -> Result<String, Error> {
    let mut dummy_buf = [0u8; 6];
    bcrypt_hash64_decode(salt, &mut dummy_buf)?;

    let mut dgst_b = Md5::new();
    dgst_b.update(pass);
    dgst_b.update(salt.as_bytes());
    dgst_b.update(pass);
    let mut hash_b = dgst_b.finalize();

    let mut dgst_a = Md5::new();
    dgst_a.update(pass);
    dgst_a.update(MD5_MAGIC.as_bytes());
    dgst_a.update(salt.as_bytes());

    let mut plen = pass.len();
    while plen > 0 {
	dgst_a.update(&hash_b[..min(plen, 16)]);
	if plen < 16 {
	    break;
	}
	plen -= 16;
    }

    plen = pass.len();
    while plen > 0 {
	match plen & 1 {
	    0 => dgst_a.update(&pass[..1]),
	    1 => dgst_a.update(&[0u8]),
	    _ => unreachable!()
	}
	plen >>= 1;
    }

    let mut hash_a = dgst_a.finalize();

    for r in 0..1000 {
	let mut dgst_a = Md5::new();
	if r % 2 == 1 {
	    dgst_a.update(pass);
	} else {
	    dgst_a.update(&hash_a);
	}
	if r % 3 > 0 {
	    dgst_a.update(salt.as_bytes());
	}
	if r % 7 > 0 {
	    dgst_a.update(pass);
	}
	if r % 2 == 0 {
	    dgst_a.update(pass);
	} else {
	    dgst_a.update(&hash_a);
	}
	hash_a = dgst_a.finalize();
    }

    for (i, &ti) in MD5_TRANSPOSE.iter().enumerate() {
	hash_b[i] = hash_a[ti as usize];
    }
    Ok(format!("{}{}${}", MD5_MAGIC, salt, md5_sha2_hash64_encode(&hash_b)))
}