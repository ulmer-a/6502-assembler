# 6502 High level language calling convention

## Register model
The 65C02 architecture has three 8 bit hardware registers (A, X, Y). Additionally,
the high level language compiler uses the first 32 bytes of the zeropage to hold
temporary values, these memory locations are considered special registers in the
following specification. This makes for a total of 35 8-bit registers:
* A
* X
* Y
* R0 - R31 (first 32 zeropage memory locations)

## Function call
The first three arguments that fit into 8 bits are passed in the A, X and Y registers
respectively. All remaining arguments are passed in R0..R7.

## Return value
The return value is passed in the A register. If it doesn't fit into the A register, it
is passed in R0..R7.

## Scratch registers
R0..R15 are scratch registers, whereas R16..R31 must be preserved by the function.

