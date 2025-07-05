use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::phx::Vector3f;


#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        HitModule::set_check_catch(fighter.module_accessor, false, 0);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        eflame_frame
    );
    smashline::install_acmd_scripts!(
        
    );
}
