use crate::{init_data, zero_bss};

#[test]
fn test_init_data() {
    let mut data = [0u32, 1, 2, 3, 4, 5, 6, 7];
    let mut mem = [0u32; 8];
    let sdata = &mut data[0] as *mut u32;
    let sidata = &mut mem[0] as *mut u32;
    unsafe {
        let edata = sdata.offset(data.len() as isize);
        init_data::<u32>(sdata, edata, sidata);
    };

    assert_eq!(data, mem);
}

#[test]
fn test_zero_bss() {
    let mut mem = [0xFFFF_FFFFu32; 8];
    let sbss = &mut mem[0] as *mut u32;
    unsafe {
        let ebss = sbss.offset(mem.len() as isize);
        zero_bss::<u32>(sbss, ebss);
    };

    assert_eq!(mem, [0u32; 8]);
}
