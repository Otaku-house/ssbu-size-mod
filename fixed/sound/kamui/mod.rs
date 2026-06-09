use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
//use crate::kamui::sv_animcmd::get_value_float;
use smash::app::sv_animcmd::*;


            // specialS

#[acmd_script( agent = "kamui", script = "effect_specialswallattackf", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialswallattackf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 22, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_tyousoutotu_wind"), Hash40::new("top"), 0, 0, -6, 0, 0, 0, 0.45, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_tyousoutotu_atk"), Hash40::new("top"), 0, 1.5, 15.5, 0, 0, 90, 1, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("kamui_tyousoutotu_wind"), false, false);
    }
}

#[acmd_script( agent = "kamui", script = "effect_specialswallattackb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialswallattackb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 3, 14, 0, 12, 210, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_tyousoutotu_wind"), Hash40::new("top"), 0, 0, 5, 0, 180, 0, 0.45, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_tyousoutotu_atk"), Hash40::new("top"), 0, 1, -12, 0, 180, 90, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("kamui_tyousoutotu_wind"), false, false);
    }
}


#[acmd_script( agent = "kamui", script = "game_attack13", category = ACMD_GAME, low_priority )]
unsafe fn kamui_attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=150, FKB=0, BKB=50, Size=4.0, X=0.0, Y=3.0, Z=1.0, X2=0.0, Y2=3.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=150, FKB=0, BKB=50, Size=3.0, X=0.0, Y=3.0, Z=2.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=50, KBG=160, FKB=0, BKB=50, Size=5.0, X=0.0, Y=9.0, Z=11.5, X2=0.0, Y2=9.0, Z2=11.0, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=150, FKB=0, BKB=50, Size=4.2, X=0.0, Y=9.0, Z=22.0, X2=0.0, Y2=9.0, Z2=10.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=4.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=8.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=5.0, X=0.0, Y=12.0, Z=4.0, X2=0.0, Y2=12.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            //ATTACK(ID=4, Part=0, Bone=hash40("legl"), Damage=5.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //ATTACK(ID=5, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //ATTACK(ID=6, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)

            //        AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=22.0, Z=-2.0, X2=0.0, Y2=22.0, Z2=2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    //    AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        wait(Frames=8)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=5.0, Angle=310, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_stilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.5, Angle=45, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=3.0, Z=8.0, X2=0.0, Y2=3.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=10.5, Angle=45, KBG=100, FKB=0, BKB=30, Size=5.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=10.5, Angle=45, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.5, Angle=45, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=8.0, Z=9.0, X2=0.0, Y2=8.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)    
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.87)
    });
}

#[acmd_script( agent = "kamui", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn kamui_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=5.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=4.0, X=-1.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=4.0, X=-1.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.5, Angle=45, KBG=100, FKB=0, BKB=30, Size=3.0, X=0.0, Y=3.0, Z=5.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn kamui_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=13)
        if( ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) == true ){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=32, KBG=95, FKB=0, BKB=40, Size=3.0, X=0.0, Y=8.7, Z=17.3, X2=0.0, Y2=8.7, Z2=4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=95, FKB=0, BKB=50, Size=3.0, X=0.0, Y=9.0, Z=-13.0, X2=0.0, Y2=9.0, Z2=-6.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=35, KBG=85, FKB=0, BKB=50, Size=2.5, X=0.0, Y=9.0, Z=-22.0, X2=0.0, Y2=9.0, Z2=-15.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
                CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
            }
        }
        else{
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=32, KBG=95, FKB=0, BKB=40, Size=3.0, X=0.0, Y=8.7, Z=17.3, X2=0.0, Y2=8.7, Z2=4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=95, FKB=0, BKB=50, Size=3.0, X=0.0, Y=9.0, Z=-13.0, X2=0.0, Y2=9.0, Z2=-6.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=35, KBG=85, FKB=0, BKB=50, Size=2.5, X=0.0, Y=9.0, Z=-22.0, X2=0.0, Y2=9.0, Z2=-15.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_death"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)            
                CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
            }
        }
        if(is_excute){
                    }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=33)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_attacks4hi", category = ACMD_GAME, low_priority )]
unsafe fn kamui_ssmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        //execute(9)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        
        if(WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == true ){
            if(is_excute){
                ArticleModule::generate_article(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1)
                //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("attack_s4_s"))
                ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0)
                ArticleModule::set_frame(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 8.0)
            }
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=35)
        FT_MOTION_RATE(FSM=1.2)
    });
}

