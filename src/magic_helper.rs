use bit_twiddles;

pub static ROOK_MASK: &'static [u64] = &[
    0x7e01010101010100, // a1
    0x7c02020202020200, // a2
    0x7a04040404040400, // a3
    0x7608080808080800, // a4
    0x6e10101010101000, // a5
    0x5e20202020202000, // a6
    0x3e40404040404000, // a7
    0x7e80808080808000, // a8
    0x7e010101010100,   // b1
    0x7c020202020200,   // b2
    0x7a040404040400,   // b3
    0x76080808080800,   // b4
    0x6e101010101000,   // b5
    0x5e202020202000,   // b6
    0x3e404040404000,   // b7
    0x7e808080808000,   // b8
    0x17e0101010100,    // c1
    0x27c0202020200,    // c2
    0x47a0404040400,    // c3
    0x8760808080800,    // c4
    0x106e1010101000,   // c5
    0x205e2020202000,   // c6
    0x403e4040404000,   // c7
    0x807e8080808000,   // c8
    0x1017e01010100,    // d1
    0x2027c02020200,    // d2
    0x4047a04040400,    // d3
    0x8087608080800,    // d4
    0x10106e10101000,   // d5
    0x20205e20202000,   // d6
    0x40403e40404000,   // d7
    0x80807e80808000,   // d8
    0x101017e010100,    // e1
    0x202027c020200,    // e2
    0x404047a040400,    // e3
    0x8080876080800,    // e4
    0x1010106e101000,   // e5
    0x2020205e202000,   // e6
    0x4040403e404000,   // e7
    0x8080807e808000,   // e8
    0x10101017e0100,    // f1
    0x20202027c0200,    // f2
    0x40404047a0400,    // f3
    0x8080808760800,    // f4
    0x101010106e1000,   // f5
    0x202020205e2000,   // f6
    0x404040403e4000,   // f7
    0x808080807e8000,   // f8
    0x1010101017e00,    // g1
    0x2020202027c00,    // g2
    0x4040404047a00,    // g3
    0x8080808087600,    // g4
    0x10101010106e00,   // g5
    0x20202020205e00,   // g6
    0x40404040403e00,   // g7
    0x80808080807e00,   // g8
    0x101010101017e,    // h1
    0x202020202027c,    // h2
    0x404040404047a,    // h3
    0x8080808080876,    // h4
    0x1010101010106e,   // h5
    0x2020202020205e,   // h6
    0x4040404040403e,   // h7
    0x8080808080807e,   // h8
];

pub static BISHOP_MASK: &'static [u64] = &[
    0x2040810204000,     // a1
    0x4081020400000,     // a2
    0xa102040000000,     // a3
    0x14224000000000,    // a4
    0x28440200000000,    // a5
    0x50080402000000,    // a6
    0x20100804020000,    // a7
    0x40201008040200,     // a8
    0x20408102000,       // b1
    0x40810204000,       // b2
    0xa1020400000,       // b3
    0x142240000000,      // b4
    0x284402000000,      // b5
    0x500804020000,      // b6
    0x201008040200,      // b7
    0x402010080400,      // b8
    0x2000204081000,     // c1
    0x4000408102000,     // c2
    0xa000a10204000,     // c3
    0x14001422400000,    // c4
    0x28002844020000,    // c5
    0x50005008040200,    // c6
    0x20002010080400,    // c7
    0x40004020100800,    // c8
    0x4020002040800,     // d1
    0x8040004081000,     // d2
    0x100a000a102000,    // d3
    0x22140014224000,    // d4
    0x44280028440200,    // d5
    0x8500050080400,     // d6
    0x10200020100800,    // d7
    0x20400040201000,    // d8
    0x8040200020400,     // e1
    0x10080400040800,    // e2
    0x20100a000a1000,    // e3
    0x40221400142200,    // e4
    0x2442800284400,     // e5
    0x4085000500800,     // e6
    0x8102000201000,     // e7
    0x10204000402000,    // e8
    0x10080402000200,    // f1
    0x20100804000400,    // f2
    0x4020100a000a00,    // f3
    0x402214001400,      // f4
    0x24428002800,       // f5
    0x2040850005000,     // f6
    0x4081020002000,     // f7
    0x8102040004000,     // f8
    0x20100804020000,    // g1
    0x40201008040000,    // g2
    0x4020100a0000,      // g3
    0x4022140000,        // g4
    0x244280000,         // g5
    0x20408500000,       // g6
    0x2040810200000,     // g7
    0x4081020400000,     // g8
    0x40201008040200,    // h1
    0x402010080400,      // h2
    0x4020100a00,        // h3
    0x40221400,          // h4
    0x2442800,           // h5
    0x204085000,         // h6
    0x20408102000,       // h7
    0x2040810204000,     // h8
];

