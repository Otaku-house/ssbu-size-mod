use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;

  



#[acmd_script( agent = "palutena", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_speciallw", category = ACMD_SOUND, low_priority )]
unsafe fn sound_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_final01"))
        }
    });
}


#[acmd_script( agent = "palutena", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        //macros::SEARCH(agent, 0, 0, Hash40::new("bust"), 120.0, 0.0, 0.0, 0.0, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 361, 112, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

#[acmd_script( agent = "palutena", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn palutena_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 1.3, 361, 100, 30, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        //AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "palutena", script = "game_appeallwl", category = ACMD_GAME, low_priority )]
unsafe fn palutena_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 1.3, 361, 100, 0, 90, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "palutena", script = "game_appeallwr", category = ACMD_GAME, low_priority )]
unsafe fn palutena_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 66.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 1.3, 361, 100, 0, 90, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


#[acmd_script( agent = "palutena", script = "game_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            EFFECT_FLW_POS(hash40("palutena_counter_flash"), hash40("shield"), -1, 0, -3, 0, 0, 0, 1, true)
        }
        /* 
        get_value_float(SO_VAR_FLOAT_LR)
        if(0xee630(0, 0)){
            if(is_excute){
                EFFECT_FLW_POS(hash40("palutena_backlight"), hash40("top"), 0, 23, -1, 0, 90, 0, 1, false)
                LAST_EFFECT_SET_RATE(0.7)
            }
            else {
                if(is_excute){
                    EFFECT_FLW_POS(hash40("palutena_backlight"), hash40("top"), 0, 23, 1, 0, -90, 0, 1, false)
                    LAST_EFFECT_SET_RATE(0.7)
                }
            }
        }*/
        frame(Frame=6)
        if(is_excute){
            FLASH(1, 1, 1, 0.75)
        }
        wait(Frames=1)
        for(5 Iterations){
            if(is_excute){
                FLASH(0.7, 0.7, 0.7, 0.5)
            }
            wait(Frames=2)
            if(is_excute){
                FLASH(0.67, 0, 0.78, 0.31)
            }
            wait(Frames=2)
            if(is_excute){
                COL_NORMAL()
            }
            wait(Frames=2)
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_attack11", category = ACMD_GAME , low_priority )]
unsafe fn palutena_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=20, FKB=0, BKB=25, Size=2.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=20, FKB=0, BKB=25, Size=2.2, X=0.0, Y=8.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=20, FKB=0, BKB=20, Size=2.5, X=0.0, Y=8.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=20, FKB=0, BKB=20, Size=2.5, X=0.0, Y=8.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
            AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=4.0, Unk=false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        FT_MOTION_RATE(FSM=0.7)
        wait(Frames=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_attacks3", category = ACMD_GAME , low_priority )]
unsafe fn palutena_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=4.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=17)
        if(is_excute){
            HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=90, KBG=100, FKB=25, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=45, KBG=100, FKB=45, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=285, KBG=100, FKB=25, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=100, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=25)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=20, KBG=100, FKB=35, BKB=0, Size=4.1, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=6.0, Angle=85, KBG=100, FKB=30, BKB=0, Size=3.0, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=1, Part=1, Bone=hash40("stick"), Damage=7.0, Angle=40, KBG=80, FKB=0, BKB=78, Size=4.8, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=1, Bone=hash40("stick"), Damage=7.0, Angle=40, KBG=80, FKB=0, BKB=78, Size=3.9, X=0.0, Y=-5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=41)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=44)
        if(is_excute){
            AttackModule::clear_all()
            HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_attackhi3", category = ACMD_GAME , low_priority )]
unsafe fn palutena_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.4, Angle=361, KBG=100, FKB=0, BKB=70, Size=4.0, X=0.0, Y=2.0, Z=7.5, X2=0.0, Y2=2.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=7)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=18.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=130, KBG=100, FKB=50, BKB=0, Size=2.0, X=0.0, Y=15.0, Z=10.5, X2=0.0, Y2=15.0, Z2=-5.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.4, Angle=190, KBG=100, FKB=30, BKB=0, Size=2.0, X=0.0, Y=20.0, Z=10.5, X2=0.0, Y2=20.0, Z2=-5.0, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=30)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=31)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=85, KBG=136, FKB=0, BKB=75, Size=3.0, X=0.0, Y=23.0, Z=10.5, X2=0.0, Y2=23.0, Z2=-5.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=85, KBG=136, FKB=0, BKB=75, Size=3.0, X=0.0, Y=17.0, Z=10.5, X2=0.0, Y2=17.0, Z2=-5.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=33)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_catch", category = ACMD_GAME )]
unsafe fn palutena_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=7)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=360.6, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=9.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=180.8, X=0.0, Y=8.0, Z=2.2, X2=0.0, Y2=8.0, Z2=11.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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


#[acmd_script( agent = "palutena", script = "game_squat", category = ACMD_GAME )]
unsafe fn palutena_squat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("bust"), Damage=5.0, Angle=60, KBG=103, FKB=0, BKB=52, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_appealhil", category = ACMD_SOUND )]
unsafe fn palutena_soundappealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_final01"))
        }
        frame(Frame=41)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_appeal_h01"))
        }
        frame(Frame=78)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=89)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_appealhir", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundappealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_appeal01"))
        }
        frame(Frame=41)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_appeal_h01"))
        }
        frame(Frame=79)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=88)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "game_appeallwl", category = ACMD_SOUND )]