#[acmd_script( agent = "kamui", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn kamui_ssmashs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        //execute(9)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        
        if(WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == true ){
            if(is_excute){
                ArticleModule::generate_article(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1)
                //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("attack_s4_s"))
                ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0)
                ArticleModule::set_frame(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 8.0)
            }
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=35)
        FT_MOTION_RATE(FSM=1.2)
    });
}

#[acmd_script( agent = "kamui", script = "game_attacks4lw", category = ACMD_GAME, low_priority )]
unsafe fn kamui_ssmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        //execute(9)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        
        if(WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK)  == true ){
            if(is_excute){
                ArticleModule::generate_article(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1)
                //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("attack_s4_s"))
                ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("attack_s4_s"), false, -1.0)
                ArticleModule::set_frame(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, 8.0)
            }
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=35)
        FT_MOTION_RATE(FSM=1.2)
    });
}

#[acmd_script( agent = "kamui", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn kamui_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=-3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
        }
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=93, KBG=86, FKB=0, BKB=50, Size=2.0, X=0.0, Y=8.0, Z=3.0, X2=0.0, Y2=19.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=93, KBG=86, FKB=0, BKB=50, Size=2.0, X=0.0, Y=8.0, Z=-2.2, X2=0.0, Y2=19.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=15.0, Angle=93, KBG=91, FKB=0, BKB=50, Size=3.3, X=0.0, Y=30.0, Z=0.0, X2=0.0, Y2=26.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=95, KBG=75, FKB=0, BKB=75, Size=2.8, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=-8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "kamui_dragonhand", script = "game_specialnmax", category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialnmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            MotionModule::set_rate(FSR=1.4)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=102, FKB=0, BKB=50, Size=40.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)            
        }
        frame(Frame=7)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=8)
        if(is_excute){
            MotionModule::set_rate(FSR=1.0)
            QUAKE(CAMERA_QUAKE_KIND_M)
            //CameraModule::req_quake(*CAMERA_QUAKE_KIND_M)
        }
        
        if(WorkModule::is_flag(module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) == true ){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=100, FKB=0, BKB=28, Size=7.0, X=0.0, Y=9.0, Z=22.0, X2=0.0, Y2=9.0, Z2=14.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
            }
            else{
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=100, FKB=0, BKB=28, Size=7.0, X=7.0, Y=12.0, Z=22.0, X2=0.0, Y2=12.0, Z2=14.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)

                //if(sv_animcmd::get_value_float((*SO_VAR_FLOAT_LR).try_into().unwrap(), 0)){
                //    if(is_excute){
                //        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=100, FKB=0, BKB=28, Size=7.0, X=0.0, Y=8.9, Z=22.0, X2=0.0, Y2=8.9, Z2=14.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
                //    }
                //    else{
                //        if(is_excute){
                //            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=50, KBG=100, FKB=0, BKB=28, Size=7.0, X=7.0, Y=12.0, Z=22.0, X2=0.0, Y2=12.0, Z2=14.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BITE)
                //        }
                //    }
                //}
            }
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_specialsjump", category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialsjump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1)
            //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("special_s_jump"))
            ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_jump"), false, -1.0)
        }
        frame(Frame=5)
        if(is_excute){
            if( ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) == true ){
                if(is_excute){
                    MotionModule::change_motion(Hash40::new("special_s_wall_attack_b"), 1.0, 1.0, false, 1.0, false, false)
                }
            }
            if( ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == true ){
                if(is_excute){
                    MotionModule::change_motion(Hash40::new("special_s_wall_attack_f"), 1.0, 1.0, false, 1.0, false, false)
                }
            }
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_specialswallattackb", category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialsattackb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("special_s_wall_attack_b"))
            ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_attack_b"), false, -1.0)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=48, KBG=100, FKB=0, BKB=63, Size=4.0, X=0.0, Y=1.7, Z=12.5, X2=0.0, Y2=1.7, Z2=24.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::set_target_category(ID=0, Hitbits=COLLISION_CATEGORY_MASK_NO_IF)
            AttackModule::set_size(ID=0, Size=0.1)
            //attack(MA_MSC_CMD_ATTACK_NODE, 0, hash40("top"), 0, 2, 0)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=55, KBG=98, FKB=0, BKB=70, Size=9.3, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=13)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM)
            WorkModule::on_flag(Flag=FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_WALL_ATTACK_B_REVERSE_LR)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=55, KBG=98, FKB=0, BKB=70, Size=9.3, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=55, KBG=90, FKB=0, BKB=70, Size=4.5, X=0.0, Y=3.4, Z=-4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=34)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL)
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_specialswallattackf", category = ACMD_GAME, low_priority )]
unsafe fn kamui_specialsattackf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            //methodlib::L2CValue::as_hash()const(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, hash40("special_s_wall_attack_b"))
            ArticleModule::change_motion(*FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_attack_f"), false, -1.0)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=55, KBG=98, FKB=0, BKB=70, Size=9.3, X=0.0, Y=2.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=55, KBG=98, FKB=0, BKB=70, Size=9.3, X=0.0, Y=2.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            WorkModule::on_flag(Flag=FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM)
        }
        wait(Frames=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=55, KBG=90, FKB=0, BKB=70, Size=4.5, X=0.0, Y=3.7, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=25)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL)
        }
    });
}

