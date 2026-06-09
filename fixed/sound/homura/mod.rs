use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;

        // bust attack

#[acmd_script( agent = "eflame", script = "game_appealsl", category = ACMD_GAME )]
unsafe fn game_appealsl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 2.6, 0.4, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 1.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }  
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }  
}

#[acmd_script( agent = "eflame", script = "game_appealsr", category = ACMD_GAME )]
unsafe fn game_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 2.6, 0.4, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 1.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }  
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }  
}

#[acmd_script( agent = "eflame", script = "game_appeallwr", category = ACMD_GAME )]
unsafe fn game_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 2.6, 0.4, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        //macros::ATTACK(agent, 1, 0, Hash40::new("waist"), 4.0, 140, 115, 0, 98, 1.0, 1.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }  
    wait(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }  
}

#[acmd_script( agent = "eflame", script = "sound_appeallwr", category = ACMD_SOUND )]
unsafe fn sound_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_eflame_ottotto"));
    }
}

            // wait motion appeal

#[acmd_script( agent = "eflame", script = "sound_appeallwr", category = ACMD_SOUND )]
unsafe fn sound_appeallwr_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_step_right_m"));
    }
    frame(agent.lua_state_agent, 115.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_step_right_m"));
    } 
    frame(agent.lua_state_agent, 136.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_eflame_step_right_m"));
    }
}

#[acmd_script( agent = "eflame", script = "effect_appeallwr", category = ACMD_EFFECT )]
unsafe fn effect_appeallwr_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        
    }
}


#[acmd_script( agent = "eflame", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn homura_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)    
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn homura_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)    
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_squat", category = ACMD_GAME, low_priority )]
unsafe fn homura_squat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("bust"), Damage=1.5, Angle=361, KBG=130, FKB=0, BKB=90, Size=1.3, X=0.0, Y=1.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn homura_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=6)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=8)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=9)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                //ArticleModule::add_motion_partial()
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-1.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-2.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-3.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("bust"), Damage=10.5, Angle=270, KBG=130, FKB=0, BKB=90, Size=1.3, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=-2.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-7.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=2.4, X=0.0, Y=-13.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=2.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=3.0, X=0.0, Y=1.0, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=-2.0, Z=-13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=-5.0, Z=-14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=-8.0, Z=-15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=3.0, X=0.0, Y=4.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=3.0, X=0.0, Y=6.0, Z=-13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=5.0, Z=-16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=4.0, Z=-19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=2.4, X=0.0, Y=3.0, Z=-22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=85, FKB=0, BKB=65, Size=3.0, X=0.0, Y=8.0, Z=-8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=26)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
                //ArticleModule::add_motion_partial()
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=33)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}



#[acmd_script( agent = "eflame", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn homura_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::set_int64(hash40("special_hi") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=1)
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_EFLAME_GENERATE_ARTICLE_FIREPILLAR, true, *FIGHTER_EFLAME_GENERATE_ARTICLE_FIREPILLAR)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=55, KBG=100, FKB=0, BKB=60, Size=7.5, X=0.0, Y=4.0, Z=-1.0, X2=0.0, Y2=4.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=70, KBG=100, FKB=0, BKB=65, Size=7.0, X=0.0, Y=-4.0, Z=6.0, X2=0.0, Y2=-8.0, Z2=6.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
        }
        frame(Frame=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
        frame(Frame=60)
//      IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn homura_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        if(is_excute){
            WorkModule::set_int64(hash40("attack_lw4_hold") as i64, FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        FT_MOTION_RATE(FSM=3.0)
        frame(Frame=4)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=74, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=35, KBG=71, FKB=0, BKB=75, Size=4.0, X=0.0, Y=7.0, Z=16.0, X2=0.0, Y2=7.0, Z2=10.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=35, KBG=66, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=35, KBG=66, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-12.0, X2=0.0, Y2=7.0, Z2=-10.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=74, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=35, KBG=71, FKB=0, BKB=75, Size=4.0, X=0.0, Y=7.0, Z=-16.0, X2=0.0, Y2=7.0, Z2=-10.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=35, KBG=66, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=35, KBG=66, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=10.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=27)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=46)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_specialn4", category = ACMD_GAME, low_priority )]
