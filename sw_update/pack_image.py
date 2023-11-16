#!/bin/python3

import sys, io
import struct

from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection


def stm32_crc(data):
    crc=0xffffffff
    buf = bytearray()
    for b in data:
        buf.insert(0, b)
        if len(buf) == 4:
            for val in buf:
                crc ^= val << 24
                for _ in range(8):
                    crc = crc << 1 if (crc & 0x80000000) == 0 else (crc << 1) ^ 0x104c11db7
            buf = bytearray()
    return crc

def version(major, minor, patch, buildindex=0):
        return major + (minor<<8) + (patch<<16) + (buildindex<<24)

class ReadApp():
    """Read elf file and store all binaries and symbols"""
    def __init__(self, file_name):
        in_stream =  open(file_name, "rb")
        self.elf_file = ELFFile(in_stream, sys.stdout)
        self.file_name = file_name
    def get_binary(self, flash_start, flash_end):
        """Return the binary data that lies within the defined range"""
        last_adr = 0
        segment_data = []

        print(f"\nLoading app image from segments in '{self.file_name}'")
        print(f"  {'Address':8} {'Length':8}")
        for segment in self.elf_file.iter_segments():
            addr = segment['p_paddr']
            if addr >= flash_start and addr < flash_end:
                data = segment.data()
                length = len(data)
        
                print(f"  {addr:08X} {length:08X}")
                segment_data.append((addr - flash_start, data))
                last_adr = addr + length

        binary_len = last_adr - flash_start
        binary = bytearray(binary_len)
        for ofs, data in segment_data:
            binary[ofs:ofs + len(data)] = data

        # Be shure, that binary is word aligned
        while len(binary) % 4 != 0: # Go safe to get 4 byte alignment 
            binary += b'\x00'

        self.binary = binary
        return binary

    def get_symbol_address(self, symbol_name):
        """Return the address of the symbol"""
        symtab = self.elf_file.get_section_by_name('.symtab')
        symbol = symtab.get_symbol_by_name(symbol_name)
        return symbol[0].entry['st_value']

class Binary():
    """A class to create binary Larus images"""
    def __init__(self, storage_adr):
        """storage_adr: Address where the image is to be stored"""
        self.storage_adr = storage_adr

    def read_new_app(self, new_app, new_app_start, new_app_max):
        """Load the app that is to be executed later"""
        new_app = ReadApp(new_app)
        self.new_app_dest = new_app_start
        self.new_app_bin = new_app.get_binary(new_app_start, new_app_max)

    def read_copy_app(self, copy_app, copy_app_start, copy_app_max):
        """Load the copy routine that loads the future app in the right place."""
        copy_app = ReadApp(copy_app)
        self.copy_app_bin = copy_app.get_binary(copy_app_start, copy_app_max)
        self.copy_func = copy_app.get_symbol_address("main")

    def create_meta_data(self, hw_version, sw_version):
        """Create the meta data needed"""
        data = {
             'Magic Number': 0x1c80_73ab_2085_3579,
             'CRC <place holder>': 0x12345678,
             'Meta Data Version': 1,
             'Storage Address': self.storage_adr,
             'Hardware Version': hw_version,
             'Software Version': sw_version,
             'Copy Function': self.copy_func,
             'New App': self.storage_adr + 0x1000 + len(self.copy_app_bin),
             'New App Len': len(self.new_app_bin),
             'New App Dest': self.new_app_dest
        }
        print('\nCreating Meta Data:')
        for key, value in data.items():
            print(f"  {key:21}{value:08X}")


        self.meta_data = struct.pack ('<QLLLLLLLLL', *data.values())
        while len(self.meta_data) < 0x1000:
            self.meta_data += b'\x00'

    def write_file(self, file_name, hw_version, sw_version):
        """Save binary to disk"""
        self.create_meta_data(hw_version, sw_version)
        binary = bytearray(self.meta_data + self.copy_app_bin + self.new_app_bin)

        print("\nCalculate CRC 'Storage Address' -> <end>")
        crc_data = stm32_crc(binary[12:]) # Start at storag_adr -> end
        binary[8:12] = struct.pack("<L", crc_data)
        print(f"  CRC inserted         {crc_data:08X}")

        print(f"\nTotal size of binary: {round(len(binary) / 1024)}k")
        print(f"Writing binary to file '{file_name}'")
        with open(file_name, "wb") as bin_file:
            bin_file.write(binary)

print("Larus App Image Generator")
print(hex(version(2,0,1,0)))
binary = Binary(storage_adr=0x0808_0000)
binary.read_new_app(
    new_app="vario.elf", 
    new_app_start=0x0800_0000, 
    new_app_max=0x0807_ffff
)
binary.read_copy_app(
    copy_app="assets/copy_stm32f407_1m.elf", 
    copy_app_start=0x0808_1000, 
    copy_app_max=0x0808_5000
)
binary.write_file(
    "image.bin", 
    hw_version=version(1, 0, 0, 0), 
    sw_version=version(0, 1, 0, 0)
)

