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

class A(Struct):
    def __init__(self, name: str=""):
        """
        struct

        Args
            name (str, optional): The name of this instance. Can be whatever you want or just an empty string. Defaults to "".
        """
        super().__init__(name)

    def __matmul__(self, _dollar___offset):
        if not (isinstance(_dollar___offset, Dollar) or isinstance(_dollar___offset, IntStruct)):
            raise Exception(f'An object of class "Dollar" must be used with the "@" operator. {{type(_dollar___offset)}} was used instead')
        if isinstance(_dollar___offset, IntStruct):
            _dollar___offset = _dollar___offset.to_dollar()
        _dollar___offset_copy = _dollar___offset.copy()
        self.b: u8 = u8('b') @ _dollar___offset

        super().init_struct(_dollar___offset_copy, _dollar___offset.copy())
        return self

a: A = A() @ Dollar(0x00, byts)
c: u8

c = a.b
