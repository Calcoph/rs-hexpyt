from rs_hexpyt.primitives import Dollar, Struct, BitField, IntStruct, u8, u16, u24, u32, u48, u64, u96, u128, s8, s16, s24, s32, s48, s64, s96, s128, Float, double, char, char16, Bool, Padding, Array, Enum, sizeof, addressof

# Template to read from a file. follow the instructions.
# _dollar___offset has this name so it doesn't clash with others. Feel free to rename it.
if True: # Change this from "if True" to "if False", then put the file path below.
    byts = b''
else:
    file_path = "" # Put the file path here and change the above "if True" to "if False".
    with open(file_path, "rb") as f:
        byts = f.read()
_dollar___offset = Dollar(0x00, byts)
# End of template

a: Array[u8] = Array(u8, 5) @ Dollar(0x00, byts)
b: u8
b = a[3]
