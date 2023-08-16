import struct

def to_u32(val: int|float) -> bytes:
    return round(val).to_bytes(4, byteorder='little', signed=False)


def to_u16(val: int|float) -> bytes:
    return round(val).to_bytes(2, byteorder='little', signed=False)


def to_u8(val: int|float) -> bytes:
    return round(val).to_bytes(1, byteorder='little', signed=False)

def to_i32(val: int|float) -> bytes:
    return round(val).to_bytes(4, byteorder='little', signed=True)


def to_i16(val: int|float) -> bytes:
    return round(val).to_bytes(2, byteorder='little', signed=True)


def to_i8(val: int|float) -> bytes:
    return round(val).to_bytes(1, byteorder='little', signed=True)

def to_f32(val: float) -> bytes:
    return struct.pack('<f', val)