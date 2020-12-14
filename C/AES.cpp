#include <cstdio>
#include "AES-KeyExp.h"
#include "AES-Encryption.h"

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

int main(int argc, char* argv[])
{
    BYTE bKey[16]	= { 0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A,0x0B,0x0C,0x0D,0x0E,0x0F };
    BYTE bInput[16] = { 0x00,0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88,0x99,0xAA,0xBB,0xCC,0xDD,0xEE,0xFF };

    //BYTE bInput[16]	= { 0x32,0x43,0xf6,0xa8,0x88,0x5a,0x30,0x8d,0x31,0x31,0x98,0xa2,0xe0,0x37,0x07,0x34 };
    //BYTE bKey[16]	= { 0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c };

    BYTE bExpKey[176];

    //Print Cleatext
    printf("Cleartext\n");
    for(int i = 1; i<=16; i++){
        printf(("%02X%c"),bInput[i-1], i % 4 == 0 ? '\n' : ' ');
    }

    KeyExpansion(bKey, bExpKey);
    ComputeTBoxes();
    AesEncyption(bInput, bExpKey);

    // Print Key
    printf("\n\nKey\n");
    for(int i = 1; i<=16; i++){
        printf("%02X%c", bKey[i-1], i % 4 == 0 ? '\n' : ' ');
    }
    // Print Ciphertext
    printf("\n\nCiphertext\n");
    for(int i = 1; i<=16; i++){
        printf("%02X%c" , bInput[i-1], i % 4 == 0 ? '\n' : ' ');
    }

    return 0;
}


