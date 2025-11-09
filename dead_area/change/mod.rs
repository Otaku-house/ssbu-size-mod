use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash_script::*;
use smash::app::*;
use smash::phx::Vector3f;
use smash::phx::Vector4f;


pub static mut SS: [i32; 8] = [0; 8];

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;
        let mut counter = 0;

        let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
        
        //コマンドでスイッチ

        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                while counter != 7 {
                    SS[counter] = 0;
                    counter += 1;
                }
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                while counter != 7 {
                    SS[counter] = 1;
                    counter += 1;
                }
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                while counter != 7 {
                    SS[counter] = 2;
                    counter += 1;
                }
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                while counter != 7 {
                    SS[counter] = 3;
                    counter += 1;
                }
            }
        }
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                while counter != 7 {
                    SS[counter] = 0;
                    counter += 1;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                while counter != 7 {
                    SS[counter] = 1;
                    counter += 1;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                while counter != 7 {
                    SS[counter] = 2;
                    counter += 1;
                }
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                while counter != 7 {
                    SS[counter] = 3;
                    counter += 1;
                }
            }
        }

        //バーストライン変更

        if fighter_kind != *FIGHTER_KIND_PEACH
        && fighter_kind != *FIGHTER_KIND_DAISY
        && fighter_kind != *FIGHTER_KIND_ZELDA
        && fighter_kind != *FIGHTER_KIND_ROSETTA
        && fighter_kind != *FIGHTER_KIND_PALUTENA
        && fighter_kind != *FIGHTER_KIND_SZEROSUIT
        && fighter_kind != *FIGHTER_KIND_KAMUI
        && fighter_kind != *FIGHTER_KIND_REFLET
        && fighter_kind != *FIGHTER_KIND_LUCINA
        && fighter_kind != *FIGHTER_KIND_WIIFIT
        && fighter_kind != *FIGHTER_KIND_BAYONETTA
        && fighter_kind != *FIGHTER_KIND_MASTER
        && fighter_kind != *FIGHTER_KIND_TANTAN
        && fighter_kind != *FIGHTER_KIND_EFLAME
        && fighter_kind != *FIGHTER_KIND_ELIGHT 
        {
            if SS[entry_id] == 0 {
                if pos.x < -300.0 
                || pos.x > 300.0
                || pos.y < -300.0
                || pos.y > 300.0
                {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_INSTANT_DEATH_RESERVED)
                } 
            }
            if SS[entry_id] == 1 {
                if pos.x < -3000.0 
                || pos.x > 3000.0
                || pos.y < -3000.0
                || pos.y > 3000.0
                {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_INSTANT_DEATH_RESERVED)
                } 
            }
            if SS[entry_id] == 2 {
                if pos.x < -10000.0 
                || pos.x > 10000.0
                || pos.y < -10000.0
                || pos.y > 10000.0
                {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_INSTANT_DEATH_RESERVED)
                } 
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