pub static MAGIC_ROOK_NUM: &'static [u64] = &[
    0x8080010a601241,     // a1
    0x1008010400021,      // a2
    0x4082001007241,      // a3
    0x211009001200509,    // a4
    0x8015001002441801,   // a5
    0x801000804000603,    // a6
    0xc0900220024a401,    // a7
    0x1000200608243,      // a8
    0x102042111804200,    // b1
    0x40002010004001c0,   // b2
    0x19220045508200,     // b3
    0x20030010060a900,    // b4
    0x8018028040080,      // b5
    0x88240002008080,     // b6
    0x10301802830400,     // b7
    0x332a4081140200,     // b8
    0x2240088020c28000,   // c1
    0x1001201040c004,     // c2
    0xa02008010420020,    // c3
    0x10003009010060,     // c4
    0x4008008008014,      // c5
    0x80020004008080,     // c6
    0x282020001008080,    // c7
    0x50000181204a0004,   // c8
    0x8180004000402000,   // d1
    0x488c402000401001,   // d2
    0x4018a00080801004,   // d3
    0x1230002105001008,   // d4
    0x8904800800800400,   // d5
    0x42000c42003810,     // d6
    0x8408110400b012,     // d7
    0x18086182000401,     // d8
    0x4204400080008ea0,   // e1
    0xb002400180200184,   // e2
    0x2020200080100380,   // e3
    0x10080080100080,     // e4
    0x2204080080800400,   // e5
    0xa40080360080,       // e6
    0x2040604002810b1,    // e7
    0x8c218600004104,     // e8
    0x208002904001,       // f1
    0x90004040026008,     // f2
    0x208808010002001,    // f3
    0x2002020020704940,   // f4
    0x8048010008110005,   // f5
    0x6820808004002200,   // f6
    0xa80040008023011,    // f7
    0xb1460000811044,     // f8
    0x10138001a080c010,   // g1
    0x804008200480,       // g2
    0x10011012000c0,      // g3
    0x22004128102200,     // g4
    0x200081201200c,      // g5
    0x202a001048460004,   // g6
    0x81000100420004,     // g7
    0x4000800380004500,   // g8
    0xa180022080400230,   // h1
    0x40100040022000,     // h2
    0x80088020001002,     // h3
    0x80080280841000,     // h4
    0x4200042010460008,   // h5
    0x4800a0003040080,    // h6
    0x400110082041008,    // h7
    0x8000a041000880,     // h8
];

pub static MAGIC_BISHOP_NUM: &'static [u64] = &[
    0x2400202602104000,   // a1
    0x208520209440204,    // a2
    0x40c000022013020,    // a3
    0x2000104000420600,   // a4
    0x400000260142410,    // a5
    0x800633408100500,    // a6
    0x2404080a1410,       // a7
    0x138200122002900,     // a8
    0x40201a444400810,    // b1
    0x4611010802020008,   // b2
    0x80000b0401040402,   // b3
    0x20004821880a00,     // b4
    0x8200002022440100,   // b5
    0x9431801010068,      // b6
    0x1040c20806108040,   // b7
    0x804901403022a40,    // b8
    0x412101004020818,    // c1
    0x8022080a09404208,   // c2
    0x1401210240484800,   // c3
    0x22244208010080,     // c4
    0x1105040104000210,   // c5
    0x2040088800c40081,   // c6
    0x8184810252000400,   // c7
    0x4004610041002200,   // c8
    0x244100800400200,    // d1
    0x4000901010080696,   // d2
    0x280404180020,       // d3
    0x800042008240100,    // d4
    0x220008400088020,    // d5
    0x4020182000904c9,    // d6
    0x23010400020600,     // d7
    0x41040020110302,     // d8
    0x20208050a42180,     // e1
    0x1004804b280200,     // e2
    0x2048020024040010,   // e3
    0x102c04004010200,    // e4
    0x20408204c002010,    // e5
    0x2411100020080c1,    // e6
    0x102a008084042100,   // e7
    0x941030000a09846,    // e8
    0x940000410821212,    // f1
    0x1808024a280210,     // f2
    0x40c0422080a0598,    // f3
    0x4228020082004050,   // f4
    0x200800400e00100,    // f5
    0x20b001230021040,    // f6
    0x90a0201900c00,      // f7
    0x4940120a0a0108,     // f8
    0x8005181184080048,   // g1
    0x1001c20208010101,   // g2
    0x1001080204002100,   // g3
    0x1810080489021800,   // g4
    0x62040420010a00,     // g5
    0x5028043004300020,   // g6
    0xc0080a4402605002,   // g7
    0x8a00a0104220200,    // g8
    0x2910054208004104,   // h1
    0x2100630a7020180,    // h2
    0x5822022042000000,   // h3
    0x2ca804a100200020,   // h4
    0x204042200000900,    // h5
    0x2002121024000002,   // h6
    0x80404104202000e8,   // h7
    0x812a020205010840,   // h8
];

