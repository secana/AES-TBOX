#include "Types.h"

#pragma once

// 1..9 main rounds with ShiftRow, SBox and MixColumn through TBoxes.
void AesEncyption(LPBYTE lpbState, LPCBYTE lpbKey);

// compute the four TBoxes
void ComputeTBoxes();

// concatenate four byte to a dword
DWORD ConCat(BYTE b0, BYTE b1, BYTE b2, BYTE b3);

// multiply in GF 2^8 and reduce by AES polynom if necessary
BYTE GFMult(BYTE bFac1, BYTE bFac2);

// shift row
void ShiftRow(LPBYTE lpbState);

// add the subkey to the state
void KeyAdd(LPBYTE lpbState, LPCBYTE lpbKey, int iRount);

// TBox lookup
void TBoxLUP(LPBYTE lpbState);
