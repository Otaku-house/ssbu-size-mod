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

#[acmd_script( agent = "zelda", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn zelda_bustattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        //bust attack
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=0.0, Y=11.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME )]
unsafe fn zelda_bustattacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        //bust attack
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=0.0, Y=11.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME )]
unsafe fn zelda_bustattacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        //bust attack
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=0.0, Y=10.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}



#[acmd_script( agent = "zelda", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn zelda_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START)
        }
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=13)
        FT_MOTION_RATE(FSM=1)
        for(7 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=40, FKB=20, BKB=0, Size=8.5, X=0.0, Y=7.0, Z=-0.5, X2=0.0, Y2=7.0, Z2=0.5, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=160, KBG=40, FKB=20, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=-10.0, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            }
            wait(Frame=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        frame(Frame=28)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=7.0, X=0.0, Y=8.0, Z=-4.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=5.0, X=0.0, Y=8.0, Z=-11.0, X2=0.0, Y2=8.0, Z2=11.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=43)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END)
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn zelda_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START)
        }
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=13)
        FT_MOTION_RATE(FSM=1)
        for(7 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=160, KBG=40, FKB=20, BKB=0, Size=8.5, X=0.0, Y=7.0, Z=-0.5, X2=0.0, Y2=7.0, Z2=0.5, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=160, KBG=40, FKB=20, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=-10.0, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            }
            wait(Frame=1)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
        frame(Frame=28)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=7.0, X=0.0, Y=8.0, Z=-4.0, X2=0.0, Y2=8.0, Z2=4.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=5.0, X=0.0, Y=8.0, Z=-11.0, X2=0.0, Y2=8.0, Z2=11.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=43)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END)
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn zelda_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=62, KBG=125, FKB=0, BKB=15, Size=3.0, X=0.0, Y=2.0, Z=2.5, X2=0.0, Y2=2.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.5, Angle=62, KBG=125, FKB=0, BKB=15, Size=3.0, X=0.0, Y=1.7, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.5, Angle=62, KBG=125, FKB=0, BKB=15, Size=3.0, X=0.0, Y=1.5, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=7)
        if(is_excute){
            AttackModule::clear_all()
        } 
    });
}

#[acmd_script( agent = "zelda", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn zelda_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=20, KBG=89, FKB=0, BKB=20, Size=4.2, X=0.0, Y=3.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=89, FKB=0, BKB=20, Size=3.0, X=0.0, Y=3.0, Z=11.0, X2=0.0, Y2=5.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=89, FKB=0, BKB=20, Size=5.0, X=0.0, Y=4.0, Z=-7.0, X2=0.0, Y2=4.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=89, FKB=0, BKB=20, Size=5.0, X=0.0, Y=4.0, Z=-7.0, X2=0.0, Y2=4.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=20, KBG=100, FKB=0, BKB=20, Size=4.2, X=0.0, Y=3.0, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=3.0, X=0.0, Y=3.0, Z=-9.0, X2=0.0, Y2=7.0, Z2=-4.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=89, FKB=0, BKB=20, Size=5.0, X=0.0, Y=4.0, Z=-7.0, X2=0.0, Y2=4.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn zelda_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        
    });
}

#[acmd_script( agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn zelda_attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        
    });
}

#[acmd_script( agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn zelda_attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("footl"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=3.1, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.1, X=-2.0, Y=0.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.2, X=0.7, Y=0.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.5, Angle=361, KBG=68, FKB=0, BKB=72, Size=2.3, X=3.2, Y=0.4, Z=-0.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=3, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.4, X=3.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            ATTACK(ID=4, Part=0, Bone=hash40("handl"), Damage=15.0, Angle=361, KBG=69, FKB=0, BKB=72, Size=2.5, X=6.2, Y=0.0, Z=1.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.88)
        
    });
}

#[acmd_script( agent = "zelda", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn zelda_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=10)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=300.8, X=0.0, Y=9.0, Z=4.0, X2=0.0, Y2=9.0, Z2=11.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=301.9, X=0.0, Y=9.0, Z=2.1, X2=0.0, Y2=9.0, Z2=13.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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

#[acmd_script( agent = "zelda", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn zelda_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            //MotionModule::change_motion(Hash40::new("slip"), 1.0, 1.0, false, 1.0, false, false)
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)
        }
    });
}

#[acmd_script( agent = "zelda", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn zelda_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            //MotionModule::change_motion(Hash40::new("slip"), 1.0, 1.0, false, 1.0, false, false)
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_SLIP, true)
        }
    });
}

#[acmd_script( agent = "zelda_phantom", script = "game_build", category = ACMD_GAME, low_priority )]
unsafe fn zelda_phantombuild(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=0)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=2)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=17)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=19)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=27)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=31)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=37)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=44)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=49)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
        frame(Frame=120)
        if(is_excute){
            WorkModule::inc_int(WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_INT_TERM)
        }
    });
}

#[acmd_script( agent = "zelda_phantom", script = "game_build", category = ACMD_SOUND, low_priority )]
unsafe fn zelda_phantombuildsound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            STOP_SE(hash40("se_zelda_special_l10"))
        }
        frame(Frame=22)
        if(is_excute){
            STOP_SE(hash40("se_zelda_special_l10_02"))
        }
        frame(Frame=39)
        if(is_excute){
            STOP_SE(hash40("se_zelda_special_l10_03"))
        }
        frame(Frame=55)
        if(is_excute){
            STOP_SE(hash40("se_zelda_special_l10_04"))
        }
    });
}