pub static MAGIC_ROOK_SHIFTS: &'static [u64] = &[
    52,   // a1
    53,   // a2
    53,   // a3
    53,   // a4
    53,   // a5
    53,   // a6
    53,   // a7
    52,    // a8
    53,   // b1
    54,   // b2
    54,   // b3
    54,   // b4
    54,   // b5
    54,   // b6
    54,   // b7
    53,   // b8
    53,   // c1
    54,   // c2
    54,   // c3
    54,   // c4
    54,   // c5
    54,   // c6
    54,   // c7
    53,   // c8
    53,   // d1
    54,   // d2
    54,   // d3
    54,   // d4
    54,   // d5
    54,   // d6
    54,   // d7
    53,   // d8
    53,   // e1
    54,   // e2
    54,   // e3
    54,   // e4
    54,   // e5
    54,   // e6
    54,   // e7
    53,   // e8
    53,   // f1
    54,   // f2
    54,   // f3
    54,   // f4
    54,   // f5
    54,   // f6
    54,   // f7
    53,   // f8
    53,   // g1
    54,   // g2
    54,   // g3
    54,   // g4
    54,   // g5
    54,   // g6
    54,   // g7
    53,   // g8
    52,   // h1
    53,   // h2
    53,   // h3
    53,   // h4
    53,   // h5
    53,   // h6
    53,   // h7
    52,   // h8
];

pub static MAGIC_BISHOP_SHIFTS: &'static [u64] = &[
    58,   // a1
    59,   // a2
    59,   // a3
    59,   // a4
    59,   // a5
    59,   // a6
    59,   // a7
    58,   // a8
    59,   // b1
    59,   // b2
    59,   // b3
    59,   // b4
    59,   // b5
    59,   // b6
    59,   // b7
    59,   // b8
    59,   // c1
    59,   // c2
    57,   // c3
    57,   // c4
    57,   // c5
    57,   // c6
    59,   // c7
    59,   // c8
    59,   // d1
    59,   // d2
    57,   // d3
    55,   // d4
    55,   // d5
    57,   // d6
    59,   // d7
    59,   // d8
    59,   // e1
    59,   // e2
    57,   // e3
    55,   // e4
    55,   // e5
    57,   // e6
    59,   // e7
    59,   // e8
    59,   // f1
    59,   // f2
    57,   // f3
    57,   // f4
    57,   // f5
    57,   // f6
    59,   // f7
    59,   // f8
    59,   // g1
    59,   // g2
    59,   // g3
    59,   // g4
    59,   // g5
    59,   // g6
    59,   // g7
    59,   // g8
    58,   // h1
    59,   // h2
    59,   // h3
    59,   // h4
    59,   // h5
    59,   // h6
    59,   // h7
    58,   // h8
];




struct MagicHelper {
    magic_bishop_moves: [[u64; 4096]; 64],
    magic_rook_moves: [[u64; 1024]; 64],
    knight_moves: [u64; 64],
    king_moves: [u64; 64]
}

//impl MagicHelper {
//    pub fn new() -> MagicHelper {
//        MagicHelper {
//            magic_bishop_moves: MagicHelper::gen_magic_bishop(),
//            magic_rook_moves: MagicHelper::gen_magic_rook()
//        }
//    }
//
//    pub fn default() -> MagicHelper { MagicHelper::new() }
//
//    fn gen_magic_bishop() -> [[u64; 4096]; 64] {
//        let mut arr: [[u64; 4096]; 64] = [[0; 4096]; 64];
//        let mut mask: u64 = 0;
//        for bitRef in 0..64 {
//            mask = BISHOP_MASK[bitRef];
//
//        }
//
//    }
//
//    fn gen_magic_rook() -> [[u64; 1024]; 64] {
//        let mut arr: [[u64; 1024]; 64] = [[0; 1024]; 64];
//
//    }
//}

