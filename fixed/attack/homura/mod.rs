use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;




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


pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        homura_attack11,
        homura_dtilt,
        homura_dsmash,
        homura_specialn4,
        homura_specialairn4,
        homura_catch
        
    );
}
