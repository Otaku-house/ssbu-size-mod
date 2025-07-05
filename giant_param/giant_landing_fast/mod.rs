use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash::lib::lua_const::*;
use smash_script::*;
use smash::app::*;



pub static mut SCALE: [f32; 8] = [1.0; 8];

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;
        
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
            if SCALE[entry_id] >= 50.0
            && SCALE[entry_id] < 500.0 {
                MotionModule::set_rate(fighter.module_accessor, 1.02);
            }
            if SCALE[entry_id] >= 500.0
            && SCALE[entry_id] < 5000.0 {
                MotionModule::set_rate(fighter.module_accessor, 1.5);
            }
            if SCALE[entry_id] >= 5000.0
            && SCALE[entry_id] < 50000.0 {
                MotionModule::set_rate(fighter.module_accessor, 1.1);
            }
            if SCALE[entry_id] >= 50000.0 {
                MotionModule::set_rate(fighter.module_accessor, 1.2);
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