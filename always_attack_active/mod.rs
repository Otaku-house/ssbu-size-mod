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



//mod always_attack;
//mod appeal_attack;
mod attack_fixed;
//mod entry_attack;
//mod wait_attack;
//mod walk_attack;

pub fn install() {
    //always_attack::install();
    //appeal_attack::install();
    attack_fixed::install();
    //entry_attack::install();
    //wait_attack::install();
    //walk_attack::install();
    
  }