fn gen_king_moves() -> [u64; 64] {
    let mut moves: [u64;64] = [0; 64];

    for index in 0..64 {
        let mut mask: u64 = 0;
        let file = index % 8;
        // LEFT
        if file != 0 {
            mask |= 1 << (index - 1);
        }
        // RIGHT
        if file != 7 {
            mask |= 1 << (index + 1);
        }
        // UP
        if index < 56  {
            mask |= 1 << (index + 8);
        }
        // DOWN
        if index > 7  {
            mask |= 1 << (index - 8);
        }
        // LEFT UP
        if file != 0 && index < 56 {
            mask |= 1 << (index + 7);
        }
        // LEFT DOWN
        if file != 0 && index > 7 {
            mask |= 1 << (index - 9);
        }
        // RIGHT DOWN
        if file!= 7 && index > 7 {
            mask |= 1 << (index - 7);
        }
        // RIGHT UP
        if file != 7 && index < 56 {
            mask |= 1 << (index + 0);
        }
        moves[index] = mask;
    }
    moves
}

fn gen_knight_moves() -> [u64; 64] {
    let mut moves: [u64;64] = [0; 64];
    for index in 0..64 {
        let mut mask: u64 = 0;
        let file = index % 8;

        // 1 UP   + 2 LEFT
        if file > 1 && index < 56 {
            mask |= 1 << (index + 6);
        }
        // 2 UP   + 1 LEFT
        if file != 0 && index < 48 {
            mask |= 1 << (index + 15);
        }
        // 2 UP   + 1 RIGHT
        if file != 7 && index < 48 {
            mask |= 1 << (index + 17);
        }
        // 1 UP   + 2 RIGHT
        if file < 6 && index < 56 {
            mask |= 1 << (index + 10);
        }
        // 1 DOWN   + 2 RIGHT
        if file < 6 && index > 7 {
            mask |= 1 << (index - 6);
        }
        // 2 DOWN   + 1 RIGHT
        if file != 7 && index > 15 {
            mask |= 1 << (index - 15 );
        }
        // 2 DOWN   + 1 LEFT
        if file != 0 && index > 15 {
            mask |= 1 << (index - 17 );
        }
        // 1 DOWN   + 2 LEFT
        if file > 1 && index > 7 {
            mask |= 1 << (index - 10 );
        }
        moves[index] = mask;
    }
    moves
}

pub fn gen_rook_masks() {
    let mut arr_masks: [u64; 64] = [0; 64];
    let mut shifts: [u8; 64] = [0; 64];

    let mut bit_ref: usize = 0;
    while bit_ref < 64 {
        let mut mask: u64 = 0;
        let mut i = bit_ref + 8;
        while i < 56 {
            mask |= (1 as u64) << (i as u8);
            i += 8;
        }
        if bit_ref > 7 {
            let mut i = bit_ref - 8;
            while i > 7 {
                mask |= (1 as u64) << (i as u8);
                i -= 8;
            }
        }
        let mut i = bit_ref + 1;
        while i % 8 != 0 && i <= 63 {
            mask |= (1 as u64) << (i as u8);
            i += 1;
        }
        if bit_ref > 0  {
            let mut i = bit_ref - 1;
            while i % 8 != 0 && i >= 0 {
                mask |= (1 as u64) << (i as u8);
                i -= 1;
            }
        }
        arr_masks[bit_ref] = mask;
        format_bits(format!("{:b}",mask));
        bit_ref += 1;
    }
}

pub fn gen_bishop_masks() {
    let mut arr_masks: [u64; 64] = [0; 64];
    let mut shifts: [u8; 64] = [0; 64];

    let mut bitRef: i32 = 0;
    while bitRef < 64 {
        let mut mask: u64 = 0;

        let mut i = bitRef + 9;
        while i < 56 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i += 9;
        }
        let mut i = bitRef - 9;
        while i > 7 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i -= 9;
        }
        let mut i = bitRef + 7;
        while i < 56 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i += 7;
        }

        let mut i = bitRef - 7;
        while i > 7 && i % 8 != 0 && i % 8 != 7 {
            mask |= (1 as u64) << (i as u8);
            i -= 7;
        }
        format_bits(format!("{:b}",mask));
        arr_masks[bitRef as usize] = mask;
        bitRef += 1;
    }
}

pub fn format_bits(bits: String) {
    let x = 64 - bits.len();
    let mut i = 0;
    while i < x {
        print!("0");
        i += 1;
    }
    println!("{}",bits);
}



#[test]
fn test_kings() {
    let arr = gen_king_moves().to_vec();
    let sum = arr.iter().fold(0 as  u64,|a, &b| a + (bit_twiddles::pop_count(b) as u64));
    assert_eq!(sum, (3*4) + (5 * 6 * 4) + (8 * 6 * 6));
}

#[test]
fn test_knights() {
    let arr = gen_knight_moves().to_vec();
    let sum = arr.iter().fold(0 as  u64,|a, &b| a + (bit_twiddles::pop_count(b) as u64));
    assert_eq!(sum, (2 * 4) + (4 * 4) + (3 * 2 * 4) + (4 * 4 * 4) + (6 * 4 * 4) + (8 * 4 * 4));
}