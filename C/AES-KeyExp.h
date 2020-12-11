#include "Types.h"

#pragma once
// Rotate a BYTE[4] one byte to the left
void Rotate(LPBYTE lpbWord);

// table lookup for the Rcon in the key schedule
BYTE RconTLU(BYTE num);

// table lookup for the Sbox
BYTE SBoxTLU(BYTE num);

/*
	Rotate dword by one to the left
	SBox tlu for every byte in the dword
	xor the first byte dword with Rcon tlu value
*/
void KeyScheduleCore(LPDWORD lpdwValue, int iteration);

/*
	Expand the 16 Byte key to a 176 Byte Key (11 SubKeys with 16 Bytes)
*/
void KeyExpansion(LPCBYTE key, LPBYTE expandedKey);
