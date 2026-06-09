use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;


#[acmd_script( agent = "eflame", script = "game_appeallwl", category = ACMD_GAME )]
unsafe fn game_appeallwl(agent: &mut L2CAgentBase) {
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



#[acmd_script( agent = "elight", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn hikari_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)    
        }
        
    });
}

#[acmd_script( agent = "elight", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn hikari_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)    
        }
        
    });
}

#[acmd_script( agent = "elight", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn hikari_pai(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("bust"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=65, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)    
        }
        wait(Frame=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_attackhi3", category = ACMD_SOUND, low_priority )]
unsafe fn hikari_sepai(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_STEP(hash40("se_elight_step_left_s"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_STEP(hash40("se_elight_step_left_s"))
        }
    });
}



#[acmd_script( agent = "elight", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn hikari_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //frame(0, 3)
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            WorkModule::set_int64(hash40("attack_lw4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=4)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=8)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=70, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.5, Angle=35, KBG=69, FKB=0, BKB=75, Size=4.0, X=0.0, Y=7.0, Z=15.0, X2=0.0, Y2=7.0, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=35, KBG=65, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.5, Angle=35, KBG=65, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-12.0, X2=0.0, Y2=7.0, Z2=-10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=70, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.5, Angle=35, KBG=69, FKB=0, BKB=75, Size=4.0, X=0.0, Y=7.0, Z=-15.0, X2=0.0, Y2=7.0, Z2=-10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=35, KBG=65, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.5, Angle=35, KBG=65, FKB=0, BKB=75, Size=3.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=27)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=88, KBG=102, FKB=0, BKB=50, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=75, KBG=102, FKB=0, BKB=55, Size=5.0, X=6.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=42)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=46)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "elight", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn hikari_specialn1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 10, 10, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=140, KBG=100, FKB=0, BKB=60, Size=1.0, X=2.6, Y=0.4, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        //frame(0, 5)
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=8.0, X2=0.0, Y2=12.0, Z2=16.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=15, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=9.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=110, KBG=100, FKB=20, BKB=10, Size=5.0, X=0.0, Y=6.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=9.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=0, BKB=80, Size=5.0, X=0.0, Y=4.0, Z=10.0, X2=0.0, Y2=4.0, Z2=-4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            CameraModule::req_quake(*CAMERA_QUAKE_KIND_L)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=8.0, X2=0.0, Y2=10.0, Z2=16.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=15, BKB=0, Size=5.0, X=0.0, Y=4.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=7.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=110, KBG=100, FKB=20, BKB=10, Size=5.0, X=0.0, Y=4.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=7.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
        AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
        }
        frame(Frame=6)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=100, FKB=50, BKB=25, Size=5.0, X=0.0, Y=7.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=145, KBG=100, FKB=45, BKB=20, Size=5.0, X=0.0, Y=7.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=100, FKB=50, BKB=25, Size=5.0, X=0.0, Y=5.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=145, KBG=100, FKB=45, BKB=20, Size=5.0, X=0.0, Y=5.0, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        frame(Frame=8)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=4.0, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=3.5, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=95, KBG=100, FKB=20, BKB=10, Size=2.8, X=11.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=95, KBG=100, FKB=20, BKB=10, Size=2.8, X=14.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=100, KBG=100, FKB=20, BKB=10, Size=6.0, X=0.0, Y=10.0, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=6.0, X=0.0, Y=10.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=3.5, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=95, KBG=100, FKB=20, BKB=10, Size=2.8, X=0.0, Y=11.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=95, KBG=100, FKB=20, BKB=10, Size=2.8, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=100, KBG=100, FKB=20, BKB=10, Size=6.0, X=0.0, Y=6.0, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=20, BKB=10, Size=6.0, X=0.0, Y=6.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
        }
        frame(Frame=18)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=366, KBG=100, FKB=10, BKB=0, Size=2.8, X=14.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=12.0, Z=10.0, X2=0.0, Y2=12.0, Z2=16.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=366, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=6.0, Z=10.0, X2=0.0, Y2=6.0, Z2=16.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=4.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=3.5, X=10.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=2.8, X=15.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=2.8, X=18.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=7.0, X=0.0, Y=10.0, Z=10.0, X2=0.0, Y2=10.0, Z2=20.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=8.0, Angle=42, KBG=100, FKB=0, BKB=85, Size=5.0, X=0.0, Y=4.0, Z=3.0, X2=0.0, Y2=4.0, Z2=-3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=4.0, X=0.0, Y=5.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=3.5, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=2.8, X=0.0, Y=15.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=2.8, X=0.0, Y=18.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=6.0, Angle=42, KBG=84, FKB=0, BKB=75, Size=7.0, X=0.0, Y=11.0, Z=10.0, X2=0.0, Y2=11.0, Z2=17.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
        }
        frame(Frame=37)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
        }
        frame(Frame=38)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2)
        }
        frame(Frame=42)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=46)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 3.33, 3.33, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //frame(0, 52)
        frame(Frame=52)
        FT_MOTION_RATE(FSM=1)
        /*
        frame(Frame=70)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
        */
    });
}