unsafe fn homura_specialn4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::set_int64(hash40("special_n4") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        if(is_excute){
            AttackModule::disable_tip()
        }
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("bust"), Damage=10.5, Angle=361, KBG=130, FKB=0, BKB=80, Size=1.3, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=10.5, Angle=361, KBG=100, FKB=0, BKB=80, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=6)
        if(WorkModule::get_int(module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) == 3){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=50, KBG=100, FKB=0, BKB=80, Size=6.5, X=0.0, Y=5.5, Z=13.0, X2=0.0, Y2=5.5, Z2=-13.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
                //WorkModule::is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_ON_SCALING)
                //ItemModule::have_item(smash::app::ItemKind(*ITEM_KIND_MUSHROOM),0,0,true,true)
                //ItemModule::drop_item(0.0,-1.0,0)
                CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
            }
        }
        else{            
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=50, KBG=100, FKB=0, BKB=75, Size=6.5, X=0.0, Y=5.5, Z=13.0, X2=0.0, Y2=5.5, Z2=-13.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
                CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
            }        
        }
        
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_specialairn4", category = ACMD_GAME, low_priority )]
unsafe fn homura_specialairn4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::set_int64(hash40("special_n4") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        if(is_excute){
            AttackModule::disable_tip()
        }
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("bust"), Damage=10.5, Angle=361, KBG=130, FKB=0, BKB=80, Size=1.3, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=10.5, Angle=361, KBG=100, FKB=0, BKB=80, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=6)
        if(WorkModule::get_int(module_accessor, *FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_INT_ROTATE_NUM) == 3){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=50, KBG=100, FKB=0, BKB=80, Size=6.5, X=0.0, Y=5.5, Z=13.0, X2=0.0, Y2=5.5, Z2=-13.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
                //WorkModule::is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_ON_SCALING)
                //ItemModule::have_item(smash::app::ItemKind(*ITEM_KIND_MUSHROOM),0,0,true,true)
                //ItemModule::drop_item(0.0,-1.0,0)
            }
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_EFLAME_STATUS_SPECIAL_N_WORK_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=21)
//      IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn homura_ssmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        if(is_excute){
            WorkModule::set_int64(hash40("attack_s4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=12)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=76, FKB=0, BKB=75, Size=4.5, X=0.0, Y=4.0, Z=3.5, X2=0.0, Y2=4.0, Z2=4.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.5, X=0.0, Y=12.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.0, X=0.0, Y=17.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.5, X=0.0, Y=19.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.0, X=0.0, Y=19.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.0, X=0.0, Y=8.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            //methodlib::L2CValue::as_hash()const(0, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(0, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(1, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(1, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(2, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(2, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(3, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(3, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(4, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(4, Hash40::new("eflame_attack_s4s_explosion"))
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear(5, false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.5, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.0, X=0.0, Y=9.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.5, X=0.0, Y=11.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.0, X=0.0, Y=13.0, Z=23.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.0, X=0.0, Y=5.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            //methodlib::L2CValue::as_hash()const(0, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(0, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(1, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(1, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(2, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(2, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(3, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(3, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(4, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(4, Hash40::new("eflame_attack_s4s_explosion"))
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=5.0, X=0.0, Y=4.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=4.5, X=0.0, Y=4.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.5, X=0.0, Y=2.0, Z=25.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=2.0, X=0.0, Y=7.0, Z=27.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=76, FKB=0, BKB=70, Size=3.0, X=0.0, Y=2.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            //methodlib::L2CValue::as_hash()const(0, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(0, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(1, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(1, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(2, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(2, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(3, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(3, Hash40::new("eflame_attack_s4s_explosion"))
            //methodlib::L2CValue::as_hash()const(4, hash40("eflame_attack_s4s_explosion"))
            AttackModule::set_optional_hit_effect(4, Hash40::new("eflame_attack_s4s_explosion"))
            WorkModule::on_flag(Flag=FIGHTER_EFLAME_STATUS_ATTACK_FLAG_S4_GROUND_CHECK)
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)) {
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if (MotionModule::is_changing(module_accessor)) {
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn homura_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        if(is_excute){
            WorkModule::set_int64(hash40("attack_hi4_hold") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=8)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=10)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=85, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=10.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=85, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=12.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=115, KBG=100, FKB=0, BKB=85, Size=8.0, X=0.0, Y=8.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=125, KBG=100, FKB=0, BKB=85, Size=8.0, X=0.0, Y=12.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=15)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=83, KBG=80, FKB=0, BKB=85, Size=5.0, X=0.0, Y=22.0, Z=3.0, X2=0.0, Y2=22.0, Z2=11.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=83, KBG=80, FKB=0, BKB=85, Size=5.0, X=0.0, Y=24.0, Z=0.0, X2=0.0, Y2=24.0, Z2=8.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=83, KBG=80, FKB=0, BKB=85, Size=5.0, X=0.0, Y=24.0, Z=-3.0, X2=0.0, Y2=24.0, Z2=8.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=83, KBG=80, FKB=0, BKB=85, Size=5.0, X=0.0, Y=24.0, Z=-8.0, X2=0.0, Y2=24.0, Z2=8.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=83, KBG=80, FKB=0, BKB=85, Size=5.0, X=0.0, Y=24.0, Z=-11.0, X2=0.0, Y2=24.0, Z2=8.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=83, KBG=75, FKB=0, BKB=80, Size=5.0, X=0.0, Y=24.0, Z=-11.0, X2=0.0, Y2=24.0, Z2=8.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=32)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=38)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=40)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=75, FKB=0, BKB=80, Size=4.0, X=0.0, Y=3.0, Z=-1.0, X2=0.0, Y2=3.0, Z2=1.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=42)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=75, FKB=0, BKB=80, Size=4.0, X=0.0, Y=3.0, Z=-3.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=44)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=50)
        FT_MOTION_RATE(FSM=1.1)
        frame(Frame=60)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_script( agent = "eflame", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn homura_stilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=5)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=4.0, X=0.0, Y=3.0, Z=-2.0, X2=0.0, Y2=3.0, Z2=2.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(0, 10)
        if(is_excute){
            AttackModule::clear(5, false)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.0, X=-1.5, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.0, X=-1.5, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=2.5, X=-1.5, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=2.0, X=-1.5, Y=16.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.0, X=0.0, Y=10.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.5)
            ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=2, ID2=3, ShieldstunMul=1.5)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.5, X=2.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.5, X=2.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.0, X=2.0, Y=13.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=2.5, X=2.0, Y=17.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=3.0, X=0.0, Y=5.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.5)
            ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=2, ID2=3, ShieldstunMul=1.5)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=26)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn homura_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=3)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(0, 7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=87, KBG=62, FKB=0, BKB=82, Size=5.0, X=0.0, Y=4.0, Z=7.0, X2=0.0, Y2=4.0, Z2=0.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=87, KBG=62, FKB=0, BKB=82, Size=3.5, X=0.0, Y=3.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=93, KBG=62, FKB=0, BKB=82, Size=3.0, X=0.0, Y=2.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=15)
        //IS_EXIST_ARTICLE(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(fighter.module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        FT_MOTION_RATE(FSM=0.75)
        //FT_MOTION_RATE(0, 0.75)
        frame(Frame=27)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_script( agent = "eflame", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn homura_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=20, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=20, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=15, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=15, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=20, FKB=0, BKB=15, Size=4.5, X=0.0, Y=4.0, Z=8.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=7.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=7.0, Unk=false)
        }
        frame(Frame=6)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
        frame(Frame=11)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn homura_turn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=65, Size=1.5, X=0.0, Y=15.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)    
        }
        wait(Frame=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "eflame", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn homura_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=2)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=5)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=6)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=300.3, X=0.0, Y=8.5, Z=4.0, X2=0.0, Y2=8.5, Z2=8.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=100.65, X=0.0, Y=8.5, Z=2.35, X2=0.0, Y2=8.5, Z2=10.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        //methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()()
        wait(Frames=2)
        if(is_excute){
            //grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            GrabModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}


// sound script

#[acmd_script( agent = "eflame", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
	        PLAY_SE(hash40("se_eflame_swing_s01"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attack12", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_swing_m01"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attack13", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_swing_l01"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack01"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=56)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attack100start", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattack100start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_swing_04"))
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_attack100end", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_attack100_end"))
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack01"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attackdash", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_attackdash01"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_attackdash02"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack02"))
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=52)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_attackhard_h01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack02"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=62)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_smash_h01"))
        }
        frame(Frame=11)
        if(is_excute){
            //PLAY_SE(0x165bd33078)
            PLAY_SE(hash40("vc_eflame_smash_h_rand"))
        }
        frame(Frame=34)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=70)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_smash_l01"))
        }
        frame(Frame=8)
        if(is_excute){
            //PLAY_SE(0x16c042726e)
            PLAY_SE(hash40("vc_eflame_smash_l_rand"))
        }
        frame(Frame=21)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=63)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_attackhard_s01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack03"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundattacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=12)
        if(is_excute){
            //PLAY_SE(0x1632c2c220)
            PLAY_SE(hash40("vc_eflame_smash_s_rand"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_smash_s01_swish"))
        }
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_smash_s01"))
        }
        frame(Frame=67)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=96)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_element_squat"))
        }
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_specialhi", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundspecialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_h03"))
        }
        frame(Frame=61)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_specials1start", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundspecials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_s01"))
        }
        frame(Frame=12)
        if(is_excute){
            //PLAY_SE(0x1a5b3e1f92)
            PLAY_SE(hash40("vc_eflame_special_s01_rand"))
        }
        frame(Frame=59)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=70)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        
    });
}

