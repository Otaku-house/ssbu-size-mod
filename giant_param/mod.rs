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


//mod giant_jump_fixed;
mod giant_landing_fast;
//mod giant_walk_speed;



pub fn install() {
    //giant_jump_fixed::install();
    giant_landing_fast::install();
    //giant_walk_speed::install();
    
    
  }

