use crate::{init_data, zero_bss};

#[test]
fn test_init_data() {
    let mut data = [0u32, 1, 2, 3, 4, 5, 6, 7];
    let mut mem = [0u32; 8];
    let sdata = data.as_mut_ptr();
    let sidata = mem.as_mut_ptr();
    unsafe {
        let edata = sdata.offset(data.len() as isize);
        init_data::<u32>(sdata, edata, sidata);
    };

    assert_eq!(data, mem);
}

#[test]
fn test_zero_bss() {
    let mut mem = [0xFFFF_FFFFu32; 8];
    let sbss = mem.as_mut_ptr();
    unsafe {
        let ebss = sbss.offset(mem.len() as isize);
        zero_bss::<u32>(sbss, ebss);
    };

    assert_eq!(mem, [0u32; 8]);
}