#[acmd_script( agent = "elight", script = "game_specialn2", category = ACMD_GAME , low_priority )]
unsafe fn hikari_specialn2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 10, 10, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=2)
        if(is_excute){
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=140, KBG=100, FKB=0, BKB=60, Size=1.0, X=2.6, Y=0.4, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        //frame(0, 5)
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=15, BKB=0, Size=7.0, X=0.0, Y=10.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=10.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=110, KBG=100, FKB=20, BKB=10, Size=7.0, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=80, Size=5.0, X=0.0, Y=4.0, Z=10.0, X2=0.0, Y2=4.0, Z2=-4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            CameraModule::req_quake(*CAMERA_QUAKE_KIND_L)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=90, KBG=100, FKB=15, BKB=0, Size=7.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=8.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
           ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=110, KBG=100, FKB=20, BKB=10, Size=7.0, X=0.0, Y=8.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=20, BKB=0, Size=7.0, X=0.0, Y=8.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */     
        if(is_excute){
            AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
        }
        frame(Frame=6)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=100, FKB=60, BKB=35, Size=5.0, X=0.0, Y=7.0, Z=-13.0, X2=0.0, Y2=7.0, Z2=-8.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=145, KBG=100, FKB=45, BKB=20, Size=5.0, X=0.0, Y=7.0, Z=-13.0, X2=0.0, Y2=7.0, Z2=-8.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=100, FKB=60, BKB=35, Size=5.0, X=0.0, Y=5.0, Z=-13.0, X2=0.0, Y2=7.0, Z2=-8.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=145, KBG=100, FKB=45, BKB=20, Size=5.0, X=0.0, Y=5.0, Z=-13.0, X2=0.0, Y2=7.0, Z2=-8.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK1)
        }
        frame(Frame=12)
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=4.0, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=3.5, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=100, KBG=100, FKB=20, BKB=15, Size=2.8, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=10, Size=2.8, X=14.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=11.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=11.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=11.0, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=4.0, X=0.0, Y=11.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=8, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=3.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=3.5, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=100, KBG=100, FKB=20, BKB=15, Size=2.8, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=10, Size=2.8, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=7.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=20, Size=6.0, X=0.0, Y=7.0, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=2.0, Angle=15, KBG=100, FKB=20, BKB=15, Size=6.0, X=0.0, Y=7.0, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=6, ShieldstunMul=2)
        }
        frame(Frame=17)
        if(is_excute){
            WHOLE_HIT(HIT_STATUS_NORMAL)
        }
        frame(Frame=18)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=366, KBG=100, FKB=10, BKB=0, Size=2.8, X=15.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=13.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=13.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=366, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=8.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=6.0, X=0.0, Y=8.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=2)
        }
        frame(Frame=28)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=3.0, Z=3.0, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=35)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=15.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=13.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=13.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=4.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.0, Angle=90, KBG=100, FKB=10, BKB=0, Size=2.8, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=14.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=2)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=2)
        }
        frame(Frame=37)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=45)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=4.0, X=2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=3.5, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=2.8, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=2.8, X=15.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=7.0, X=0.0, Y=12.0, Z=10.0, X2=0.0, Y2=12.0, Z2=26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=8.0, Angle=42, KBG=100, FKB=0, BKB=85, Size=5.0, X=0.0, Y=4.0, Z=3.0, X2=0.0, Y2=4.0, Z2=-3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        /*
        if(0x357740(FIGHTER_INSTANCE_WORK_ID_INT_KIND, FIGHTER_KIND_KIRBY)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=4.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=3.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=2.8, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=2.8, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=8.5, Angle=42, KBG=78, FKB=0, BKB=55, Size=7.0, X=0.0, Y=12.0, Z=10.0, X2=0.0, Y2=12.0, Z2=26.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_ALL, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        */
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_ATTACK_SET)
        }
        frame(Frame=46)
        if(is_excute){
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
        }
        frame(Frame=47)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_ELIGHT_STATUS_SPECIAL_N_FLAG_HIT_CHECK2)
        }
        frame(Frame=52)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 3.33, 3.33, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 3.33, 3.33, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        /*
        frame(Frame=70)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
        */
    });
}