#[acmd_script( agent = "kamui", script = "game_catch", category = ACMD_GAME )]
unsafe fn kamui_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=7)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=300.1, X=0.0, Y=7.5, Z=4.0, X2=0.0, Y2=7.5, Z2=8.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=100.55, X=0.0, Y=7.5, Z=2.45, X2=0.0, Y2=7.5, Z2=10.25, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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

#[acmd_script( agent = "kamui", script = "sound_appealhil", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal01"))
        }
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_squat"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_h01"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_h02"))
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_appealhir", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal01"))
        }
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_squat"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_h01"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_h02"))
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_appeallwl", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappeallwl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal03"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l02"))
        }
        frame(Frame=21)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l03"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l01"))
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_appeallwr", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappeallwr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal03"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l02"))
        }
        frame(Frame=21)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l03"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_l01"))
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_appealsl", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal02"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s02"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s03"))
        }
        frame(Frame=36)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s04"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s01"))
        }
        frame(Frame=64)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_appealsr", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundappealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("vc_kamui_appeal02"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s02"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s03"))
        }
        frame(Frame=36)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s04"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_appeal_s01"))
        }
        frame(Frame=64)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attack11", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_swing_s"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=37)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attack12", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_swing_m"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attack13", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundattack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_swing_l"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attackhi3", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundutilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_attackhard_h01"))
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_hard"))
        }
        frame(Frame=38)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_squat", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_squat"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attacks3", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundstilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_attackhard_s01"))
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_hard"))
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=41)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attackhi4", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundusmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=11)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_02"))
        }
        wait(Frames=1)
        
        if(WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_smash_h"))
            }
            else{
                if(is_excute){
                    PLAY_SE(hash40("vc_kamui_attack06"))
                }
            }
        }
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_h01"))
        }
        wait(Frames=8)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_h02"))
        }
        frame(Frame=34)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=52)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=60)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attacks4hi", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundssmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s04"))
            //sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
            SoundModule::stop_status_se()
        }
        wait(Frames=4)
        if(WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_smash_s"))
            }
            else{
                if(is_excute){
                    PLAY_SE(hash40("vc_kamui_attack07"))
                }
            }
        }
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s01"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        wait(Frames=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s02"))
        }
        frame(Frame=54)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attacks4", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundssmashs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s04"))
            //sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
            SoundModule::stop_status_se()
        }
        wait(Frames=4)
        if(WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_smash_s"))
            }
            else{
                if(is_excute){
                    PLAY_SE(hash40("vc_kamui_attack07"))
                }
            }
        }
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s01"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        wait(Frames=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s02"))
        }
        frame(Frame=54)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attacks4lw", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundssmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s04"))
            //sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
            SoundModule::stop_status_se()
        }
        wait(Frames=4)
        if(WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_smash_s"))
            }
            else{
                if(is_excute){
                    PLAY_SE(hash40("vc_kamui_attack07"))
                }
            }
        }
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s01"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        wait(Frames=10)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s02"))
        }
        frame(Frame=54)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_attacklw4", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_sounddsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=10)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_02"))
        }
        wait(Frames=1)
        
        if(WorkModule::get_float(module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) == 1.0 ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_smash_l"))
            }
            else{
                if(is_excute){
                    PLAY_SE(hash40("vc_kamui_attack05"))
                }
            }
        }
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_l01"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_l02"))
        }
        frame(Frame=35)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=55)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=64)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_throwlw", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundthrowlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frames=2)
        if(is_excute){
            //PLAY_SE(hash40("se_kamui_dragon_start"))
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack_hard"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=8)
        if(is_excute){
            //PLAY_SE(hash40("vc_kamui_dragon"))
        }
        frame(Frame=27)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_03"))
        }
        frame(Frame=41)
        if(is_excute){
            //PLAY_SE(hash40("se_kamui_dragon_end"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_throwf", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        frame(Frame=9)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s01"))
            PLAY_SE(hash40("se_common_throw_02"))
        }
        frame(Frame=14)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
        frame(Frame=27)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s02"))
        }
        frame(Frame=43)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
        frame(Frame=53)
        if(is_excute){
            PLAY_STEP(hash40("se_kamui_step_left_s"))
        }
    });
}

