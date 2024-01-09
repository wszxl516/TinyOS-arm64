#!/usr/bin/python3
import argparse
import os
import pathlib
import struct
import subprocess

MAGIC = 'symbols\0'


class Header:
    magic = MAGIC
    version: int
    entry_num: int
    size: int

    @staticmethod
    def new(version: int, num: int, size: int):
        h = Header()
        h.size = size
        h.version = version
        h.entry_num = num
        return h

    def size(self):
        self.to_bytes().__len__()

    def to_bytes(self):
        magic_len = self.magic.__len__()
        return struct.pack(f"<{magic_len}sLLQ", self.magic.encode(), self.version, self.entry_num, self.size)


class Symbol:
    address: int
    size: int
    type: int
    name: str

    @staticmethod
    def from_line(line_str: str):
        sym_self = Symbol()
        value = line_str.split()
        sym_self.address, sym_self.size, sym_self.type, sym_self.name = (
            int(value[0], 16), int(value[1], 16), value[2], value[3])
        return sym_self

    def to_bytes(self):
        str_len = self.name.__len__()
        return struct.pack(f"<QQ{str_len}sb", self.address, self.size, self.name.encode(), 0)

    def __str__(self):
        s = f"SYMBOL({self.address}, {self.size}, \'{self.type}\', \"{self.name}\")"
        return s

    def __repr__(self):
        return self.__str__()


def main(elf_path: str, dist_path: str, section_size: int, verbose: bool):
    rust_nm = "".join((os.environ["HOME"], "/.cargo/bin/rust-nm"))
    cmd = " ".join((rust_nm,
                    "--defined-only", "--print-size",
                    "--print-armap", "--size-sort",
                    "--radix=x", elf_path))
    print(cmd)
    res = subprocess.getoutput(cmd)
    all_len = 0
    entry_num = 0
    header = Header.new(1, 0, 0)
    with open(dist_path, "wb") as fp:
        header_bytes = header.to_bytes()
        fp.write(header_bytes)
        all_len += header_bytes.__len__()
        for line in res.splitlines():
            try:
                sym = Symbol.from_line(line)
            except ValueError:
                continue
            if sym.type == 't' or sym.type == 'T' or sym.type == 'A' or sym.type == 'a':
                if verbose:
                    print("0x{:018x}, 0x{:018x}, {}".format(sym.address, sym.size, sym.name))
                b = sym.to_bytes()
                all_len += len(b)
                fp.write(b)
                entry_num += 1
        fill_len = section_size - all_len
        if verbose:
            print("symbols size: 0x{:018x}".format(all_len))
            print("symbols entry num: {}".format(entry_num))
        fp.write(b'\0' * fill_len)
        fp.seek(0)
        header.size = all_len
        header.entry_num = entry_num
        fp.write(header.to_bytes())


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose', '-v', action='store_true')
    parser.add_argument("elf_path", type=pathlib.Path)
    parser.add_argument("dist_path", type=pathlib.Path)
    parser.add_argument("section_size", type=int)
    args = parser.parse_args()
    main(args.elf_path.__str__(), args.dist_path.__str__(), args.section_size, args.verbose)
