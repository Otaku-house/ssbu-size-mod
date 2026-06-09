use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;




#[acmd_script( agent = "reflet", script = "game_speciallwstart", category = ACMD_SOUND, low_priority )]
unsafe fn game_speciallwstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    wait(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) == true {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 307.0, 0.0, 10.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 304.2, 0.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(11.2), *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_A);
        }
    }
}

#[acmd_script( agent = "reflet", script = "game_specialairlwstart", category = ACMD_SOUND, low_priority )]
unsafe fn game_specialairlwstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    wait(agent.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) == true {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("top"), 307.0, 0.0, 10.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 1, Hash40::new("top"), 304.2, 0.0, 10.0, 14.0, Some(0.0), Some(10.0), Some(11.2), *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_A);
        }
    }
}

#[acmd_script( agent = "reflet", script = "game_attacklw3", category = ACMD_SOUND, low_priority )]
unsafe fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 4.0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 0.0, 5.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 30, 40, 0, 40, 4.5, 0.0, 3.0, 11.0, Some(0.0), Some(4.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        //AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 5.0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "reflet", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 308.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 301.9, 0.0, 8.0, 2.1, Some(0.0), Some(8.0), Some(11.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

#[acmd_script( agent = "reflet", script = "game_specialnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 1.5, 30, 70, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 3.0, 30, 70, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd_script( agent = "reflet", script = "game_specialairnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnshoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 1.5, 30, 70, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 3.0, 30, 70, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

/*
#[acmd_script( agent = "reflet", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 13.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 70, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 12.0, 40, 80, 0, 55, 4.2, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 8.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) == true {
            if macros::is_excute(agent) {
                VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        battle_object();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 7.2, 361, 80, 0, 55, 3.7, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword"), 6.0, 361, 80, 0, 45, 2.5, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    else {
        /*
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
        methodlib::L2CValue::as_pointer()const(-1, 0);
        throwaway_sword();*/
        throwaway_sword(battle_object(agent.module_accessor), &Vector2f{x: -8.0, 7: 14.0}, true);
        //FighterSpecializer_Reflet::throwaway_sword(fighter.global_table[0x4].get_ptr() as *mut Fighter, Vector2f{x: 1.5, y: 22.0}, true);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}*/

/*
#[acmd_script( agent = "reflet", script = "game_", category = ACMD_GAME, low_priority )]
unsafe fn game_(agent: &mut L2CAgentBase) {
    
}

#[acmd_script( agent = "reflet", script = "game_", category = ACMD_GAME, low_priority )]
unsafe fn game_(agent: &mut L2CAgentBase) {
    
}

#[acmd_script( agent = "reflet", script = "game_", category = ACMD_GAME, low_priority )]
unsafe fn game_(agent: &mut L2CAgentBase) {
    
}
*/


pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        game_attacklw3,
        //game_specialairlwstart,
        //game_speciallwstart,
        game_catch,
        game_specialnshoot,
        game_specialairnshoot
    );
}
