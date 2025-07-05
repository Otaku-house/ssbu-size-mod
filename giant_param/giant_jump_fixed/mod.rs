use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smashline::*;
use smash_script::*;
use smash::app::*;
use smash::phx::Vector3f;



pub static mut SCALE: [f32; 8] = [1.0; 8];




#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(0);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

          
        if status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_JUMP
        || status_kind == smash::lib::lua_const::FIGHTER_STATUS_KIND_JUMP_AERIAL {
            if SCALE[entry_id] >= 100.0 
            && SCALE[entry_id] < 1000.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 700000000.0, z: 0.0} as *const Vector3f);
            }
            if SCALE[entry_id] >= 1000.0 
            && SCALE[entry_id] < 10000.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 7000000000.0, z: 0.0} as *const Vector3f);
            }
            if SCALE[entry_id] >= 10000.0
            && SCALE[entry_id] < 100000.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 70000000000.0, z: 0.0} as *const Vector3f);
            }
            if SCALE[entry_id] >= 100000.0 {
                KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 700000000000.0, z: 0.0} as *const Vector3f);
            }
        }

    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
        
    );
    smashline::install_agent_frames!(
        
    );
}