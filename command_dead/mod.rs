use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash::lib::lua_const::*;



#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_INSTANT_DEATH_RESERVED);
                }
            }
        }     
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
        
    );
}