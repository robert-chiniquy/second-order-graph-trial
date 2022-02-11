pub const TEST_SIZE: u32 = 900;
pub const TEST_STEP: usize = 300;
pub const TEST_UNIT: u32 = 8;

pub fn example_input(size: u32) -> Vec<(u32, u32)> {
    let mut ret: Vec<(u32, u32)> = Vec::with_capacity(size as usize * TEST_UNIT as usize);

    for i in 1..size {
        for j in 1..(TEST_UNIT / 2) {
            ret.append(&mut vec![
                (11 * i * j, 10 * i * j),
                (13 * i * j, 20 * i * j),
            ])
        }
    }

    ret
}