#[acmd_script( agent = "elight", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn hikari_ssmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //FT_MOTION_RATE(0, 0.5)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            WorkModule::set_int64(hash40("attack_s4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=9)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=76, FKB=0, BKB=75, Size=4.5, X=0.0, Y=4.0, Z=3.5, X2=0.0, Y2=4.0, Z2=4.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            CameraModule::req_quake(*CAMERA_QUAKE_KIND_KL)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.5, X=0.0, Y=12.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=17.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.5, X=0.0, Y=21.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.0, X=0.0, Y=21.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=8.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=19)
        if(is_excute){
            AttackModule::clear(5, false)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.5, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=8.0, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.5, X=0.0, Y=11.0, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.0, X=0.0, Y=13.0, Z=22.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=5.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.5, X=0.0, Y=2.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=2.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.5, X=0.0, Y=2.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=2.0, X=0.0, Y=2.0, Z=23.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.5, Angle=361, KBG=62, FKB=0, BKB=75, Size=3.0, X=0.0, Y=2.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=37)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        frame(Frame=42)
        if(is_excute){
            CameraModule::stop_quake(*CAMERA_QUAKE_KIND_KL)
        }
        //frame(0, 64)
        frame(Frame=64)
        FT_MOTION_RATE(FSM=0.91)
    });
}

#[acmd_script( agent = "elight", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn hikari_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //FT_MOTION_RATE(0, 0.5)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=5)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            WorkModule::set_int64(hash40("attack_hi4_hold") as i64, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND)
        }
        frame(Frame=8)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=12)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=85, KBG=100, FKB=0, BKB=85, Size=4.0, X=0.0, Y=12.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=120, KBG=100, FKB=0, BKB=90, Size=5.5, X=0.0, Y=8.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=130, KBG=100, FKB=0, BKB=90, Size=5.5, X=0.0, Y=14.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=15)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=10, FKB=50, BKB=0, Size=2.0, X=0.0, Y=26.0, Z=0.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=100, FKB=50, BKB=0, Size=2.0, X=0.0, Y=26.0, Z=0.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=60, BKB=0, Size=2.0, X=0.0, Y=21.0, Z=0.0, X2=0.0, Y2=21.0, Z2=5.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=100, FKB=65, BKB=25, Size=4.5, X=0.0, Y=21.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        }
        frame(Frame=17)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=10, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-5.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=100, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-5.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=21.0, Z=-3.0, X2=0.0, Y2=21.0, Z2=5.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=145, KBG=100, FKB=65, BKB=25, Size=4.5, X=0.0, Y=21.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=10, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-10.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=100, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-10.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=21.0, Z=-8.0, X2=0.0, Y2=21.0, Z2=5.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=145, KBG=100, FKB=65, BKB=25, Size=4.5, X=0.0, Y=21.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=10, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-10.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=366, KBG=100, FKB=50, BKB=0, Size=3.0, X=0.0, Y=26.0, Z=-10.0, X2=0.0, Y2=26.0, Z2=7.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=21.0, Z=-8.0, X2=0.0, Y2=21.0, Z2=5.0, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.5, Angle=145, KBG=100, FKB=65, BKB=25, Size=4.5, X=0.0, Y=21.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.5, Angle=145, KBG=100, FKB=65, BKB=25, Size=4.5, X=0.0, Y=21.0, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.1, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
        }
        frame(Frame=29)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=83, KBG=108, FKB=0, BKB=75, Size=7.0, X=0.0, Y=25.0, Z=-7.0, X2=0.0, Y2=25.0, Z2=4.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=32)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=34)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //FT_MOTION_RATE(0, 0.75)
        FT_MOTION_RATE(FSM=0.75)
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
    });
}