#[acmd_script( agent = "zelda_phantom", script = "game_build", category = ACMD_EFFECT, low_priority )]
unsafe fn zelda_phantombuildeffect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            EFFECT_OFF()
        }
        frame(Frame=5)
        if(is_excute){
            EFFECT_OFF()
        }
        frame(Frame=8)
        if(is_excute){
            EFFECT_OFF()
        }
        frame(Frame=9)
        if(is_excute){
            EFFECT_OFF()
        }
        frame(Frame=14)
        if(is_excute){
            EFFECT_OFF()
        }
    });
}


#[acmd_script( agent = "zelda", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_squat"))
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appealhil", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_h01"))
            PLAY_SE(hash40("se_zelda_ware01"))
            PLAY_SE(hash40("se_zelda_appeal_h01"))
        }
        frame(Frame=28)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_ware01"))
        }
        frame(Frame=30)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appealhir", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_h01"))
            PLAY_SE(hash40("se_zelda_ware01"))
            PLAY_SE(hash40("se_zelda_appeal_h01"))
        }
        frame(Frame=28)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_ware01"))
        }
        frame(Frame=30)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appeallwl", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappeallwl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_l01"))
        }
        frame(Frame=25)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=80)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appeallwr", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappeallwr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=15)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_l01"))
        }
        frame(Frame=25)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=80)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appealsl", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_ware04"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_s01"))
        }
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_appeal_s01"))
        }
        frame(Frame=20)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_appealsr", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundappealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_ware04"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_appeal_s01"))
        }
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_appeal_s01"))
        }
        frame(Frame=20)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=72)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_swing_s"))
        }
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_magic01"))
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=31)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attack100end", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            STOP_SE(hash40("se_zelda_attack100"))
            PLAY_SE(hash40("se_zelda_attack100end"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=46)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attackdash", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=6)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
            PLAY_SE(hash40("se_zelda_attackdash_01"))
            PLAY_SE(hash40("se_zelda_swing_s"))
        }
        wait(Frames=13)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_attackdash_02"))
        }
        frame(Frame=20)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=30)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=6)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_attackhard_h01"))
            PLAY_SE(hash40("se_zelda_swing_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        wait(Frames=4)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_attack06"))
            PLAY_SE(hash40("se_zelda_smash_h01"))
        }
        wait(Frames=18)
        if(is_excute){
            PLAY_SE(0x12c3b6d87d)
        }
        frame(Frame=56)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=3)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_attack05"))
            PLAY_SE(hash40("se_zelda_smash_l01"))
        }
        frame(Frame=51)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_swing_m"))
            PLAY_SE(hash40("se_zelda_attackhard_s01"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=39)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attacks3hi", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_swing_m"))
            PLAY_SE(hash40("se_zelda_attackhard_s01"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=39)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attacks3lw", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=8)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_swing_m"))
            PLAY_SE(hash40("se_zelda_attackhard_s01"))
        }
        frame(Frame=11)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=39)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundattacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=12)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        wait(Frames=3)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_smash_s"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_smash_s01"))
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=37)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=58)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=67)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_specialn", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundspecialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("vc_zelda_special_n01"))
            PLAY_SE(hash40("se_zelda_special_n01"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=37)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=54)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_throwf", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=8)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        wait(Frames=4)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_attackdash_01"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        frame(Frame=25)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_throwhi", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundthrowhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        frame(Frame=8)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
        wait(Frames=4)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_smash_h01"))
        }
        wait(Frames=19)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SEQUENCE(hash40("seq_zelda_rnd_attack"))
        }
        frame(Frame=47)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}

#[acmd_script( agent = "zelda", script = "sound_catchattack", category = ACMD_SOUND , low_priority )]
unsafe fn zelda_soundcatchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_zelda_step_right_s"))
        }
    });
}


pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        
        zelda_bustattacks3,
        zelda_bustattacks3hi,
        zelda_bustattacks3lw
        /*
        zelda_attacklw3,
        zelda_attacklw4,
        zelda_attacks3,
        zelda_attacks3hi,
        zelda_attacks3lw,*/
        //zelda_catch
        /*
        zelda_soundappealhil,
        zelda_soundappealhir,
        zelda_soundappeallwl,
        zelda_soundappeallwr,
        zelda_soundappealsl,
        zelda_soundappealsr,
        zelda_soundattack100end,
        zelda_soundattack11,
        zelda_soundattackdash,
        zelda_soundattackhi3,
        zelda_soundattackhi4,
        zelda_soundattacklw4,
        zelda_soundattacks3hi,
        zelda_soundattacks3lw,
        zelda_soundattacks3,
        zelda_soundattacks4,
        zelda_soundspecialn,
        zelda_soundsquat,
        zelda_soundthrowf,
        zelda_soundthrowhi,
        zelda_soundcatchattack
        
        zelda_specialairlw,
        zelda_speciallw,
        zelda_phantombuild,
        zelda_phantombuildeffect,
        zelda_phantombuildsound
        */
    );
}
