use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash_script::*;
use smash::app::*;
use smash::phx::*;

pub static mut ROTX: [f32; 8] = [0.0; 8];
pub static mut ROTY: [f32; 8] = [0.0; 8];
pub static mut ZOOM: [f32; 8] = [3.0; 8];
pub static mut SWITCH: [f32; 8] = [0.0; 8];
pub static mut SCALE: [f32; 8] = [1.0; 8];

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if sv_information::is_ready_go() == false {
            ROTX[entry_id] = 0.0;
            ROTY[entry_id] = 0.0;
            ZOOM[entry_id] = 3.0;
            SWITCH[entry_id] = 0.0;
            SCALE[entry_id] = 1.0;
        }
         
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 3.0 * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 3.0 * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                CameraModule::set_camera_type(module_accessor, 1); //カメラリセット
                SWITCH[entry_id] = 0.0;
                ROTX[entry_id] = 0.0;
                ROTY[entry_id] = 0.0;
                ZOOM[entry_id] = 3.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                CameraModule::set_camera_type(module_accessor, 1); //カメラリセット
                SWITCH[entry_id] = 0.0;
                ROTX[entry_id] = 0.0;
                ROTY[entry_id] = 0.0;
                ZOOM[entry_id] = 3.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                CameraModule::set_camera_type(module_accessor, 9); //カメラ位置固定
                SWITCH[entry_id] = 1.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                CameraModule::set_camera_type(module_accessor, 9); //カメラ位置固定
                SWITCH[entry_id] = 1.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if ControlModule::get_stick_y(module_accessor) >= 0.8 {
                if SWITCH[entry_id] == 1.0 {
                    ROTY[entry_id] -= 0.05;
                }
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
            if ControlModule::get_stick_y(module_accessor) <= -0.8 {
                if SWITCH[entry_id] == 1.0 {
                    ROTY[entry_id] += 0.05;
                }
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
            if ControlModule::get_stick_x(module_accessor) >= 0.8 {
                if SWITCH[entry_id] == 1.0 {
                    ROTX[entry_id] += 0.05;
                }
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
            if ControlModule::get_stick_x(module_accessor) <= -0.8 {
                if SWITCH[entry_id] == 1.0 {
                    ROTX[entry_id] -= 0.05;
                }    
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
        }
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            if ControlModule::get_stick_y(module_accessor) >= 0.8 {
                if SWITCH[entry_id] == 1.0 {
                    if ZOOM[entry_id] - 0.1 >= 0.5 {
                        if SCALE[entry_id] >= 10.0 {
                            ZOOM[entry_id] = ZOOM[entry_id] - 1.0;
                        }
                        else {
                            ZOOM[entry_id] = ZOOM[entry_id] - 0.1;
                        }
                        
                    }
                    else {
                        ZOOM[entry_id] = 0.5;
                    }
                }
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
            }
            if ControlModule::get_stick_y(module_accessor) <= -0.8 {
                if SWITCH[entry_id] == 1.0 {
                    if SCALE[entry_id] >= 10.0 {
                        ZOOM[entry_id] = ZOOM[entry_id] + 1.0;
                    }
                    else {
                        ZOOM[entry_id] = ZOOM[entry_id] + 0.1;
                    }
                }
                CameraModule::set_camera_type(module_accessor, 1);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ ZOOM[entry_id] * SCALE[entry_id],/*yrot*/ ROTY[entry_id],/*xrot*/ ROTX[entry_id] );
                SWITCH[entry_id] = 1.0;
                
            }
        }
        
        
        
        

    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
}