#[acmd_script( agent = "elight", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn hikari_stilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=5)
        
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        
        //FT_MOTION_RATE(0, 1)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=11.5, Angle=35, KBG=78, FKB=0, BKB=52, Size=4.0, X=0.0, Y=3.0, Z=-2.0, X2=0.0, Y2=3.0, Z2=2.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.0, X=-1.5, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.0, X=-1.5, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=2.5, X=-1.5, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.0, X=0.0, Y=10.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.5, X=2.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.5, X=2.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.0, X=2.0, Y=13.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.5, Angle=62, KBG=65, FKB=0, BKB=52, Size=3.0, X=0.0, Y=5.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=12)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=22)
        
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //frame(0, 30)
        frame(Frame=30)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_script( agent = "elight", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn hikari_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if(ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)){
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_open"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if(MotionModule::is_changing(module_accessor)){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //frame(0, 5)
        frame(Frame=5)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=87, KBG=55, FKB=0, BKB=60, Size=3.0, X=0.0, Y=2.0, Z=8.0, X2=0.0, Y2=2.0, Z2=-1.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=93, KBG=60, FKB=0, BKB=67, Size=2.5, X=0.0, Y=2.0, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=93, KBG=60, FKB=0, BKB=67, Size=2.0, X=0.0, Y=2.0, Z=16.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=87, KBG=55, FKB=0, BKB=60, Size=3.0, X=0.0, Y=2.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=13)
        //IS_EXIST_ARTICLE(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)
        if (ArticleModule::is_exist(module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD)) {
            if(is_excute){
                //methodlib::L2CValue::as_hash()const(FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, hash40("to_close"), 5, 5, false, false, 0, false, true, false)
                ArticleModule::add_motion_partial(*FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false)
            }
        }
        //MotionModule::is_changing()
        if (MotionModule::is_changing(module_accessor)) {
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING)
            }
        }
        //frame(0, 14)
        frame(Frame=14)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=24)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_script( agent = "elight", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn hikari_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=15, Size=2.5, X=0.0, Y=8.0, Z=11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
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

#[acmd_script( agent = "elight", script = "game_turn", category = ACMD_GAME, low_priority )]
unsafe fn hikari_turn(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "elight", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn hikari_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
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
            GrabModule::clear_all()
            //grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

//  sound script


#[acmd_script( agent = "elight", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_elight_swing_s01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}


#[acmd_script( agent = "elight", script = "sound_attack12", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_elight_swing_m01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=46)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attack13", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_elight_swing_l01"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack01"))
        }
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=55)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attack100end", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_attack100_end"))
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack01"))
        }
        frame(Frame=29)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attack100start", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattack100start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_swing_04"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attackdash", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_elight_attackdash01"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack02"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=43)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_attackhard_h01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack02"))
        }
        frame(Frame=39)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=63)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_elight_smash_h01"))
        }
        frame(Frame=11)
        if(is_excute){
            //PLAY_SE(0x162653130e)
            PLAY_SE(hash40("vc_elight_smash_h_rand"))
        }
        frame(Frame=32)
        if(is_excute){
            PLAY_SE(hash40("se_elight_smash_h02"))
        }
        frame(Frame=41)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=83)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            //PLAY_SE(0x16bdc25118)
            PLAY_SE(hash40("vc_elight_smash_l_rand"))
        }
        frame(Frame=8)
        if(is_excute){
            PLAY_SE(hash40("se_elight_smash_l01"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=67)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=75)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_attackhard_s01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack03"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=52)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=61)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundattacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            //PLAY_SE(0x164f42e156)
            PLAY_SE(hash40("vc_elight_smash_s_rand"))
        }
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_elight_smash_s01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=62)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=92)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_element_squat"))
        }
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_specialn", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundspecialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n02_01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n02_02"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n02_03"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=36)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n02_end"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=82)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=91)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_specialn2", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundspecialn2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n03_01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n03_02"))
        }
        frame(Frame=18)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n03_03"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=35)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n03_04"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_n03_end"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=92)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=101)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_specialsstart", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundspecialsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_specialsend", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundspecials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            //PLAY_SE(0x18f0ac038e)
            //PLAY_SE(hash40("vc_elight_smash_s_rand"))
            //PLAY_SE(hash40("se_elight_special_s02_01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        /*/
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s02_02"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s02_03"))
        }
        frame(Frame=19)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s02_04"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s02_05"))
        }
        frame(Frame=34)
        if(is_excute){
            PLAY_SE(hash40("se_elight_special_s02_end"))
        }
        */
        frame(Frame=41)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=51)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_throwf", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_elight_dash_start"))
        }
        frame(Frame=9)
        if(is_excute){
            PLAY_SE(hash40("se_elight_throw_f01"))
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack01"))
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}

