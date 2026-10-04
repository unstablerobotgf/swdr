"""Read named registers over SWD: regs.py PERIPH.REG ... (addresses from the SVD)."""
import os, subprocess, sys, xml.etree.ElementTree as ET
os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
r = ET.parse('pac/stm32wl33-pac/STM32WL33.svd').getroot()
addr = {}
for p in r.iter('peripheral'):
    base = int(p.findtext('baseAddress'), 0)
    for reg in p.iter('register'):
        addr[f"{p.findtext('name')}.{reg.findtext('name')}"] = base + int(reg.findtext('addressOffset'), 0)
addr.update({'NVIC.ISER': 0xE000E100, 'NVIC.ISPR': 0xE000E200, 'SCB.ICSR': 0xE000ED04})
import os
P = [os.path.abspath('.tools/bin/probe-rs.exe'), 'read', 'b32', '', '1', '--chip-description-path', 'probe/STM32WL3_Series.yaml', '--chip', 'STM32WL33CCVx', '--speed', '3300']
for name in sys.argv[1:]:
    a = int(name, 0) if name.startswith('0x') else addr[name]; P[3] = hex(a)
    for _ in range(8):
        out = subprocess.run(P, capture_output=True, text=True).stdout.strip()
        if out: break
    print(f"{name:32} {a:#010x}  {out.split(':')[-1].strip() if out else 'ERR'}")