unsafe fn palutena_soundappeallwl2(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            macros::PLAY_SE(agent, Hash40::new("vc_palutena_win02"));
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("vc_palutena_win03"));
        }
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_l01"));
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_palutena_step_right_s"));
    }
}

#[acmd_script( agent = "palutena", script = "sound_appeallwl", category = ACMD_SOUND)]
unsafe fn palutena_soundappeallwl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=19)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_win03"))
        }
        frame(Frame=55)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_appeal_l01"))
        }
        frame(Frame=62)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_appeallwr", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundappeallwr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=19)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_appeal03"))
        }
        frame(Frame=55)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_appeal_l01"))
        }
        frame(Frame=62)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_appealsl", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundappealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_dash_start"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_appeal02"))
            PLAY_SE(hash40("se_palutena_appeal_s02"))
        }
        frame(Frame=81)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_appealsr", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundappealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_dash_start"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("vc_palutena_appeal02"))
            PLAY_SE(hash40("se_palutena_appeal_s02"))
        }
        frame(Frame=81)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_swing_s"))
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=32)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attack100end", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            //sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
            SoundModule::stop_status_se()
            PLAY_SE(hash40("vc_palutena_attack03"))
            PLAY_SE(hash40("se_palutena_attack100end"))
        }
        frame(Frame=40)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attackdash", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
            PLAY_SE(hash40("se_palutena_attackdash"))
        }
        frame(Frame=6)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        wait(Frames=49)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
            PLAY_SE(hash40("se_palutena_attackhard_h01"))
        }
        frame(Frame=8)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=17)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
            PLAY_SE(hash40("vc_palutena_attack07"))
            PLAY_SE(hash40("se_palutena_smash_h01"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=74)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attacklw3", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
            PLAY_SE(hash40("se_palutena_attackhard_l01"))
        } 
        frame(Frame=54)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_smash_l"))
            PLAY_SE(hash40("se_palutena_smash_l01"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=77)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=82)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=8)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
            PLAY_SE(hash40("se_palutena_attackhard_s01"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=44)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=60)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundattacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=14)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_smash_s"))
            PLAY_SE(hash40("se_palutena_smash_s01"))
        }
        frame(Frame=64)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_squat"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_speciallw", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundspeciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_special_l01"))
        }
        frame(Frame=59)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_speciallwattack", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundspeciallwattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_special_l02"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_special_l03"))
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_special_l01"))
        }
        frame(Frame=23)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_specialn", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundspecialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        //IS_EXIST_ARTICLE(FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTORETICLE)
        if( ArticleModule::is_exist(fighter.module_accessor ,*FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTORETICLE) == true ){
            if(is_excute){
                PLAY_SEQUENCE(hash40("seq_palutena_rnd_special_n"))
            }
        }
        if(is_excute){
            PLAY_STATUS(hash40("se_palutena_special_n01"))
        }
        wait(Frames=40)
        if(is_excute){
            //sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
            SoundModule::stop_status_se()
        } 
        wait(Frame=6)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        wait(Frame=11)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_specials", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundspecials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_special_s01"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=19)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_special_s"))
        }
        frame(Frame=64)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=90)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_throwb", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundthrowb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
        }
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_throw"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_throwf", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        wait(Frames=18)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_throw"))
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=61)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

#[acmd_script( agent = "palutena", script = "sound_throwhi", category = ACMD_SOUND , low_priority )]
unsafe fn palutena_soundthrowhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        wait(Frames=15)
        if(is_excute){
            PLAY_SE(hash40("se_palutena_throw"))
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SEQUENCE(hash40("seq_palutena_rnd_attack"))
        }
        frame(Frame=48)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
        frame(Frame=57)
        if(is_excute){
            PLAY_STEP(hash40("se_palutena_step_right_s"))
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        /* 
        palutena_soundappeallwl,
        palutena_soundappealhil,
        palutena_squat,
        game_speciallw,
        sound_speciallw,
        effect_speciallw
        effect_attackairb,
        sound_attackairb,
        game_attackairb
        */
        //palutena_appeallwl,
        //palutena_appeallwr,
        //palutena_catchattack,
        //palutena_specialn
        //palutena_squat
        //palutena_attack11,
        palutena_attackhi3,
        palutena_attacks3,
        palutena_catch
        /*
        palutena_soundappealhil,
        palutena_soundappealhir,
        palutena_soundappeallwl,
        palutena_soundappeallwr,
        palutena_soundappealsl,
        palutena_soundappealsr,
        palutena_soundattack100end,
        palutena_soundattack11,
        palutena_soundattackdash,
        palutena_soundattackhi3,
        palutena_soundattackhi4,
        palutena_soundattacklw3,
        palutena_soundattacklw4,
        palutena_soundattacks3,
        palutena_soundattacks4,
        palutena_soundspeciallw,
        palutena_soundspeciallwattack,
        palutena_soundspecialn,
        palutena_soundspecials,
        palutena_soundsquat,
        palutena_soundthrowb,
        palutena_soundthrowf,
        palutena_soundthrowhi
        */
        
    );
}
