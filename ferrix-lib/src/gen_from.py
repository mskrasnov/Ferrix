#!/bin/env python3

s = """\
    Other,
    Unknown,
    DaughterBoard,
    ZIFSocket,
    ReplaceablePiggyBack,
    NoUpgrade,
    LIFSocket,
    Slot1,
    Slot2,
    PinSocket370,
    SlotA,
    SlotM,
    Socket423,
    SocketASocket462,
    Socket478,
    Socket754,
    Socket940,
    Socket939,
    SocketmPGA604,
    SocketLGA771,
    SocketLGA775,
    SocketS1,
    SocketAM2,
    SocketF1207,
    SocketLGA1366,
    SocketG34,
    SocketAM3,
    SocketC32,
    SocketLGA1156,
    SocketLGA1567,
    SocketPGA988A,
    SocketBGA1288,
    SocketrPGA988B,
    SocketBGA1023,
    SocketBGA1224,
    SocketLGA1155,
    SocketLGA1356,
    SocketLGA2011,
    SocketFS1,
    SocketFS2,
    SocketFM1,
    SocketFM2,
    SocketLGA2011_3,
    SocketLGA1356_3,
    SocketLGA1150,
    SocketBGA1168,
    SocketBGA1234,
    SocketBGA1364,
    SocketAM4,
    SocketLGA1151,
    SocketBGA1356,
    SocketBGA1440,
    SocketBGA1515,
    SocketLGA3647_1,
    SocketSP3,
    SocketSP3r23,
    SocketLGA2066,
    SocketBGA1392,
    SocketBGA1510,
    SocketBGA1528,
    SocketLGA4189,
    SocketLGA1200,
    SocketLGA4677,
    SocketLGA1700,
    SocketBGA1744,
    SocketBGA1781,
    SocketBGA1211,
    SocketBGA2422,
    SocketLGA1211,
    SocketLGA2422,
    SocketLGA5773,
    SocketBGA5773,
    SocketAM5,
    SocketSP5,
    SocketSP6,
    SocketBGA883,
    SocketBGA1190,
    SocketBGA4129,
    SocketLGA4710,
    SocketLGA7529,
    None"""
strings = s.split('\n')

for i in range(len(strings)):
    strings[i] = strings[i].replace(",", "").replace("  ", "")

from_s = """impl From<smbioslib::ProcessorUpgrade> for ProcessorUpgrade {
    fn from(value: smbioslib::ProcessorUpgrade) -> Self {
        match value {
"""

for s in strings:
    from_s += "            "
    from_s += f"smbioslib::ProcessorUpgrade::{s} => Self::{s},\n"

from_s += "            }\n        }\n}\n"

print(from_s)
