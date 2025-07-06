use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

pub static mut SCALE: [f32; 8] = [1.0; 8];
pub static mut SCALEF: [f32; 8] = [1.0; 8];
pub static mut SCALEE: [f32; 8] = [1.0; 8];
pub static mut SCALEL: [f32; 8] = [1.0; 8];
pub static mut STF: [f32; 8] = [0.0; 8];
pub static mut STE: [f32; 8] = [0.0; 8];
pub static mut STL: [f32; 8] = [0.0; 8];
pub static mut CT: [i32; 8] = [0; 8];
pub static mut ID: [usize; 8] = [0; 8];


#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let damage_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DAMAGE_ENTRY_ID) as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        
        if sv_information::is_ready_go() == true {
            if status_kind == *FIGHTER_STATUS_KIND_DEAD {
                if MotionModule::frame(fighter.module_accessor) <= 1.0 {
                    if damage_id >= 0 && damage_id <= 7 {
                        if ID[entry_id] == 0 {
                            CT[damage_id] = CT[damage_id] + 1;
                            ID[entry_id] = 1;
                        }
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
                ID[entry_id] = 0;
            }
        } 
        if sv_information::is_ready_go() == false {
            CT[entry_id] = 0; 
            ID[entry_id] = 0;
        } 

        if sv_information::is_ready_go() == true {
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 
        || SCALE[entry_id] < 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if fighter_kind == *FIGHTER_KIND_EFLAME {
            if STE[entry_id] == 1.0 {
                if SCALEE[entry_id] == SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / shield, *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = 1.0 / shield;
                    STE[entry_id] = 0.0;
                }
                if SCALEF[entry_id] != SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEF[entry_id] = SCALEE[entry_id];
                    STE[entry_id] = 0.0;
                }
            }
            if STE[entry_id] == 0.0 {
                SCALEF[entry_id] = 1.0 / shield;
                if SCALEF[entry_id] != SCALEE[entry_id] {
                    STF[entry_id] = 1.0;
                }
                if SCALEF[entry_id] == SCALEE[entry_id] {
                    STF[entry_id] = 0.0;
                }
            }
            if sv_information::is_ready_go() == false {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEF[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0;
                STF[entry_id] = 0.0;
                STE[entry_id] = 0.0;
            }
    
            if SCALEF[entry_id] > 1.0 
            || SCALEF[entry_id] < 1.0 {
                AttackModule::set_power_up(module_accessor, SCALEF[entry_id]);
                DamageModule::set_damage_mul(module_accessor, 1.0 / SCALEF[entry_id]);
                SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
                SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
            }
        }

        if fighter_kind == *FIGHTER_KIND_ELIGHT {
            if STF[entry_id] == 1.0 {
                if SCALEF[entry_id] == SCALEE[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / shield, *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = 1.0 / shield;
                    STF[entry_id] = 0.0;
                }
                if SCALEE[entry_id] != SCALEF[entry_id] {
                    ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                    SCALEE[entry_id] = SCALEF[entry_id];
                    STF[entry_id] = 0.0;
                }
            }
            if STF[entry_id] == 0.0 {
                SCALEE[entry_id] = 1.0 / shield;
                if SCALEE[entry_id] != SCALEF[entry_id] {
                    STE[entry_id] = 1.0;
                }
                if SCALEE[entry_id] == SCALEF[entry_id] {
                    STE[entry_id] = 0.0;
                }
            }
            if sv_information::is_ready_go() == false {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEF[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0;
                STF[entry_id] = 0.0;
                STE[entry_id] = 0.0;
            }
    
            if SCALEE[entry_id] > 1.0 
            || SCALEE[entry_id] < 1.0 {
                AttackModule::set_power_up(module_accessor, SCALEE[entry_id]);
                DamageModule::set_damage_mul(module_accessor, 1.0 / SCALEE[entry_id]);
                SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
                SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
            }
        }


        if fighter_kind == *FIGHTER_KIND_PZENIGAME {
            if sv_information::is_ready_go() == true {
                if STL[entry_id] == 1.0 {
                    if SCALEL[entry_id] == SCALEE[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEE[entry_id] = 1.0 / shield;
                        if SCALEE[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEE[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STL[entry_id] = 0.0;
                    }
                    if SCALEL[entry_id] != SCALEE[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEL[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEE[entry_id] = SCALEL[entry_id];
                        if SCALEE[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEE[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STL[entry_id] = 0.0;
                    }
                }
                if STL[entry_id] == 0.0 {
                    STE[entry_id] = 1.0;
                    SCALEE[entry_id] = 1.0 / shield;
                }
            }
            if sv_information::is_ready_go() == false {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEF[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0;
                SCALEL[entry_id] = 1.0;
                STF[entry_id] = 0.0;
                STE[entry_id] = 0.0;
                STL[entry_id] = 0.0;
            }
        }

        if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
            if sv_information::is_ready_go() == true {
                if STE[entry_id] == 1.0 {
                    if SCALEE[entry_id] == SCALEF[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEF[entry_id] = 1.0 / shield;
                        if SCALEF[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEF[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STE[entry_id] = 0.0;
                    }
                    if SCALEE[entry_id] != SCALEF[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEF[entry_id] = SCALEE[entry_id];
                        if SCALEF[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEF[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STE[entry_id] = 0.0;
                    }
                }
                if STE[entry_id] == 0.0 {
                    STF[entry_id] = 1.0;
                    SCALEF[entry_id] = 1.0 / shield;
                }   
            }
            if sv_information::is_ready_go() == false {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEE[entry_id] = 1.0;
                SCALEF[entry_id] = 1.0;
                SCALEL[entry_id] = 1.0;
                STE[entry_id] = 0.0;
                STF[entry_id] = 0.0;
                STL[entry_id] = 0.0;
            }
        }

        if fighter_kind == *FIGHTER_KIND_PLIZARDON {
            if sv_information::is_ready_go() == true {
                if STF[entry_id] == 1.0 {
                    if SCALEL[entry_id] == SCALEF[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEL[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEL[entry_id] = 1.0 / shield;
                        if SCALEL[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEL[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STF[entry_id] = 0.0;
                    }
                    if SCALEL[entry_id] != SCALEF[entry_id] {
                        ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                        SCALEL[entry_id] = SCALEF[entry_id];
                        if SCALEL[entry_id] != 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEL[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                        }
                        if SCALEL[entry_id] == 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_TERM, 1.0, *FIGHTER_SCALING_STATUS_NONE);
                        }
                        STF[entry_id] = 0.0;
                    }
                }
                if STF[entry_id] == 0.0 {
                    STL[entry_id] = 1.0;
                    SCALEL[entry_id] = 1.0 / shield;
                }
            }
            if sv_information::is_ready_go() == false {
                ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEF[entry_id] = 1.0;
                SCALEE[entry_id] = 1.0;
                SCALEL[entry_id] = 1.0;
                STF[entry_id] = 0.0;
                STE[entry_id] = 0.0;
                STL[entry_id] = 0.0;
            }
        }

    }
}


#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn peach_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let scale = smash::app::lua_bind::ModelModule::scale(fighter.module_accessor);

        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        
        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            //&& smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            //|| smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FURAFURA
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                /*test */
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                /*test */
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }
        /*
        if scale >= 0.5 {
            GrabModule::set_scale_2nd(module_accessor, scale);
            AttackModule::set_attack_scale(module_accessor, scale, true);
        }
        */

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        //|| if sv_information::is_ready_go() == false 
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_wear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_wear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_appeal_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("seq_peach_rnd_attack")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("seq_peach_rnd_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("seq_peach_rnd_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("seq_peach_rnd_futtobi01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackair_b01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackair_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackair_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackair_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackair_n01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackhard_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackhard_h02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_attackhard_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_attack01")}, 1.0); 
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_smash_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_smash_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_smash_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_smash_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_smash_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_jump04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_h02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_h03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_n01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_n02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_n03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_n04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_s04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_throw_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_throw_f02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_throw_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_throw_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_swing_s_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_wear02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_wear01_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_h01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_h03_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_special_s02_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_step_right_s_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_001")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_002")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_003")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_peach_009")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_peach_final01")}, 1.0);
            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 


        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            //&& smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
            //|| smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FURAFURA
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_002")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_003")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_009")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_daisy_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_hit_binta")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_hit_fryingpan")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_hit_golfclub")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_hit_tennisracket")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackdash02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackhard_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_smash_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_catch_kinopio")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_throw_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_throw_f02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_throw_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_throw_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_wear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_wear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_swing_s_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_landing01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_special_H03_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_daisy_jump01_win03")}, 1.0);            

        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosalina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_002")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_rosetta_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_step_lp_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_crawing_F")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_crawing_B")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N02_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N02_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N02_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final02_")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_wear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_wear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_appeal_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_galaxy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_m_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N02_s_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_special_N03_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_m_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_rosetta_swing_m_win03")}, 1.0);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]
fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 0.97;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attack08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_attackair_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_appeal_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_appeal_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_appeal_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_zelda_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_magic01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_magic10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_magic11")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackdash_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_attackdash_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L03_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L03_max")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L07_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L07_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L10_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L10_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_L10_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_throw_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_final03_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_win01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_ware01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_ware04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_catchloop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_zelda_cliffjump")}, 1.0);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 0.95;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_001")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_002")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_003")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_win_pit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_win_pitb")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_palutena_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_left_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_right_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_step_lp_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_swing_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_special_L06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_throw")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_jump02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appeal_S02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackair_N01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_landing01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_attackhard_H01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_palutena_appeal_S02_win03")}, 1.0);    
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 0.99;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_dragon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_final_dragon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_kamui_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_horn_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_horn_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_dragon_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_dragon_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackdash_landing")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_attackair_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_smash_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N05b")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_N07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_criticalhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S04b")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_special_S06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appeal_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_appear03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win02_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_win03b")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_kamui_final06")}, 1.0);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_001")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win03_chrom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_win_lucina")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_chrom09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_final_reflet_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_reflet_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackbomb")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_F02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_B02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N11")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_N12")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_mp_empty")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_throw")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_close")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_book_landing")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_s04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appeal_l04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win1")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win2_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_win3_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final10")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final12")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final14")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final15")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_final17")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_sword")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_fire")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_finalhit_elec")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_special_S02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_fire_02_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_reflet_smash_S01_win03")}, 1.0);   
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]
fn szerosuit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_crawing_left_F")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_crawing_right_F")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_crawing_left_H")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_crawing_right_H")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_escape_jet")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackair_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_final08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_entry_jet")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_01_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_04_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_special_S01_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_swing_m_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_appeal_L01_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_jump01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_landing01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_win2_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_jump02_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_landing02_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_wait2_04_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_swing_l_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_szerosuit_attackhard_L01_win03")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_001")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_002")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_003")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_004")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_005")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_szerosuit_knockout")}, 1.0);            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_left_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_step_right_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_jumpround")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_attackl_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_attackl_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_criticalhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S02H")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S02L")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S03H")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S03L")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S03S")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S04H")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S04L")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_special_S04S")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_final06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_hitground")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swordpullout")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swordin")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_appeal_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_win1")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_s_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_l_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_win1_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swing_l_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_appeal_H01_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_swordin_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_lucina_win3_win03")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_final")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_win_marth")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_win_ike")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_lucina_knockout")}, 1.0);            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 1.03;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_m04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_step_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_escape01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_escape02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_batwithin02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_swing_punch_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_swing_punch_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_swing_punch_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_swing_kick_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_wecked_chargefull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_loveisblue_spin")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_head")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_cartridge")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_cartridge_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_cartridge_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_shoot_cartridge_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackhard_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackhard_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_F02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_F03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_attackair_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_smash_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_smash_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N06_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N06_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N06_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N06_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N07_head01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N07_head02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N07_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N07_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_N08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_special_S05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_throw_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_throw_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_throw_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_L05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_L06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appeal_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_appear03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win01_06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win03_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win03_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win03_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_win03_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_hit01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_hit02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv1")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv2")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv3")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv4")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv5")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv6")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv7")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv8")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_bayonetta_final_actlv_comp")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_S07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damage03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damage03_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damagefly03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_throw_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_throw_F02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_special_L04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal03_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_win06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_win07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_win08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_win09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_knockout")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal01_soundtest")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_bayonetta_appeal03_soundtest")}, 1.0);            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 1.02;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_attack09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_n01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_n02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_n03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_n04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_h02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_h03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l06_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_special_l06_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_guest_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_guest_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_guest_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_appeal04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_master_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_ll_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_l_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_m_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_s_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_ll_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_l_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_m_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_s_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_ll01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_ll02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_ll03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_left_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_ll01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_ll02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_ll03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_step_right_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_m02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_m03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_s03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_ll01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_ll02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_smash_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_attackdash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_ll03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_jump02_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_cloth_stop_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_dash_stop_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_dash_turn_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_jump02_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_escapeN")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_escapeF")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_escapeB")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_escapeairN")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_swing_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_swing_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_swing_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attack100_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_hit_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackdash01_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackhard_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_N05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_attackair_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_L02_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_N06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H07_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_H07_kick_hit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_special_S05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_throw_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_throw_B02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_hit_axe")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_appear03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_master_win03_02")}, 1.0); 
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 1.076;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        let mag : f32 = 2.0; /*巨大化倍率*/ 

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH 
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            }
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALE[entry_id] = SCALE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }   
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALE[entry_id] = 1.0;
            CT[entry_id] = 0;
        }

        if SCALE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALE[entry_id] >= 0.95
        && SCALE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_escapef")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_cliffcatch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack100_01_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_attack100_02_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_special_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_punch_06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_knockout")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_silent_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_silent_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_special_h02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_tantan_final03")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_spring")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_spring_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_spring_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_spring_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_left_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_spring")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_left_ll_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_right_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_silent_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_landing02_ft")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_rise")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_escapen")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_escapeB")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_escapecliff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit01_punch_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit01_punch_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit01_punch_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit01_punch_ll")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit02_punch_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit02_punch_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit02_punch_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit03_punch_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit03_punch_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_hit03_punch_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_swing_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_swing_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_swing_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack100_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack100end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackdash02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackhard_L01_slide")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackhard_L01_gravel")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_N02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_N03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attackair_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_smash_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_catch01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_short")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_short_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_long")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_long_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_beam_ready")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_beam")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_beam_max")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_doragon_short")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack01_doragon_smash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_short")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_short_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_long")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_long_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_charge")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_smash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack02_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_short")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_short_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_long")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_long_pull")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_charge")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack03_smash")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack_hit_ground_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack_hit_ground_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_attack_hit_ground_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H01_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_H05_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_L01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_L01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_special_L01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final02_hit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_final06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_appear03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_tantan_win03")}, 1.0);            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 1.112;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        let mag : f32 = 2.0; /*巨大化倍率*/ 
        
        if STE[entry_id] == 1.0 {
            if SCALEE[entry_id] == SCALEF[entry_id] {
                ShieldModule::set_attack_mul(module_accessor, 1.0 / shield, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEF[entry_id] = 1.0 / shield;
                STE[entry_id] = 0.0;
            }
            if SCALEF[entry_id] != SCALEE[entry_id] {
                /*test */
                ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEE[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                /*test */
                SCALEF[entry_id] = SCALEE[entry_id];
                STE[entry_id] = 0.0;
            }
        }
        if STE[entry_id] == 0.0 {
            SCALEF[entry_id] = 1.0 / shield;
            if SCALEF[entry_id] != SCALEE[entry_id] {
                STF[entry_id] = 1.0;
            }
            if SCALEF[entry_id] == SCALEE[entry_id] {
                STF[entry_id] = 0.0;
            }
        }

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            /*test */
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            /*test */
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALEF[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALEF[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEF[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            } 
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALEF[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                /*test */
                SCALEF[entry_id] = SCALEF[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                /*test */
                CT[entry_id] = 0;
            }  
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEF[entry_id] = 1.0;
            SCALEE[entry_id] = 1.0;
            STF[entry_id] = 0.0;
            STE[entry_id] = 0.0;
            CT[entry_id] = 0;
        }

        if SCALEF[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALEF[entry_id] >= 1.0 
        || SCALEF[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALEF[entry_id] >= 0.95
        && SCALEF[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_attack09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_cliffcatch01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_cliffcatch_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_element_silent_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_n01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_n02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_n02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_n03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_s_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_h_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_l_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_smash_h_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s01_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s02_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s01_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s01_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_s01_rand03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01_rand03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01_rand04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h01_rand05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h04_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_h04_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_l04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_special_l05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_final04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_win04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_win03_mix")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_eflame_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_ll_loop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_escapen")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_swing_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_swing_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_swing_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attack100_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attack100_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackdash02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_throw_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_throw_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_throw_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_throw_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_smash_S01_SWISH")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_smash_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_NSTART")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_NHOLD")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N01_00")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N02_00")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N03_00")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N03_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_N03_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S03_00")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S03_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06_00")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S01_flick")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S02_flick")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06_hop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_special_S06_reflect")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final02_00_a")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final02_00_b")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final_diver_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final_diver_out")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final_hit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final_hit_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_final_hit_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_S03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appeal_S04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appear01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_appear03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win01_06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win02_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_eflame_win03")}, 1.0);            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        //let scalef = scale / 1.112;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        let mag : f32 = 2.0; /*巨大化倍率*/ 
        
        if STF[entry_id] == 1.0 {
            if SCALEF[entry_id] == SCALEE[entry_id] {
                ShieldModule::set_attack_mul(module_accessor, 1.0 / shield, *FIGHTER_SHIELD_KIND_GUARD);
                SCALEE[entry_id] = 1.0 / shield;
                STF[entry_id] = 0.0;
            }
            if SCALEE[entry_id] != SCALEF[entry_id] {
                ShieldModule::set_attack_mul(module_accessor, 1.0 / SCALEF[entry_id], *FIGHTER_SHIELD_KIND_GUARD);
                SCALEE[entry_id] = SCALEF[entry_id];
                STF[entry_id] = 0.0;
            }
        }
        if STF[entry_id] == 0.0 {
            SCALEE[entry_id] = 1.0 / shield;
            if SCALEE[entry_id] != SCALEF[entry_id] {
                STE[entry_id] = 1.0;
            }
            if SCALEE[entry_id] == SCALEF[entry_id] {
                STE[entry_id] = 0.0;
            }
        }

        if sv_information::is_ready_go() == true {
            //if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ENTRY 
            if smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
            {
                if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                    if smash::app::FighterUtil::is_scaling(module_accessor) == false {
                        if SCALEE[entry_id] > 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                        if SCALEE[entry_id] < 1.0 {
                            smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALEE[entry_id], *FIGHTER_SCALING_STATUS_BIG);    
                        }
                    }
                }
            } 
            if CT[entry_id] >= 1 {
                SoundModule::play_se(module_accessor, Hash40::new("se_item_mushroom"), true, true, true, true, smash::app::enSEType(0));
                ShieldModule::set_attack_mul(module_accessor, 1.0 / (SCALEE[entry_id] * mag), *FIGHTER_SHIELD_KIND_GUARD);
                SCALEE[entry_id] = SCALEE[entry_id] * mag;
                smash::app::sv_battle_object::fixed_scaling(fighter_id, true, *FIGHTER_SCALING_KIND_KINOKO, *FIGHTER_SCALING_TYPE_BIG, SCALE[entry_id], *FIGHTER_SCALING_STATUS_BIG);
                CT[entry_id] = 0;
            }  
        }
        if sv_information::is_ready_go() == false {
            ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            SCALEE[entry_id] = 1.0;
            SCALEF[entry_id] = 1.0;
            STE[entry_id] = 0.0;
            STF[entry_id] = 0.0;
            CT[entry_id] = 0;
        }

        if SCALEE[entry_id] > 1.0 {
            AttackModule::set_power_up(module_accessor, SCALE[entry_id]);
            DamageModule::set_damage_mul(module_accessor, 1.0 / SCALE[entry_id]);
        }

        if SCALEE[entry_id] >= 1.0 
        || SCALEE[entry_id] < 1.0 {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushroom")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_item_mushd")}, 1.0);
        }

        if SCALEE[entry_id] >= 0.95
        && SCALEE[entry_id] < 1.1
        {
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dizzy")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_cliff_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swing_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_smash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_blowaway_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_down_soil_ss")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_dragoon_ride")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sheildguard")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardoff")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_guardon")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_ladderstep_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_sleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_slip_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_falldown_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_step_jump")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_wallhit")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_high_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_middle_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swim_low_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimdrown")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_high")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_low")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_swimattack_middle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_common_throw_03")}, 1.0);

            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack06")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack07")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack08")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_attack09")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_ottotto")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_damage01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_damage02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_damagefly01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_damagefly02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_missfoot01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_missfoot02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_damage_twinkle")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_swimup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_furafura")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_furasleep")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_wakeup")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_heavyget")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_passive")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_cliffcatch01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_cliffcatch_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_escapeForesight01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_escapeForesight02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_escapeForesight03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_n01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_n02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_n02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_n03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_l04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_l05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_smash_s_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_smash_h_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_smash_l_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_smash_l_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s01_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s01_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_s04_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01_rand03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h01_rand04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h02_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h02_rand")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h03_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h04_rand01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h04_rand02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_special_h04_rand03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_final01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_final02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_appeal01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_appeal02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_appeal03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_win01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_win02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_win03_mix")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("vc_elight_knockout")}, 1.0);
            
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_ll_loop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_left_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_left_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_left_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_right_l")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_right_m")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_step_right_s")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_dash_start")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_dash_stop")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_dash_turn")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_jump01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_jump02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_jump03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_landing01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_landing02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_landing03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_escapen")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_escape")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_escapeair")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_swing_s01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_swing_m01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_swing_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attack100")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attack100_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attack100_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attack100_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attack100_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackdash01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackhard_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackhard_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackhard_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackhard_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_escapeForesight01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_escapeForesight02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackair_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackair_F01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackair_B01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackair_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_attackair_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_catch")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_h01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_f01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_l01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_l02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_throw_l03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_smash_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_smash_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_smash_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_smash_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N01_hold")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N02_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N03_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N03_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N03_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N03_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_N03_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H02_01_shot")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H03_shot")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_S02_end")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_special_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_element_final01_Diver")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final02_04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final03_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final04_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_final05")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_H01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_H02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_H03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_H04")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_L01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_L02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_L03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_S01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appeal_S02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_appear02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win01_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win01_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win01_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win02_01")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win02_02")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win02_03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_elight_win03")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_element_squat")}, 1.0);
            SoundModule::set_se_pitch_ratio(module_accessor, smash::phx::Hash40{hash: hash40("se_element_rise")}, 1.0);            
        }
    }
}



pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
        
    );
    smashline::install_agent_frames!(
        
        peach_frame,
        daisy_frame,
        zelda_frame,
        rosalina_frame,
        palutena_frame,
        kamui_frame,
        reflet_frame,
        szerosuit_frame,
        lucina_frame,
        bayonetta_frame,
        master_frame,
        tantan_frame,
        eflame_frame,
        elight_frame
        
    );
    smashline::install_acmd_scripts!(
        
    );
}
