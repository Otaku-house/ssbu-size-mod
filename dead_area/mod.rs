//#![feature(concat_idents)]
//#![feature(proc_macro_hygiene)]
//#![feature(asm)]
//#[warn(stable_features)]

#![feature(
    concat_idents, 
    proc_macro_hygiene, 
    repr_simd, 
    simd_ffi
)]
#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_assignments, unused_macros, unused_must_use, unused_mut, unused_parens, unused_unsafe, unused_variables)]


mod xy300;
//mod xy300_fly;
//mod xy1000;
//mod xy1000_fly;

pub fn install() {
    xy300::install();
    //xy300_fly::install();
    //xy1000::install();
    //xy1000_fly::install();
    
  }

