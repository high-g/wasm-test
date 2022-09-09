#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

extern {
    fn date_now() -> f64;
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe { date_now() }
}

extern crate tinymt;

use tinymt::tinymt32;

#[no_mangle]
pub fn rand() -> u32 {
    let param = tinymt32::Param {
        mat1: 0x8F7011EE,
        mat2: 0xFC78FF1F,
        tmat: 0x3793fdff,
    };

    let seed = 1;
    tinymt32::from_seed(param, seed).gen()
}

#[no_mangle]
pub fn heavy_loop(loop_num: u32) {
    let mut i: u32 = 0;

    loop {
        i += 1;
        tarai(i + 6, i +2, i);

        if i == loop_num {
            break;
        }
    }
}

fn tarai(x: u32, y: u32, z: u32) -> u32 {
    if x <= y {
        y
    } else {
        tarai(tarai(x - 1, y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))
    }
}

