mod key_exp;
mod aes_enc;

use key_exp::Key;
use aes_enc::{aes_encryption, TBoxes, State};

/* Output is transposed matrix. Row <=> Columns
Cleartext
00 11 22 33
44 55 66 77
88 99 AA BB
CC DD EE FF

Key
00 01 02 03
04 05 06 07
08 09 0A 0B
0C 0D 0E 0F

Ciphertext
69 C4 E0 D8
6A 7B 04 30
D8 CD B7 80
70 B4 C5 5A
*/

const TBOXES: TBoxes = TBoxes::new();

fn main() {
    let key = Key::from([
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ]);

    let mut cleartext = State::from([
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF,
    ]);

    println!("Cleartext\n{}", cleartext);
    println!("Key\n{}", key);

    let exp_key = key.key_expansion();
    aes_encryption(&mut cleartext, &exp_key, &TBOXES);

    println!("Ciphertext\n{}", cleartext);
}
