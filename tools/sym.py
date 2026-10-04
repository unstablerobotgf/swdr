"""Print address/size of ELF symbols whose (mangled) name contains each argument."""
import struct, sys
elf = sys.argv[1]
f = open(elf, 'rb').read()
shoff = struct.unpack_from('<I', f, 0x20)[0]
sz, n, si = struct.unpack_from('<HHH', f, 0x2e)
sh = [struct.unpack_from('<IIIIIIIIII', f, shoff + i * sz) for i in range(n)]
sec = {f[sh[si][4] + s[0]:].split(b'\0')[0].decode(): s for s in sh}
st = sec['.symtab']; strt = sh[st[6]]
for i in range(st[5] // 16):
    nm, val, size, _, _, _ = struct.unpack_from('<IIIBBH', f, st[4] + i * 16)
    name = f[strt[4] + nm:].split(b'\0')[0].decode()
    if any(q in name for q in sys.argv[2:]):
        print(f"{val:#010x} {size:6} {name}")
