use block_macro::Block;
use std::{fmt::Display, ops::Index, ops::IndexMut};

const SBOX: [u8; 256] = [
    //0     1    2      3     4    5     6     7      8    9     A      B    C     D     E     F
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76, //0
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, //1
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15, //2
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75, //3
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, //4
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, //5
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, //6
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, //7
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, //8
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, //9
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, //A
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, //B
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, //C
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, //D
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf, //E
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16 ]; //F

#[derive(Block)]
pub struct State {
    bytes: [u8; 16],
}

impl Index<usize> for State {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

impl IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bytes[index]
    }
}

pub struct TBoxes {
    tbox0: [u32; 256],
    tbox1: [u32; 256],
    tbox2: [u32; 256],
    tbox3: [u32; 256],
}

impl TBoxes {
    // compute the four TBoxes
    pub const fn new() -> Self {
        let mut tbox0: [u32; 256] = [0;256];
        let mut tbox1: [u32; 256] = [0;256];
        let mut tbox2: [u32; 256] = [0;256];
        let mut tbox3: [u32; 256] = [0;256];

        let mut i = 0;
        while i < 256 {
            tbox0[i] = concat( gf_mult(SBOX[i], 2), SBOX[i], SBOX[i], gf_mult(SBOX[i], 3) );
            tbox1[i] = concat( gf_mult(SBOX[i], 3), gf_mult(SBOX[i], 2), SBOX[i], SBOX[i] );
            tbox2[i] = concat( SBOX[i], gf_mult(SBOX[i], 3), gf_mult(SBOX[i], 2), SBOX[i] );
            tbox3[i] = concat( SBOX[i], SBOX[i], gf_mult(SBOX[i], 3), gf_mult(SBOX[i], 2) );
            i += 1;
        }

        Self {
            tbox0,
            tbox1,
            tbox2,
            tbox3,
        }
    }

    // TBox lookup
    fn tbox_lup(&self, state: &mut State){
        let e0: u32 = self.tbox0[state[0] as usize] ^ self.tbox1[state[5] as usize] ^ self.tbox2[state[10] as usize] ^ self.tbox3[state[15] as usize];
        let e1: u32 = self.tbox0[state[4] as usize] ^ self.tbox1[state[9] as usize] ^ self.tbox2[state[14] as usize] ^ self.tbox3[state[3] as usize];
        let e2: u32 = self.tbox0[state[8] as usize] ^ self.tbox1[state[13] as usize] ^ self.tbox2[state[2] as usize] ^ self.tbox3[state[7] as usize];
        let e3: u32 = self.tbox0[state[12] as usize] ^ self.tbox1[state[1] as usize] ^ self.tbox2[state[6] as usize] ^ self.tbox3[state[11] as usize];
        
        state[0] = ((e0 >> 24) & 0xff) as u8;
        state[1] = ((e0 >> 16) & 0xff) as u8;
        state[2] = ((e0 >> 8) & 0xff) as u8;
        state[3] = (e0 & 0xff) as u8;

        state[4] = ((e1 >> 24) & 0xff) as u8;
        state[5] = ((e1 >> 16) & 0xff) as u8;
        state[6] = ((e1 >> 8) & 0xff) as u8;
        state[7] = (e1 & 0xff) as u8;

        state[8] = ((e2 >> 24) & 0xff) as u8;
        state[9] = ((e2 >> 16) & 0xff) as u8;
        state[10] = ((e2 >> 8) & 0xff) as u8;
        state[11] = (e2 & 0xff) as u8;

        state[12] = ((e3 >> 24) & 0xff) as u8;
        state[13] = ((e3 >> 16) & 0xff) as u8;
        state[14] = ((e3 >> 8) & 0xff) as u8;
        state[15] = (e3 & 0xff) as u8;
    }
}

// concatenate four byte to a dword
const fn concat(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    let mut dword: u32 = 0;
    dword += b0 as u32;
    dword <<= 8;
    dword += b1 as u32;
    dword <<= 8;
    dword += b2 as u32;
    dword <<= 8;
    dword += b3 as u32;

    dword
}

// multiply in GF 2^8 and reduce by AES polynom if necessary
const fn gf_mult(mut b_fac1: u8, mut b_fac2: u8) -> u8 {
    let mut p: u8 = 0;
    let mut hi_bit_set: u8;
    let mut i = 0;
    while i < 8 {
        if (b_fac2 & 1) == 1 {
            p ^= b_fac1;
        }
        hi_bit_set = b_fac1 & 0x80;
        b_fac1 <<= 1;
        if hi_bit_set == 0x80 {
            b_fac1 ^= 0x1b;		
        }
        b_fac2 >>= 1;
        i += 1;
    }
    p
}


// Whole AES Encryption with initial key add
// 1..9 main rounds with ShiftRow, SBOX and MixColumn through TBoxes and the last round without TBox
pub fn aes_encryption(state: &mut State, key: &[u8; 176], tboxes: &TBoxes){
    for i in 0..16 { // Initial Round
        state[i] ^= key[i]; 
    }
    for i in 1..10 {
        tboxes.tbox_lup(state);
        key_add(state, key, i);
    }
    for i in 0..16 { // SubBytes
        state[i] = SBOX[state[i] as usize];
    }
    shift_row(state);
    key_add(state, key, 10);
}


// shift row
fn shift_row(state: &mut State){
    let mut bt: u8;
    bt = state[1]; state[1] = state[5]; state[5] = state[9]; state[9] = state[13]; state[13] = bt;
    bt = state[2]; state[2] = state[10]; state[10] = bt; bt = state[6]; state[6] = state[14]; state[14] = bt;
    bt = state[3]; state[3] = state[15]; state[15] = state[11]; state[11] = state[7]; state[7] = bt;
}

// add the subkey to the state
fn key_add(state: &mut State, key: &[u8; 176], i_round: i32){
    for (j, i) in (i_round * 16 .. (i_round*16+16)).enumerate() {
        state[j] ^= key[i as usize];
    }
}

