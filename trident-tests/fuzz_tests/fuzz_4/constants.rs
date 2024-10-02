use anchor_lang::pubkey;
use trident_client::fuzzing::*;

pub const MAINNET_CORE_BRIDGE_ID: Pubkey = pubkey!("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth");

pub mod quardian_set_5_mock {
    use super::*;

    pub const MOCK_GUARDIAN_SET_INDEX: u32 = 5;
    pub const GUARDIAN_SET_5_MOCK: Pubkey = pubkey!("6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE");

    pub const BYTES: [u8; 220] = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 1, 0, 0, 0, 42, 1, 0, 2, 1, 0, 0, 0, 42, 0, 0, 0, 9,
        48, 120, 49, 51, 99, 53, 102, 100, 51, 1, 247, 19, 76, 225, 56, 131, 44, 20, 86, 242, 169,
        29, 100, 98, 30, 233, 12, 43, 221, 234, 0, 0, 0, 4, 215, 176, 254, 241, 1, 0, 2, 1, 0, 0,
        0, 85, 0, 0, 0, 0, 1, 60, 95, 212, 103, 225, 14, 53, 102, 41, 226, 41, 234, 86, 4, 102,
        175, 109, 33, 7, 241, 81, 132, 121, 105, 55, 6, 9, 187, 52, 210, 110, 87, 75, 188, 1, 0, 6,
        33, 233, 123, 122, 181, 192, 1, 0, 0, 0, 32, 5, 98, 140, 206, 245, 181, 133, 249, 165, 175,
        183, 100, 210, 40, 53, 242, 199, 27, 16, 190, 180, 178, 18, 228, 94, 201, 228, 208, 53, 76,
        151, 100,
    ];

    pub const ROOT_HASH: [u8; 32] = [
        5, 98, 140, 206, 245, 181, 133, 249, 165, 175, 183, 100, 210, 40, 53, 242, 199, 27, 16,
        190, 180, 178, 18, 228, 94, 201, 228, 208, 53, 76, 151, 100,
    ];

    pub const SIGNATURES: [[u8; 66]; 1] = [[
        0, 131, 85, 219, 204, 35, 255, 55, 67, 240, 121, 203, 19, 27, 82, 138, 44, 165, 200, 116,
        17, 156, 73, 173, 170, 25, 185, 116, 33, 57, 169, 0, 120, 44, 52, 95, 149, 168, 88, 131,
        63, 237, 153, 58, 59, 101, 122, 196, 52, 176, 250, 253, 52, 116, 8, 170, 79, 153, 117, 81,
        7, 135, 2, 35, 93, 1,
    ]];

    pub const RANDOM_INDEX1: usize = 5;

    pub const REFERENCE_SIG_VALUE1: u8 = 35;
}
