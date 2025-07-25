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



mod daisy;
mod hikari;
mod homura;
mod kamui;
mod lucina;
mod master;
mod minmin;
mod palutena;
mod peach;
mod reflet;
mod rosalina;
mod szerosuit;
mod zelda;


pub fn install() {
    daisy::install();
    hikari::install();
    homura::install();
    kamui::install();
    lucina::install();
    master::install();
    minmin::install();
    palutena::install();
    peach::install();
    reflet::install();
    rosalina::install();
    szerosuit::install();
    zelda::install();

  }
