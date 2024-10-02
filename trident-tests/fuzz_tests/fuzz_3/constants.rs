use anchor_lang::pubkey;
use trident_client::fuzzing::*;

pub const MAINNET_CORE_BRIDGE_ID: Pubkey = pubkey!("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth");

pub mod guardian_set_9_mock_nineteen_guardians {
    use super::*;

    pub const GUARDIAN_SET_9_MOCK_NINETEEN_GUARDIANS_ADDRESS: Pubkey =
        pubkey!("GLWXJeoT37zCViHRsFudGMBYDYo8ddcv6LyvJJw7hSjh");

    pub const NINETEEN_MOCK_GUARDIAN_SET_INDEX: u32 = 9;

    pub const BYTES: [u8; 220] = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 1, 0, 0, 0, 42, 1, 0, 2, 1, 0, 0, 0, 42, 0, 0, 0, 9,
        48, 120, 49, 51, 99, 53, 102, 52, 50, 1, 247, 19, 76, 225, 56, 131, 44, 20, 86, 242, 169,
        29, 100, 98, 30, 233, 12, 43, 221, 234, 0, 0, 0, 4, 215, 176, 254, 241, 1, 0, 2, 1, 0, 0,
        0, 85, 0, 0, 0, 0, 1, 60, 95, 66, 162, 46, 120, 199, 82, 55, 221, 141, 9, 88, 113, 92, 22,
        146, 57, 45, 147, 143, 44, 7, 157, 253, 87, 181, 34, 215, 55, 209, 204, 13, 144, 21, 0, 6,
        33, 233, 19, 13, 79, 192, 1, 0, 0, 0, 32, 41, 135, 97, 72, 28, 205, 162, 216, 66, 253, 126,
        253, 96, 145, 99, 131, 252, 59, 199, 205, 92, 216, 113, 166, 116, 204, 237, 179, 150, 67,
        232, 175,
    ];

    pub const ROOT_HASH: [u8; 32] = [
        41, 135, 97, 72, 28, 205, 162, 216, 66, 253, 126, 253, 96, 145, 99, 131, 252, 59, 199, 205,
        92, 216, 113, 166, 116, 204, 237, 179, 150, 67, 232, 175,
    ];

    pub const SIGNATURES: [[u8; 66]; 13] = [
        [
            0, 31, 125, 96, 252, 229, 161, 35, 81, 113, 150, 228, 99, 222, 136, 104, 1, 238, 118,
            154, 32, 5, 188, 104, 32, 186, 85, 13, 183, 51, 179, 255, 89, 23, 161, 19, 116, 59,
            138, 104, 25, 242, 63, 19, 52, 3, 150, 141, 107, 0, 98, 140, 170, 143, 153, 183, 60,
            115, 205, 22, 213, 141, 226, 174, 246, 0,
        ],
        [
            1, 217, 156, 25, 242, 243, 14, 236, 118, 209, 120, 13, 241, 142, 80, 172, 180, 167,
            207, 228, 200, 235, 208, 54, 152, 157, 242, 140, 93, 52, 86, 19, 34, 57, 180, 26, 248,
            241, 50, 154, 156, 189, 183, 176, 204, 160, 190, 38, 110, 1, 108, 190, 13, 37, 218, 45,
            59, 216, 67, 127, 1, 80, 178, 29, 205, 1,
        ],
        [
            2, 196, 137, 210, 193, 76, 72, 222, 32, 217, 186, 231, 6, 157, 161, 181, 204, 126, 226,
            37, 56, 27, 230, 101, 123, 18, 34, 31, 179, 71, 217, 174, 204, 14, 106, 126, 20, 127,
            196, 245, 215, 185, 190, 7, 106, 13, 245, 92, 163, 22, 49, 27, 103, 45, 153, 118, 103,
            70, 198, 184, 180, 96, 132, 9, 149, 0,
        ],
        [
            3, 126, 114, 167, 161, 204, 97, 200, 130, 192, 7, 127, 148, 30, 247, 86, 113, 222, 95,
            106, 169, 35, 39, 152, 107, 109, 66, 203, 192, 63, 155, 155, 117, 18, 106, 95, 201, 91,
            79, 241, 9, 144, 196, 255, 224, 91, 65, 3, 199, 210, 107, 139, 173, 94, 3, 187, 237,
            55, 198, 82, 50, 10, 168, 186, 91, 0,
        ],
        [
            4, 100, 226, 231, 199, 20, 157, 158, 228, 130, 140, 37, 219, 88, 46, 17, 83, 34, 230,
            76, 38, 208, 107, 215, 121, 203, 17, 205, 76, 13, 169, 42, 110, 10, 174, 36, 27, 229,
            134, 60, 156, 123, 196, 140, 129, 6, 254, 66, 154, 127, 248, 49, 64, 106, 255, 102,
            149, 155, 37, 32, 87, 229, 187, 213, 141, 0,
        ],
        [
            5, 164, 56, 33, 92, 191, 155, 234, 93, 195, 247, 89, 90, 124, 102, 212, 117, 191, 63,
            91, 114, 190, 84, 93, 235, 71, 138, 159, 127, 204, 217, 239, 97, 4, 3, 247, 60, 145,
            234, 216, 2, 188, 61, 216, 195, 79, 124, 129, 168, 223, 228, 175, 225, 14, 56, 158,
            154, 138, 159, 132, 217, 164, 132, 120, 164, 0,
        ],
        [
            6, 66, 172, 53, 46, 250, 223, 120, 46, 120, 55, 144, 69, 188, 199, 201, 125, 25, 2,
            191, 40, 152, 36, 151, 224, 172, 74, 151, 90, 201, 102, 203, 22, 84, 174, 214, 183, 1,
            127, 130, 60, 225, 135, 60, 214, 204, 7, 40, 208, 108, 170, 151, 172, 234, 173, 85,
            222, 59, 1, 238, 237, 20, 19, 108, 111, 0,
        ],
        [
            7, 218, 173, 254, 187, 14, 46, 179, 30, 58, 40, 171, 219, 63, 44, 148, 109, 81, 64,
            246, 84, 131, 107, 40, 232, 185, 158, 173, 103, 49, 119, 221, 195, 64, 237, 110, 219,
            136, 162, 109, 147, 234, 204, 169, 29, 231, 161, 66, 109, 180, 179, 239, 151, 169, 85,
            187, 223, 151, 63, 2, 160, 195, 64, 2, 102, 1,
        ],
        [
            8, 131, 71, 23, 196, 120, 121, 33, 49, 203, 226, 150, 170, 64, 159, 177, 41, 130, 215,
            137, 200, 144, 169, 5, 194, 48, 186, 109, 85, 87, 76, 148, 190, 64, 160, 208, 150, 32,
            187, 198, 161, 248, 232, 40, 242, 32, 20, 251, 209, 142, 175, 100, 189, 142, 254, 21,
            245, 246, 176, 83, 86, 96, 176, 214, 232, 1,
        ],
        [
            9, 135, 34, 104, 250, 0, 236, 208, 74, 65, 3, 90, 55, 253, 123, 192, 219, 53, 117, 224,
            54, 194, 134, 7, 167, 1, 10, 87, 163, 234, 79, 113, 254, 78, 68, 40, 8, 216, 20, 34,
            38, 13, 15, 34, 137, 128, 186, 220, 242, 99, 214, 146, 119, 161, 253, 168, 78, 200,
            166, 183, 5, 119, 21, 184, 255, 1,
        ],
        [
            10, 178, 161, 254, 207, 218, 27, 153, 72, 109, 2, 67, 148, 102, 44, 170, 150, 42, 57,
            173, 136, 183, 38, 73, 33, 144, 28, 27, 51, 120, 199, 182, 139, 56, 158, 34, 7, 14, 66,
            216, 136, 76, 29, 245, 8, 202, 187, 56, 157, 59, 253, 210, 228, 45, 192, 134, 174, 255,
            55, 220, 156, 69, 146, 89, 162, 0,
        ],
        [
            11, 212, 253, 188, 43, 48, 56, 172, 101, 251, 182, 1, 146, 7, 252, 133, 178, 165, 219,
            29, 69, 12, 80, 61, 238, 11, 66, 67, 145, 95, 205, 198, 215, 10, 141, 147, 145, 197,
            42, 129, 9, 101, 165, 197, 133, 202, 25, 161, 10, 125, 38, 131, 244, 81, 73, 5, 138,
            155, 60, 195, 24, 87, 156, 131, 154, 0,
        ],
        [
            12, 127, 163, 19, 149, 231, 12, 162, 112, 98, 112, 251, 29, 133, 242, 179, 208, 35,
            209, 25, 12, 71, 144, 243, 170, 247, 206, 92, 89, 240, 81, 158, 152, 3, 101, 1, 7, 34,
            2, 179, 74, 96, 147, 95, 235, 174, 124, 204, 140, 79, 158, 48, 97, 191, 43, 47, 88, 55,
            25, 81, 73, 117, 232, 144, 78, 0,
        ],
    ];
}