#[acmd_script( agent = "eflame", script = "sound_specialn", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundspecialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_STATUS(hash40("se_eflame_special_n01_00"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_STATUS(hash40("se_eflame_special_n01_01"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n01_02"))
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=83)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_specialn2", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundspecialn2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n02_00"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n02_01"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n02_02"))
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=94)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_specialn3", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundspecialn3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n03_00"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n03_01"))
        }
        frame(Frame=38)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_special_n03_02"))
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=106)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_throwf", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
	        PLAY_SE(hash40("se_eflame_swing_s01"))
        }
        frame(Frame=8)
        if(is_excute){
	        PLAY_SE(hash40("se_eflame_throw_f01"))
            PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack01"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_throwb", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundthrowb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
	        PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=14)
        if(is_excute){
	        PLAY_SE(hash40("se_common_punch_kick_swing_l"))
	        PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack01"))
        }
        frame(Frame=39)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

#[acmd_script( agent = "eflame", script = "sound_throwhi", category = ACMD_SOUND , low_priority )]
unsafe fn homura_soundthrowhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
	        PLAY_SE(hash40("se_eflame_throw_h01"))
	        PLAY_SEQUENCE(hash40("seq_eflame_rnd_attack01"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
        frame(Frame=56)
        if(is_excute){
            PLAY_SE(hash40("se_eflame_step_right_s"))
        }
    });
}

pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        /*
        game_appealsl,
        game_appealsr,
        game_appeallwr,
        sound_appeallwr*/
        sound_appeallwr_wait,
        effect_appeallwr_wait
        /*
        //homura_speciallw,
        //homura_specialairlw
        homura_attack11,
        homura_dtilt,
        //homura_stilt,
        homura_dsmash,
        //homura_ssmash,
        //homura_usmash,
        //homura_specialhi,
        homura_specialn4,
        homura_specialairn4,
        //homura_turn,
        homura_catch*/
        /*
        homura_soundattack100end,
        homura_soundattack100start,
        homura_soundattack11,
        homura_soundattack12,
        homura_soundattack13,
        homura_soundattackdash,
        homura_soundattackhi3,
        homura_soundattackhi4,
        homura_soundattacklw4,
        homura_soundattacks3,
        homura_soundattacks4,
        homura_soundspecialhi,
        homura_soundspecialn,
        homura_soundspecialn2,
        homura_soundspecialn3,
        homura_soundspecials,
        homura_soundsquat,
        homura_soundthrowb,
        homura_soundthrowf,
        homura_soundthrowhi
        */
        //homura_squat,
        //homura_attackairlw
    );
}
