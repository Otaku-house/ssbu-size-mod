use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash_script::*;
use smash::app::*;


#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.lua_state_agent);

        
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                ItemModule::have_item(module_accessor, smash::app::ItemKind(*ITEM_KIND_MUSHROOM),0,0,true,true);
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                ItemModule::drop_item(module_accessor, 0.0, 0.0, 0);
            } 
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
        //global_weapon_frame
    );
}