#[acmd_script( agent = "elight", script = "sound_throwb", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundthrowb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=14)
        if(is_excute){
            PLAY_SE(hash40("se_common_punch_kick_swing_l"))
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack01"))
        }
        frame(Frame=38)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        
    });
}

#[acmd_script( agent = "elight", script = "sound_throwhi", category = ACMD_SOUND , low_priority )]
unsafe fn hikari_soundthrowhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_elight_throw_h01"))
            PLAY_SEQUENCE(hash40("seq_elight_rnd_attack01"))
        }
        frame(Frame=33)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
        frame(Frame=55)
        if(is_excute){
            PLAY_SE(hash40("se_elight_step_left_s"))
        }
    });
}


        // throw harituke

#[acmd_script( agent = "elight", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn hikari_throwhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=0.1, Angle=84, KBG=65, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=0.1, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=8)
        if(is_excute){
            //CHECK_FINISH_CAMERA(5, 7)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
            //FighterCutInManager::set_throw_finish_offset(0, 0, 0)
        }
        frame(Frame=9)
        if(is_excute){
            //ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT, FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP, FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO)
        }    
    });
}

#[acmd_script( agent = "elight", script = "sound_throwhi", category = ACMD_SOUND )]
unsafe fn hikari_soundthrowhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            //PLAY_SE(hash40("se_elight_throw_h01"))
            PLAY_SE(hash40("vc_elight_damage01"))
        }
    });
}


pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        game_appeallwl,
        hikari_specialn1,
        hikari_specialn2
        /*
        //hikari_speciallw,
        //hikari_specialairlw
        hikari_attack11,
        hikari_specialn1,
        hikari_specialn2,
        hikari_dtilt,
        //hikari_stilt,
        hikari_dsmash,
        //hikari_ssmash,
        //hikari_usmash,
        //hikari_turn,
        hikari_catch*/
        /*
        hikari_soundattack100end,
        hikari_soundattack100start,
        hikari_soundattack11,
        hikari_soundattack12,
        hikari_soundattack13,
        hikari_soundattackdash,
        hikari_soundattackhi3,
        hikari_soundattackhi4,
        hikari_soundattacklw4,
        hikari_soundattacks3,
        hikari_soundattacks4,
        hikari_soundspecialn,
        hikari_soundspecialn2,
        hikari_soundspecials,
        hikari_soundspecialsstart,
        hikari_soundsquat,
        hikari_soundthrowb,
        hikari_soundthrowf,
        hikari_soundthrowhi
        */
        //hikari_pai,
        //hikari_sepai
    );
}