#[acmd_script( agent = "kamui", script = "sound_throwb", category = ACMD_SOUND, low_priority )]
unsafe fn kamui_soundthrowb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
            PLAY_SE(hash40("se_kamui_horn_start"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s01"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_kamui_rnd_attack"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_02"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_smash_s02"))
        }
        frame(Frame=28)
        if(is_excute){
            PLAY_SE(hash40("se_kamui_horn_end"))
        }
    });
}



#[acmd_script( agent = "kamui", script = "expression_appealhil", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_appealhil(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
    }
}

#[acmd_script( agent = "kamui", script = "expression_appealhir", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_appealhir(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
    }
}

#[acmd_script( agent = "kamui", script = "effect_appealhil", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "kamui", script = "effect_appealhir", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "kamui", script = "sound_appealhil", category = ACMD_SOUND, low_priority )]
unsafe fn sound_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_kamui_appeal_h01"));
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kamui_damagefly01"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_kamui_appeal_h02"));
    }
    frame(agent.lua_state_agent, 95.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_step_right_m"));
    }
    frame(agent.lua_state_agent, 106.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_step_right_s"));
    }
}

#[acmd_script( agent = "kamui", script = "sound_appealhir", category = ACMD_SOUND, low_priority )]
unsafe fn sound_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_kamui_appeal_h01"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kamui_damagefly02"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_kamui_appeal_h02"));
    }
    frame(agent.lua_state_agent, 95.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_step_right_m"));
    }
    frame(agent.lua_state_agent, 106.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_step_right_s"));
    }
}

#[acmd_script( agent = "kamui", script = "sound_appealsr", category = ACMD_SOUND, low_priority )]
unsafe fn sound_appealsr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kamui_appeal01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_appeal_s02"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_appeal_s03"));
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_appeal_s04"));
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_appeal_s01"));
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        
    );
    smashline::install_acmd_scripts!(

        effect_specialswallattackb,
        effect_specialswallattackf

        /*
        kamui_attack13,
        kamui_stilt,
        kamui_utilt,
        kamui_dtilt,
        kamui_dsmash,
        kamui_usmash,*/
        
        //kamui_ssmashhi,
        //kamui_ssmashs,
        //kamui_ssmashlw,
        
        //kamui_specialnmax,
        //kamui_specialsjump,
        //kamui_specialsattackb,
        //kamui_specialsattackf,
        //kamui_catch
        
        /*
        kamui_soundappealhil,
        kamui_soundappealhir,
        kamui_soundappeallwl,
        kamui_soundappeallwr,
        kamui_soundappealsl,
        kamui_soundappealsr,
        
        kamui_soundattack11,
        kamui_soundattack12,
        kamui_soundattack13,
        kamui_soundutilt,
        kamui_soundstilt,
        kamui_soundsquat,
        kamui_soundusmash,
        kamui_soundssmashhi,
        kamui_soundssmashs,
        kamui_soundssmashlw,
        kamui_sounddsmash,
        kamui_soundthrowlw,
        kamui_soundthrowf,
        kamui_soundthrowb
        */
        /* 
        expression_appealhil,
        expression_appealhir,
        effect_appealhil,
        effect_appealhir,
        sound_appealhil,
        sound_appealhir,
        sound_appealsr
        */
       
        
    );
}
