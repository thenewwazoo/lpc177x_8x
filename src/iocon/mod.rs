#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration register for pin P0[0]"]
    pub p0_0: P0_0,
    #[doc = "0x04 - I/O configuration register for pin P0[1]"]
    pub p0_1: P0_1,
    #[doc = "0x08 - I/O configuration register for pin P0[2]"]
    pub p0_2: P0_2,
    #[doc = "0x0c - I/O configuration register for pin P0[3]"]
    pub p0_3: P0_3,
    #[doc = "0x10 - I/O configuration register for pin P0[4]"]
    pub p0_4: P0_4,
    #[doc = "0x14 - I/O configuration register for pin P0[5]"]
    pub p0_5: P0_5,
    #[doc = "0x18 - I/O configuration register for pin P0[6]"]
    pub p0_6: P0_6,
    #[doc = "0x1c - I/O configuration register for pin P0[7]"]
    pub p0_7: P0_7,
    #[doc = "0x20 - I/O configuration register for pin P0[8]"]
    pub p0_8: P0_8,
    #[doc = "0x24 - I/O configuration register for pin P0[9]"]
    pub p0_9: P0_9,
    #[doc = "0x28 - I/O configuration register for pin P0[10]"]
    pub p0_10: P0_10,
    #[doc = "0x2c - I/O configuration register for pin P0[11]"]
    pub p0_11: P0_11,
    #[doc = "0x30 - I/O configuration register for pin P0[12]"]
    pub p0_12: P0_12,
    #[doc = "0x34 - I/O configuration register for pin P0[13]"]
    pub p0_13: P0_13,
    #[doc = "0x38 - I/O configuration register for pin P0[14]"]
    pub p0_14: P0_14,
    #[doc = "0x3c - I/O configuration register for pin P0[15]"]
    pub p0_15: P0_15,
    #[doc = "0x40 - I/O configuration register for pin P0[16]"]
    pub p0_16: P0_16,
    #[doc = "0x44 - I/O configuration register for pin P0[17]"]
    pub p0_17: P0_17,
    #[doc = "0x48 - I/O configuration register for pin P0[18]"]
    pub p0_18: P0_18,
    #[doc = "0x4c - I/O configuration register for pin P0[19]"]
    pub p0_19: P0_19,
    #[doc = "0x50 - I/O configuration register for pin P0[20]"]
    pub p0_20: P0_20,
    #[doc = "0x54 - I/O configuration register for pin P0[21]"]
    pub p0_21: P0_21,
    _reserved0: [u8; 4usize],
    #[doc = "0x5c - I/O configuration register for pin P0[23]"]
    pub p0_23: P0_23,
    #[doc = "0x60 - I/O configuration register for pin P0[24]"]
    pub p0_24: P0_24,
    #[doc = "0x64 - I/O configuration register for pin P0[25]"]
    pub p0_25: P0_25,
    #[doc = "0x68 - I/O configuration register for pin P0[26]"]
    pub p0_26: P0_26,
    #[doc = "0x6c - I/O configuration register for pin P0[27]"]
    pub p0_27: P0_27,
    #[doc = "0x70 - I/O configuration register for pin P0[28]"]
    pub p0_28: P0_28,
    #[doc = "0x74 - I/O configuration register for pin P0[29]"]
    pub p0_29: P0_29,
    #[doc = "0x78 - I/O configuration register for pin P0[30]"]
    pub p0_30: P0_30,
    #[doc = "0x7c - I/O configuration register for pin P0[31]"]
    pub p0_31: P0_31,
    #[doc = "0x80 - I/O configuration register for pin P1[0]"]
    pub p1_0: P1_0,
    #[doc = "0x84 - I/O configuration register for pin P1[1]"]
    pub p1_1: P1_1,
    #[doc = "0x88 - I/O configuration register for pin P1[2]"]
    pub p1_2: P1_2,
    #[doc = "0x8c - I/O configuration register for pin P1[3]"]
    pub p1_3: P1_3,
    #[doc = "0x90 - I/O configuration register for pin P1[4]"]
    pub p1_4: P1_4,
    #[doc = "0x94 - I/O configuration register for pin P1[5]"]
    pub p1_5: P1_5,
    #[doc = "0x98 - I/O configuration register for pin P1[6]"]
    pub p1_6: P1_6,
    #[doc = "0x9c - I/O configuration register for pin P1[7]"]
    pub p1_7: P1_7,
    #[doc = "0xa0 - I/O configuration register for pin P1[8]"]
    pub p1_8: P1_8,
    #[doc = "0xa4 - I/O configuration register for pin P1[9]"]
    pub p1_9: P1_9,
    #[doc = "0xa8 - I/O configuration register for pin P1[10]"]
    pub p1_10: P1_10,
    #[doc = "0xac - I/O configuration register for pin P1[11]"]
    pub p1_11: P1_11,
    #[doc = "0xb0 - I/O configuration register for pin P1[12]"]
    pub p1_12: P1_12,
    #[doc = "0xb4 - I/O configuration register for pin P1[13]"]
    pub p1_13: P1_13,
    #[doc = "0xb8 - I/O configuration register for pin P1[14]"]
    pub p1_14: P1_14,
    #[doc = "0xbc - I/O configuration register for pin P1[15]"]
    pub p1_15: P1_15,
    #[doc = "0xc0 - I/O configuration register for pin P1[16]"]
    pub p1_16: P1_16,
    #[doc = "0xc4 - I/O configuration register for pin P1[17]"]
    pub p1_17: P1_17,
    #[doc = "0xc8 - I/O configuration register for pin P1[18]"]
    pub p1_18: P1_18,
    #[doc = "0xcc - I/O configuration register for pin P1[19]"]
    pub p1_19: P1_19,
    #[doc = "0xd0 - I/O configuration register for pin P1[20]"]
    pub p1_20: P1_20,
    #[doc = "0xd4 - I/O configuration register for pin P1[21]"]
    pub p1_21: P1_21,
    #[doc = "0xd8 - I/O configuration register for pin P1[22]"]
    pub p1_22: P1_22,
    #[doc = "0xdc - I/O configuration register for pin P1[23]"]
    pub p1_23: P1_23,
    #[doc = "0xe0 - I/O configuration register for pin P1[24]"]
    pub p1_24: P1_24,
    #[doc = "0xe4 - I/O configuration register for pin P1[25]"]
    pub p1_25: P1_25,
    #[doc = "0xe8 - I/O configuration register for pin P1[26]"]
    pub p1_26: P1_26,
    #[doc = "0xec - I/O configuration register for pin P1[27]"]
    pub p1_27: P1_27,
    #[doc = "0xf0 - I/O configuration register for pin P1[28]"]
    pub p1_28: P1_28,
    #[doc = "0xf4 - I/O configuration register for pin P1[29]"]
    pub p1_29: P1_29,
    #[doc = "0xf8 - I/O configuration register for pin P1[30]"]
    pub p1_30: P1_30,
    #[doc = "0xfc - I/O configuration register for pin P1[31]"]
    pub p1_31: P1_31,
    #[doc = "0x100 - I/O configuration register for pin P2[0]"]
    pub p2_0: P2_0,
    #[doc = "0x104 - I/O configuration register for pin P2[1]"]
    pub p2_1: P2_1,
    #[doc = "0x108 - I/O configuration register for pin P2[2]"]
    pub p2_2: P2_2,
    #[doc = "0x10c - I/O configuration register for pin P2[3]"]
    pub p2_3: P2_3,
    #[doc = "0x110 - I/O configuration register for pin P2[4]"]
    pub p2_4: P2_4,
    #[doc = "0x114 - I/O configuration register for pin P2[5]"]
    pub p2_5: P2_5,
    #[doc = "0x118 - I/O configuration register for pin P2[6]"]
    pub p2_6: P2_6,
    #[doc = "0x11c - I/O configuration register for pin P2[7]"]
    pub p2_7: P2_7,
    #[doc = "0x120 - I/O configuration register for pin P2[8]"]
    pub p2_8: P2_8,
    #[doc = "0x124 - I/O configuration register for pin P2[9]"]
    pub p2_9: P2_9,
    #[doc = "0x128 - I/O configuration register for pin P2[10]"]
    pub p2_10: P2_10,
    #[doc = "0x12c - I/O configuration register for pin P2[11]"]
    pub p2_11: P2_11,
    #[doc = "0x130 - I/O configuration register for pin P2[12]"]
    pub p2_12: P2_12,
    #[doc = "0x134 - I/O configuration register for pin P2[13]"]
    pub p2_13: P2_13,
    #[doc = "0x138 - I/O configuration register for pin P2[14]"]
    pub p2_14: P2_14,
    #[doc = "0x13c - I/O configuration register for pin P2[15]"]
    pub p2_15: P2_15,
    #[doc = "0x140 - I/O configuration register for pin P2[16]"]
    pub p2_16: P2_16,
    #[doc = "0x144 - I/O configuration register for pin P2[17]"]
    pub p2_17: P2_17,
    #[doc = "0x148 - I/O configuration register for pin P2[18]"]
    pub p2_18: P2_18,
    #[doc = "0x14c - I/O configuration register for pin P2[19]"]
    pub p2_19: P2_19,
    #[doc = "0x150 - I/O configuration register for pin P2[20]"]
    pub p2_20: P2_20,
    #[doc = "0x154 - I/O configuration register for pin P2[21]"]
    pub p2_21: P2_21,
    #[doc = "0x158 - I/O configuration register for pin P2[22]"]
    pub p2_22: P2_22,
    #[doc = "0x15c - I/O configuration register for pin P2[23]"]
    pub p2_23: P2_23,
    #[doc = "0x160 - I/O configuration register for pin P2[24]"]
    pub p2_24: P2_24,
    #[doc = "0x164 - I/O configuration register for pin P2[25]"]
    pub p2_25: P2_25,
    #[doc = "0x168 - I/O configuration register for pin P2[26]"]
    pub p2_26: P2_26,
    #[doc = "0x16c - I/O configuration register for pin P2[27]"]
    pub p2_27: P2_27,
    #[doc = "0x170 - I/O configuration register for pin P2[28]"]
    pub p2_28: P2_28,
    #[doc = "0x174 - I/O configuration register for pin P2[29]"]
    pub p2_29: P2_29,
    #[doc = "0x178 - I/O configuration register for pin P2[30]"]
    pub p2_30: P2_30,
    #[doc = "0x17c - I/O configuration register for pin P2[31]"]
    pub p2_31: P2_31,
    #[doc = "0x180 - I/O configuration register for pin P3[0]"]
    pub p3_0: P3_0,
    #[doc = "0x184 - I/O configuration register for pin P3[1]"]
    pub p3_1: P3_1,
    #[doc = "0x188 - I/O configuration register for pin P3[2]"]
    pub p3_2: P3_2,
    #[doc = "0x18c - I/O configuration register for pin P3[3]"]
    pub p3_3: P3_3,
    #[doc = "0x190 - I/O configuration register for pin P3[4]"]
    pub p3_4: P3_4,
    #[doc = "0x194 - I/O configuration register for pin P3[5]"]
    pub p3_5: P3_5,
    #[doc = "0x198 - I/O configuration register for pin P3[6]"]
    pub p3_6: P3_6,
    #[doc = "0x19c - I/O configuration register for pin P3[7]"]
    pub p3_7: P3_7,
    #[doc = "0x1a0 - I/O configuration register for pin P3[8]"]
    pub p3_8: P3_8,
    #[doc = "0x1a4 - I/O configuration register for pin P3[9]"]
    pub p3_9: P3_9,
    #[doc = "0x1a8 - I/O configuration register for pin P3[10]"]
    pub p3_10: P3_10,
    #[doc = "0x1ac - I/O configuration register for pin P3[11]"]
    pub p3_11: P3_11,
    #[doc = "0x1b0 - I/O configuration register for pin P3[12]"]
    pub p3_12: P3_12,
    #[doc = "0x1b4 - I/O configuration register for pin P3[13]"]
    pub p3_13: P3_13,
    #[doc = "0x1b8 - I/O configuration register for pin P3[14]"]
    pub p3_14: P3_14,
    #[doc = "0x1bc - I/O configuration register for pin P3[15]"]
    pub p3_15: P3_15,
    #[doc = "0x1c0 - I/O configuration register for pin P3[16]"]
    pub p3_16: P3_16,
    #[doc = "0x1c4 - I/O configuration register for pin P3[17]"]
    pub p3_17: P3_17,
    #[doc = "0x1c8 - I/O configuration register for pin P3[18]"]
    pub p3_18: P3_18,
    #[doc = "0x1cc - I/O configuration register for pin P3[19]"]
    pub p3_19: P3_19,
    #[doc = "0x1d0 - I/O configuration register for pin P3[20]"]
    pub p3_20: P3_20,
    #[doc = "0x1d4 - I/O configuration register for pin P3[21]"]
    pub p3_21: P3_21,
    #[doc = "0x1d8 - I/O configuration register for pin P3[22]"]
    pub p3_22: P3_22,
    #[doc = "0x1dc - I/O configuration register for pin P3[23]"]
    pub p3_23: P3_23,
    #[doc = "0x1e0 - I/O configuration register for pin P3[24]"]
    pub p3_24: P3_24,
    #[doc = "0x1e4 - I/O configuration register for pin P3[25]"]
    pub p3_25: P3_25,
    #[doc = "0x1e8 - I/O configuration register for pin P3[26]"]
    pub p3_26: P3_26,
    #[doc = "0x1ec - I/O configuration register for pin P3[27]"]
    pub p3_27: P3_27,
    #[doc = "0x1f0 - I/O configuration register for pin P3[28]"]
    pub p3_28: P3_28,
    #[doc = "0x1f4 - I/O configuration register for pin P3[29]"]
    pub p3_29: P3_29,
    #[doc = "0x1f8 - I/O configuration register for pin P3[30]"]
    pub p3_30: P3_30,
    #[doc = "0x1fc - I/O configuration register for pin P3[31]"]
    pub p3_31: P3_31,
    #[doc = "0x200 - I/O configuration register for pin P4[0]"]
    pub p4_0: P4_0,
    #[doc = "0x204 - I/O configuration register for pin P4[1]"]
    pub p4_1: P4_1,
    #[doc = "0x208 - I/O configuration register for pin P4[2]"]
    pub p4_2: P4_2,
    #[doc = "0x20c - I/O configuration register for pin P4[3]"]
    pub p4_3: P4_3,
    #[doc = "0x210 - I/O configuration register for pin P4[4]"]
    pub p4_4: P4_4,
    #[doc = "0x214 - I/O configuration register for pin P4[5]"]
    pub p4_5: P4_5,
    #[doc = "0x218 - I/O configuration register for pin P4[6]"]
    pub p4_6: P4_6,
    #[doc = "0x21c - I/O configuration register for pin P4[7]"]
    pub p4_7: P4_7,
    #[doc = "0x220 - I/O configuration register for pin P4[8]"]
    pub p4_8: P4_8,
    #[doc = "0x224 - I/O configuration register for pin P4[9]"]
    pub p4_9: P4_9,
    #[doc = "0x228 - I/O configuration register for pin P4[10]"]
    pub p4_10: P4_10,
    #[doc = "0x22c - I/O configuration register for pin P4[11]"]
    pub p4_11: P4_11,
    #[doc = "0x230 - I/O configuration register for pin P4[12]"]
    pub p4_12: P4_12,
    #[doc = "0x234 - I/O configuration register for pin P4[13]"]
    pub p4_13: P4_13,
    #[doc = "0x238 - I/O configuration register for pin P4[14]"]
    pub p4_14: P4_14,
    #[doc = "0x23c - I/O configuration register for pin P4[15]"]
    pub p4_15: P4_15,
    #[doc = "0x240 - I/O configuration register for pin P4[16]"]
    pub p4_16: P4_16,
    #[doc = "0x244 - I/O configuration register for pin P4[17]"]
    pub p4_17: P4_17,
    #[doc = "0x248 - I/O configuration register for pin P4[18]"]
    pub p4_18: P4_18,
    #[doc = "0x24c - I/O configuration register for pin P4[19]"]
    pub p4_19: P4_19,
    #[doc = "0x250 - I/O configuration register for pin P4[20]"]
    pub p4_20: P4_20,
    #[doc = "0x254 - I/O configuration register for pin P4[21]"]
    pub p4_21: P4_21,
    #[doc = "0x258 - I/O configuration register for pin P4[22]"]
    pub p4_22: P4_22,
    #[doc = "0x25c - I/O configuration register for pin P4[23]"]
    pub p4_23: P4_23,
    #[doc = "0x260 - I/O configuration register for pin P4[24]"]
    pub p4_24: P4_24,
    #[doc = "0x264 - I/O configuration register for pin P4[25]"]
    pub p4_25: P4_25,
    #[doc = "0x268 - I/O configuration register for pin P4[26]"]
    pub p4_26: P4_26,
    #[doc = "0x26c - I/O configuration register for pin P4[27]"]
    pub p4_27: P4_27,
    #[doc = "0x270 - I/O configuration register for pin P4[28]"]
    pub p4_28: P4_28,
    #[doc = "0x274 - I/O configuration register for pin P4[29]"]
    pub p4_29: P4_29,
    #[doc = "0x278 - I/O configuration register for pin P4[30]"]
    pub p4_30: P4_30,
    #[doc = "0x27c - I/O configuration register for pin P4[31]"]
    pub p4_31: P4_31,
    #[doc = "0x280 - I/O configuration register for pin P5[0]"]
    pub p5_0: P5_0,
    #[doc = "0x284 - I/O configuration register for pin P5[1]"]
    pub p5_1: P5_1,
    #[doc = "0x288 - I/O configuration register for pin P5[2]"]
    pub p5_2: P5_2,
    #[doc = "0x28c - I/O configuration register for pin P5[3]"]
    pub p5_3: P5_3,
    #[doc = "0x290 - I/O configuration register for pin P5[4]"]
    pub p5_4: P5_4,
}
#[doc = "I/O configuration register for pin P0[0]"]
pub struct P0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[0]"]
pub mod p0_0;
#[doc = "I/O configuration register for pin P0[1]"]
pub struct P0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[1]"]
pub mod p0_1;
#[doc = "I/O configuration register for pin P0[2]"]
pub struct P0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[2]"]
pub mod p0_2;
#[doc = "I/O configuration register for pin P0[3]"]
pub struct P0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[3]"]
pub mod p0_3;
#[doc = "I/O configuration register for pin P0[4]"]
pub struct P0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[4]"]
pub mod p0_4;
#[doc = "I/O configuration register for pin P0[5]"]
pub struct P0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[5]"]
pub mod p0_5;
#[doc = "I/O configuration register for pin P0[6]"]
pub struct P0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[6]"]
pub mod p0_6;
#[doc = "I/O configuration register for pin P0[7]"]
pub struct P0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[7]"]
pub mod p0_7;
#[doc = "I/O configuration register for pin P0[8]"]
pub struct P0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[8]"]
pub mod p0_8;
#[doc = "I/O configuration register for pin P0[9]"]
pub struct P0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[9]"]
pub mod p0_9;
#[doc = "I/O configuration register for pin P0[10]"]
pub struct P0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[10]"]
pub mod p0_10;
#[doc = "I/O configuration register for pin P0[11]"]
pub struct P0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[11]"]
pub mod p0_11;
#[doc = "I/O configuration register for pin P0[12]"]
pub struct P0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[12]"]
pub mod p0_12;
#[doc = "I/O configuration register for pin P0[13]"]
pub struct P0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[13]"]
pub mod p0_13;
#[doc = "I/O configuration register for pin P0[14]"]
pub struct P0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[14]"]
pub mod p0_14;
#[doc = "I/O configuration register for pin P0[15]"]
pub struct P0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[15]"]
pub mod p0_15;
#[doc = "I/O configuration register for pin P0[16]"]
pub struct P0_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[16]"]
pub mod p0_16;
#[doc = "I/O configuration register for pin P0[17]"]
pub struct P0_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[17]"]
pub mod p0_17;
#[doc = "I/O configuration register for pin P0[18]"]
pub struct P0_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[18]"]
pub mod p0_18;
#[doc = "I/O configuration register for pin P0[19]"]
pub struct P0_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[19]"]
pub mod p0_19;
#[doc = "I/O configuration register for pin P0[20]"]
pub struct P0_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[20]"]
pub mod p0_20;
#[doc = "I/O configuration register for pin P0[21]"]
pub struct P0_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[21]"]
pub mod p0_21;
#[doc = "I/O configuration register for pin P0[22]"]
pub struct P0_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[22]"]
pub mod p0_22;
#[doc = "I/O configuration register for pin P0[23]"]
pub struct P0_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[23]"]
pub mod p0_23;
#[doc = "I/O configuration register for pin P0[24]"]
pub struct P0_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[24]"]
pub mod p0_24;
#[doc = "I/O configuration register for pin P0[25]"]
pub struct P0_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[25]"]
pub mod p0_25;
#[doc = "I/O configuration register for pin P0[26]"]
pub struct P0_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[26]"]
pub mod p0_26;
#[doc = "I/O configuration register for pin P0[27]"]
pub struct P0_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[27]"]
pub mod p0_27;
#[doc = "I/O configuration register for pin P0[28]"]
pub struct P0_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[28]"]
pub mod p0_28;
#[doc = "I/O configuration register for pin P0[29]"]
pub struct P0_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[29]"]
pub mod p0_29;
#[doc = "I/O configuration register for pin P0[30]"]
pub struct P0_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[30]"]
pub mod p0_30;
#[doc = "I/O configuration register for pin P0[31]"]
pub struct P0_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P0[31]"]
pub mod p0_31;
#[doc = "I/O configuration register for pin P1[0]"]
pub struct P1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[0]"]
pub mod p1_0;
#[doc = "I/O configuration register for pin P1[1]"]
pub struct P1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[1]"]
pub mod p1_1;
#[doc = "I/O configuration register for pin P1[2]"]
pub struct P1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[2]"]
pub mod p1_2;
#[doc = "I/O configuration register for pin P1[3]"]
pub struct P1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[3]"]
pub mod p1_3;
#[doc = "I/O configuration register for pin P1[4]"]
pub struct P1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[4]"]
pub mod p1_4;
#[doc = "I/O configuration register for pin P1[5]"]
pub struct P1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[5]"]
pub mod p1_5;
#[doc = "I/O configuration register for pin P1[6]"]
pub struct P1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[6]"]
pub mod p1_6;
#[doc = "I/O configuration register for pin P1[7]"]
pub struct P1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[7]"]
pub mod p1_7;
#[doc = "I/O configuration register for pin P1[8]"]
pub struct P1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[8]"]
pub mod p1_8;
#[doc = "I/O configuration register for pin P1[9]"]
pub struct P1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[9]"]
pub mod p1_9;
#[doc = "I/O configuration register for pin P1[10]"]
pub struct P1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[10]"]
pub mod p1_10;
#[doc = "I/O configuration register for pin P1[11]"]
pub struct P1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[11]"]
pub mod p1_11;
#[doc = "I/O configuration register for pin P1[12]"]
pub struct P1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[12]"]
pub mod p1_12;
#[doc = "I/O configuration register for pin P1[13]"]
pub struct P1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[13]"]
pub mod p1_13;
#[doc = "I/O configuration register for pin P1[14]"]
pub struct P1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[14]"]
pub mod p1_14;
#[doc = "I/O configuration register for pin P1[15]"]
pub struct P1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[15]"]
pub mod p1_15;
#[doc = "I/O configuration register for pin P1[16]"]
pub struct P1_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[16]"]
pub mod p1_16;
#[doc = "I/O configuration register for pin P1[17]"]
pub struct P1_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[17]"]
pub mod p1_17;
#[doc = "I/O configuration register for pin P1[18]"]
pub struct P1_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[18]"]
pub mod p1_18;
#[doc = "I/O configuration register for pin P1[19]"]
pub struct P1_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[19]"]
pub mod p1_19;
#[doc = "I/O configuration register for pin P1[20]"]
pub struct P1_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[20]"]
pub mod p1_20;
#[doc = "I/O configuration register for pin P1[21]"]
pub struct P1_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[21]"]
pub mod p1_21;
#[doc = "I/O configuration register for pin P1[22]"]
pub struct P1_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[22]"]
pub mod p1_22;
#[doc = "I/O configuration register for pin P1[23]"]
pub struct P1_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[23]"]
pub mod p1_23;
#[doc = "I/O configuration register for pin P1[24]"]
pub struct P1_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[24]"]
pub mod p1_24;
#[doc = "I/O configuration register for pin P1[25]"]
pub struct P1_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[25]"]
pub mod p1_25;
#[doc = "I/O configuration register for pin P1[26]"]
pub struct P1_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[26]"]
pub mod p1_26;
#[doc = "I/O configuration register for pin P1[27]"]
pub struct P1_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[27]"]
pub mod p1_27;
#[doc = "I/O configuration register for pin P1[28]"]
pub struct P1_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[28]"]
pub mod p1_28;
#[doc = "I/O configuration register for pin P1[29]"]
pub struct P1_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[29]"]
pub mod p1_29;
#[doc = "I/O configuration register for pin P1[30]"]
pub struct P1_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[30]"]
pub mod p1_30;
#[doc = "I/O configuration register for pin P1[31]"]
pub struct P1_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P1[31]"]
pub mod p1_31;
#[doc = "I/O configuration register for pin P2[0]"]
pub struct P2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[0]"]
pub mod p2_0;
#[doc = "I/O configuration register for pin P2[1]"]
pub struct P2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[1]"]
pub mod p2_1;
#[doc = "I/O configuration register for pin P2[2]"]
pub struct P2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[2]"]
pub mod p2_2;
#[doc = "I/O configuration register for pin P2[3]"]
pub struct P2_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[3]"]
pub mod p2_3;
#[doc = "I/O configuration register for pin P2[4]"]
pub struct P2_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[4]"]
pub mod p2_4;
#[doc = "I/O configuration register for pin P2[5]"]
pub struct P2_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[5]"]
pub mod p2_5;
#[doc = "I/O configuration register for pin P2[6]"]
pub struct P2_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[6]"]
pub mod p2_6;
#[doc = "I/O configuration register for pin P2[7]"]
pub struct P2_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[7]"]
pub mod p2_7;
#[doc = "I/O configuration register for pin P2[8]"]
pub struct P2_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[8]"]
pub mod p2_8;
#[doc = "I/O configuration register for pin P2[9]"]
pub struct P2_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[9]"]
pub mod p2_9;
#[doc = "I/O configuration register for pin P2[10]"]
pub struct P2_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[10]"]
pub mod p2_10;
#[doc = "I/O configuration register for pin P2[11]"]
pub struct P2_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[11]"]
pub mod p2_11;
#[doc = "I/O configuration register for pin P2[12]"]
pub struct P2_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[12]"]
pub mod p2_12;
#[doc = "I/O configuration register for pin P2[13]"]
pub struct P2_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[13]"]
pub mod p2_13;
#[doc = "I/O configuration register for pin P2[14]"]
pub struct P2_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[14]"]
pub mod p2_14;
#[doc = "I/O configuration register for pin P2[15]"]
pub struct P2_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[15]"]
pub mod p2_15;
#[doc = "I/O configuration register for pin P2[16]"]
pub struct P2_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[16]"]
pub mod p2_16;
#[doc = "I/O configuration register for pin P2[17]"]
pub struct P2_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[17]"]
pub mod p2_17;
#[doc = "I/O configuration register for pin P2[18]"]
pub struct P2_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[18]"]
pub mod p2_18;
#[doc = "I/O configuration register for pin P2[19]"]
pub struct P2_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[19]"]
pub mod p2_19;
#[doc = "I/O configuration register for pin P2[20]"]
pub struct P2_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[20]"]
pub mod p2_20;
#[doc = "I/O configuration register for pin P2[21]"]
pub struct P2_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[21]"]
pub mod p2_21;
#[doc = "I/O configuration register for pin P2[22]"]
pub struct P2_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[22]"]
pub mod p2_22;
#[doc = "I/O configuration register for pin P2[23]"]
pub struct P2_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[23]"]
pub mod p2_23;
#[doc = "I/O configuration register for pin P2[24]"]
pub struct P2_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[24]"]
pub mod p2_24;
#[doc = "I/O configuration register for pin P2[25]"]
pub struct P2_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[25]"]
pub mod p2_25;
#[doc = "I/O configuration register for pin P2[26]"]
pub struct P2_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[26]"]
pub mod p2_26;
#[doc = "I/O configuration register for pin P2[27]"]
pub struct P2_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[27]"]
pub mod p2_27;
#[doc = "I/O configuration register for pin P2[28]"]
pub struct P2_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[28]"]
pub mod p2_28;
#[doc = "I/O configuration register for pin P2[29]"]
pub struct P2_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[29]"]
pub mod p2_29;
#[doc = "I/O configuration register for pin P2[30]"]
pub struct P2_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[30]"]
pub mod p2_30;
#[doc = "I/O configuration register for pin P2[31]"]
pub struct P2_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P2[31]"]
pub mod p2_31;
#[doc = "I/O configuration register for pin P3[0]"]
pub struct P3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[0]"]
pub mod p3_0;
#[doc = "I/O configuration register for pin P3[1]"]
pub struct P3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[1]"]
pub mod p3_1;
#[doc = "I/O configuration register for pin P3[2]"]
pub struct P3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[2]"]
pub mod p3_2;
#[doc = "I/O configuration register for pin P3[3]"]
pub struct P3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[3]"]
pub mod p3_3;
#[doc = "I/O configuration register for pin P3[4]"]
pub struct P3_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[4]"]
pub mod p3_4;
#[doc = "I/O configuration register for pin P3[5]"]
pub struct P3_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[5]"]
pub mod p3_5;
#[doc = "I/O configuration register for pin P3[6]"]
pub struct P3_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[6]"]
pub mod p3_6;
#[doc = "I/O configuration register for pin P3[7]"]
pub struct P3_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[7]"]
pub mod p3_7;
#[doc = "I/O configuration register for pin P3[8]"]
pub struct P3_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[8]"]
pub mod p3_8;
#[doc = "I/O configuration register for pin P3[9]"]
pub struct P3_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[9]"]
pub mod p3_9;
#[doc = "I/O configuration register for pin P3[10]"]
pub struct P3_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[10]"]
pub mod p3_10;
#[doc = "I/O configuration register for pin P3[11]"]
pub struct P3_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[11]"]
pub mod p3_11;
#[doc = "I/O configuration register for pin P3[12]"]
pub struct P3_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[12]"]
pub mod p3_12;
#[doc = "I/O configuration register for pin P3[13]"]
pub struct P3_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[13]"]
pub mod p3_13;
#[doc = "I/O configuration register for pin P3[14]"]
pub struct P3_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[14]"]
pub mod p3_14;
#[doc = "I/O configuration register for pin P3[15]"]
pub struct P3_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[15]"]
pub mod p3_15;
#[doc = "I/O configuration register for pin P3[16]"]
pub struct P3_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[16]"]
pub mod p3_16;
#[doc = "I/O configuration register for pin P3[17]"]
pub struct P3_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[17]"]
pub mod p3_17;
#[doc = "I/O configuration register for pin P3[18]"]
pub struct P3_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[18]"]
pub mod p3_18;
#[doc = "I/O configuration register for pin P3[19]"]
pub struct P3_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[19]"]
pub mod p3_19;
#[doc = "I/O configuration register for pin P3[20]"]
pub struct P3_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[20]"]
pub mod p3_20;
#[doc = "I/O configuration register for pin P3[21]"]
pub struct P3_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[21]"]
pub mod p3_21;
#[doc = "I/O configuration register for pin P3[22]"]
pub struct P3_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[22]"]
pub mod p3_22;
#[doc = "I/O configuration register for pin P3[23]"]
pub struct P3_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[23]"]
pub mod p3_23;
#[doc = "I/O configuration register for pin P3[24]"]
pub struct P3_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[24]"]
pub mod p3_24;
#[doc = "I/O configuration register for pin P3[25]"]
pub struct P3_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[25]"]
pub mod p3_25;
#[doc = "I/O configuration register for pin P3[26]"]
pub struct P3_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[26]"]
pub mod p3_26;
#[doc = "I/O configuration register for pin P3[27]"]
pub struct P3_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[27]"]
pub mod p3_27;
#[doc = "I/O configuration register for pin P3[28]"]
pub struct P3_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[28]"]
pub mod p3_28;
#[doc = "I/O configuration register for pin P3[29]"]
pub struct P3_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[29]"]
pub mod p3_29;
#[doc = "I/O configuration register for pin P3[30]"]
pub struct P3_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[30]"]
pub mod p3_30;
#[doc = "I/O configuration register for pin P3[31]"]
pub struct P3_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P3[31]"]
pub mod p3_31;
#[doc = "I/O configuration register for pin P4[0]"]
pub struct P4_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[0]"]
pub mod p4_0;
#[doc = "I/O configuration register for pin P4[1]"]
pub struct P4_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[1]"]
pub mod p4_1;
#[doc = "I/O configuration register for pin P4[2]"]
pub struct P4_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[2]"]
pub mod p4_2;
#[doc = "I/O configuration register for pin P4[3]"]
pub struct P4_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[3]"]
pub mod p4_3;
#[doc = "I/O configuration register for pin P4[4]"]
pub struct P4_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[4]"]
pub mod p4_4;
#[doc = "I/O configuration register for pin P4[5]"]
pub struct P4_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[5]"]
pub mod p4_5;
#[doc = "I/O configuration register for pin P4[6]"]
pub struct P4_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[6]"]
pub mod p4_6;
#[doc = "I/O configuration register for pin P4[7]"]
pub struct P4_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[7]"]
pub mod p4_7;
#[doc = "I/O configuration register for pin P4[8]"]
pub struct P4_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[8]"]
pub mod p4_8;
#[doc = "I/O configuration register for pin P4[9]"]
pub struct P4_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[9]"]
pub mod p4_9;
#[doc = "I/O configuration register for pin P4[10]"]
pub struct P4_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[10]"]
pub mod p4_10;
#[doc = "I/O configuration register for pin P4[11]"]
pub struct P4_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[11]"]
pub mod p4_11;
#[doc = "I/O configuration register for pin P4[12]"]
pub struct P4_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[12]"]
pub mod p4_12;
#[doc = "I/O configuration register for pin P4[13]"]
pub struct P4_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[13]"]
pub mod p4_13;
#[doc = "I/O configuration register for pin P4[14]"]
pub struct P4_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[14]"]
pub mod p4_14;
#[doc = "I/O configuration register for pin P4[15]"]
pub struct P4_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[15]"]
pub mod p4_15;
#[doc = "I/O configuration register for pin P4[16]"]
pub struct P4_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[16]"]
pub mod p4_16;
#[doc = "I/O configuration register for pin P4[17]"]
pub struct P4_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[17]"]
pub mod p4_17;
#[doc = "I/O configuration register for pin P4[18]"]
pub struct P4_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[18]"]
pub mod p4_18;
#[doc = "I/O configuration register for pin P4[19]"]
pub struct P4_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[19]"]
pub mod p4_19;
#[doc = "I/O configuration register for pin P4[20]"]
pub struct P4_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[20]"]
pub mod p4_20;
#[doc = "I/O configuration register for pin P4[21]"]
pub struct P4_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[21]"]
pub mod p4_21;
#[doc = "I/O configuration register for pin P4[22]"]
pub struct P4_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[22]"]
pub mod p4_22;
#[doc = "I/O configuration register for pin P4[23]"]
pub struct P4_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[23]"]
pub mod p4_23;
#[doc = "I/O configuration register for pin P4[24]"]
pub struct P4_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[24]"]
pub mod p4_24;
#[doc = "I/O configuration register for pin P4[25]"]
pub struct P4_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[25]"]
pub mod p4_25;
#[doc = "I/O configuration register for pin P4[26]"]
pub struct P4_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[26]"]
pub mod p4_26;
#[doc = "I/O configuration register for pin P4[27]"]
pub struct P4_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[27]"]
pub mod p4_27;
#[doc = "I/O configuration register for pin P4[28]"]
pub struct P4_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[28]"]
pub mod p4_28;
#[doc = "I/O configuration register for pin P4[29]"]
pub struct P4_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[29]"]
pub mod p4_29;
#[doc = "I/O configuration register for pin P4[30]"]
pub struct P4_30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[30]"]
pub mod p4_30;
#[doc = "I/O configuration register for pin P4[31]"]
pub struct P4_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P4[31]"]
pub mod p4_31;
#[doc = "I/O configuration register for pin P5[0]"]
pub struct P5_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P5[0]"]
pub mod p5_0;
#[doc = "I/O configuration register for pin P5[1]"]
pub struct P5_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P5[1]"]
pub mod p5_1;
#[doc = "I/O configuration register for pin P5[2]"]
pub struct P5_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P5[2]"]
pub mod p5_2;
#[doc = "I/O configuration register for pin P5[3]"]
pub struct P5_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P5[3]"]
pub mod p5_3;
#[doc = "I/O configuration register for pin P5[4]"]
pub struct P5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration register for pin P5[4]"]
pub mod p5_4;
