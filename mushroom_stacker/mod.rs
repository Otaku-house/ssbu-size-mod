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


//mod no_reset;
mod normal;
//mod reverse;
//mod reverse_no_reset;
//mod x5;


pub fn install() {
    //no_reset::install();
    normal::install();
    //reverse::install();
    //reverse_no_reset::install();
    //x5::install();
    
  }

