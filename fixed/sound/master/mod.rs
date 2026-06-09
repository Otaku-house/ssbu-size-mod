use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd::EFFECT_COLOR;



#[acmd_script( agent = "master", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn master_attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=25, Size=3.5, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=20, Size=3.5, X=0.0, Y=9.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            //AttackModule::set_add_reaction_frame(ID=0, Frames=7, Unk=false)
            //AttackModule::set_add_reaction_frame(ID=1, Frames=7, Unk=false)
        }
        frame(Frame=6)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        frame(Frame=10)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=120, FKB=0, BKB=125, Size=4.0, X=0.0, Y=3.0, Z=-3.0, X2=0.0, Y2=0.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn master_stilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
            FT_MOTION_RATE(FSM=0.5)
        frame(Frame=4)
            FT_MOTION_RATE(FSM=1)
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("topl"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=5.0, X=0.0, Y=3.0, Z=1.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=7)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=2.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=3.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=7.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=11.5, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=6.0, X=0.0, Y=7.5, Z=10.0, X2=0.0, Y2=7.5, Z2=16.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
            AttackModule::clear(ID=4, false)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            //FighterAreaModuleImpl::enable_fix_jostle_area(3, 4)
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn master_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, -1)
            //methodlib::L2CValue::as_hash()const(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, hash40("attack_lw3"))
            ArticleModule::change_motion(*FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("attack_lw3"), false, -1.0)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=93, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=10.0, X2=0.0, Y2=2.8, Z2=12.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=96, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=17.0, X2=0.0, Y2=2.8, Z2=25.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=93, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=-1.0, X2=0.0, Y2=2.8, Z2=10.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=59)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacks4hi", category = ACMD_GAME, low_priority )]
unsafe fn master_ssmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, -1)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-3.0, X2=0.0, Y2=3.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        //execute(14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-1.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=24)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=25)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=86, FKB=0, BKB=55, Size=4.5, X=0.0, Y=5.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=13.0, Angle=361, KBG=86, FKB=0, BKB=55, Size=1.6, X=0.0, Y=3.0, Z=0.0, X2=0.0, Y2=13.0, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=19.5, Angle=36, KBG=86, FKB=0, BKB=60, Size=2.6, X=-0.5, Y=16.5, Z=0.0, X2=-0.5, Y2=23.0, Z2=0.0, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=19.5, Angle=36, KBG=86, FKB=0, BKB=60, Size=2.0, X=-0.5, Y=24.0, Z=-0.3, X2=-0.5, Y2=26.5, Z2=-0.3, Hitlag=1.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)    
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=85)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn master_ssmashs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, -1)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-3.0, X2=0.0, Y2=3.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        //execute(14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-1.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=24)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=25)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=55, Size=4.5, X=0.0, Y=5.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=55, Size=1.6, X=0.0, Y=3.0, Z=0.0, X2=0.0, Y2=13.0, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=18.0, Angle=36, KBG=90, FKB=0, BKB=60, Size=2.6, X=-0.5, Y=16.5, Z=0.0, X2=-0.5, Y2=23.0, Z2=0.0, Hitlag=1.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=18.0, Angle=36, KBG=90, FKB=0, BKB=60, Size=2.0, X=-0.5, Y=24.0, Z=-0.3, X2=-0.5, Y2=26.5, Z2=-0.3, Hitlag=1.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)           
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=85)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacks4lw", category = ACMD_GAME, low_priority )]
unsafe fn master_ssmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, -1)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-3.0, X2=0.0, Y2=3.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        //execute(14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=-1.0, X2=0.0, Y2=3.0, Z2=3.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=24)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=25)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.5, Angle=361, KBG=90, FKB=0, BKB=55, Size=4.5, X=0.0, Y=5.5, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=361, KBG=90, FKB=0, BKB=55, Size=1.6, X=0.0, Y=3.0, Z=0.0, X2=0.0, Y2=13.0, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=36, KBG=90, FKB=0, BKB=60, Size=2.6, X=-0.5, Y=16.5, Z=0.0, X2=-0.5, Y2=23.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=36, KBG=90, FKB=0, BKB=60, Size=2.0, X=-0.5, Y=24.0, Z=-0.3, X2=-0.5, Y2=26.5, Z2=-0.3, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)    
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=85)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn master_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(*FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, -1)
            //methodlib::L2CValue::as_hash()const(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, hash40("attack_lw4"))
            ArticleModule::change_motion(*FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), false, -1.0)
        }
        frame(Frame=3)
        //execute(3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        /*
        if (WorkModule::is_flag(*FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == ture ){
            //if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            if(is_excute){
                WorkModule::get_float(*FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), true, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME)
                //methodlib::L2CValue::as_hash()const(-335839065)
                //ArticleModule::change_motion()
            }
        }
        */
        frame(Frame=5)
        FT_MOTION_RATE(FSM=1.3)
        frame(Frame=15)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.7, X=0.0, Y=3.5, Z=-4.0, X2=0.0, Y2=3.5, Z2=6.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=3.7, X=0.0, Y=3.5, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.8, X=0.0, Y=13.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.7, X=0.0, Y=3.5, Z=-6.0, X2=0.0, Y2=3.5, Z2=4.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=26)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=3.7, X=0.0, Y=3.5, Z=-11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=23.0, Angle=63, KBG=66, FKB=0, BKB=74, Size=4.8, X=0.0, Y=13.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=78)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn master_dsmash2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 3.0);
    execute(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == false {
        if macros::is_excute(agent) {
            let restart_frame = WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40::new("attack_lw4"), true, restart_frame);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 4.7, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, 12.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 4.7, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(6.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 23.0, 63, 66, 0, 74, 3.7, 0.0, 3.5, -11.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 23.0, 63, 66, 0, 74, 4.8, 0.0, 13.0, 1.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "master", script = "game_specialsf", category = ACMD_GAME, low_priority )]
unsafe fn master_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=7, Part=0, Bone=hash40("footl"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=8, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=9, Part=0, Bone=hash40("top"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=25.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=19.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=11.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=14.0, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=13.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=8.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=10.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=6, ShieldstunMul=1.7)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=14.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=21.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=18.5, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=6, ShieldstunMul=1.7)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_specialsfdash", category = ACMD_GAME )]
unsafe fn master_sspecialdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST)
        }
        frame(Frame=4)
        if(is_excute){
            ATTACK(ID=7, Part=0, Bone=hash40("footl"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=8, Part=0, Bone=hash40("footr"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=9, Part=0, Bone=hash40("top"), Damage=11.0, Angle=45, KBG=76, FKB=0, BKB=48, Size=4.0, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST)
        }
        frame(Frame=8)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=25.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=3.5, X=0.0, Y=19.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=11.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=14.0, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=13.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=3.0, X=0.0, Y=8.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=10.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=6, ShieldstunMul=1.7)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=11.5, Angle=45, KBG=35, FKB=0, BKB=85, Size=5.0, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=14.0, Z2=11.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=5.5, X=0.0, Y=6.0, Z=22.0, X2=0.0, Y2=21.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=17.25, Angle=59, KBG=40, FKB=0, BKB=115, Size=2.0, X=0.0, Y=8.5, Z=28.0, X2=0.0, Y2=18.5, Z2=28.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=1.7)
            ATK_SET_SHIELD_SETOFF_MUL(ID=6, ShieldstunMul=1.7)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST)
        }
        frame(Frame=29)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST)
        }
        frame(Frame=53)
        if(is_excute){
            ArticleModule::remove_exist(*FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script( agent = "master", script = "game_catch", category = ACMD_GAME, low_priority )]
unsafe fn master_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=6)
        if(is_excute){
            CATCH(ID=0, Bone=hash40("top"), Size=300.3, X=0.0, Y=8.0, Z=4.5, X2=0.0, Y2=8.0, Z2=9.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
            CATCH(ID=1, Bone=hash40("top"), Size=301.65, X=0.0, Y=8.0, Z=2.85, X2=0.0, Y2=8.0, Z2=10.65, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
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


#[acmd_script( agent = "master", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn master_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLIP, true);
    }
}

#[acmd_script( agent = "master", script = "game_specialairnstart", category = ACMD_GAME, low_priority )]
unsafe fn master_specialairnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLIP, true);
    }
}



#[acmd_script( agent = "master", script = "sound_appealhil", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        //if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 
        //{
            frame(Frame=14)
            if(is_excute){
                PLAY_SE(hash40("se_master_appeal_h01"))
            }
        //}
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_master_cloth_attackdash"))
        }
        else {
            frame(Frame=20)
            if(is_excute){
                PLAY_SE(hash40("se_master_appeal_h01"))
                PLAY_SE(hash40("se_master_cloth_attackdash"))
            }
        }
        //if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
            frame(Frame=50)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
            frame(Frame=81)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
            frame(Frame=91)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
        //}
    });
}

#[acmd_script( agent = "master", script = "sound_appealhir", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        //if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 
        //{
            frame(Frame=14)
            if(is_excute){
                PLAY_SE(hash40("se_master_appeal_h01"))
            }
        //}
        frame(Frame=17)
        if(is_excute){
            PLAY_SE(hash40("se_master_cloth_attackdash"))
        }
        else {
            frame(Frame=20)
            if(is_excute){
                PLAY_SE(hash40("se_master_appeal_h01"))
                PLAY_SE(hash40("se_master_cloth_attackdash"))
            }
        }
        //if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 1 {
            frame(Frame=50)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
            frame(Frame=81)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
            frame(Frame=91)
            if(is_excute){
                PLAY_SE(hash40("se_master_step_right_s"))
            }
        //}
    });
}

#[acmd_script( agent = "master", script = "sound_appeallwl", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appeallwl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("vc_master_appeal03"))
            PLAY_SE(hash40("se_master_appeal_l01"))
        }
        frame(Frame=35)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=75)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
    });
}

#[acmd_script( agent = "master", script = "sound_appeallwr", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appeallwr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("vc_master_appeal03"))
            PLAY_SE(hash40("se_master_appeal_l01"))
        }
        frame(Frame=26)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=35)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=75)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=82)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
    });
}

#[acmd_script( agent = "master", script = "sound_appealsl", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_master_rnd_appeal_s"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_master_cloth_l"))
        }
        frame(Frame=23)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=29)
        if(is_excute){
            PLAY_SE(hash40("se_master_appeal_s01"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_left_s"))
            PLAY_SE(hash40("se_master_cloth_m"))
        }
        frame(Frame=69)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=76)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=77)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
    });
}

#[acmd_script( agent = "master", script = "sound_appealsr", category = ACMD_SOUND , low_priority )]
unsafe fn sound_appealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_master_rnd_appeal_s"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=10)
        if(is_excute){
            PLAY_SE(hash40("se_master_cloth_l"))
        }
        frame(Frame=23)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=29)
        if(is_excute){
            PLAY_SE(hash40("se_master_appeal_s01"))
        }
        frame(Frame=68)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_left_s"))
            PLAY_SE(hash40("se_master_cloth_m"))
        }
        frame(Frame=69)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=76)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=77)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
    });
}

#[acmd_script( agent = "master", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_master_swing_s"))
        }
        frame(Frame=4)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
        frame(Frame=30)
        if(is_excute){
            PLAY_SE(hash40("se_master_step_right_s"))
        }
    });
}

#[acmd_script( agent = "master", script = "sound_attack12", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_swing_m"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attack13", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_swing_l"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack01"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack02"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_attackhard_h01"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_master_attack07"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_h01"));
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_h02"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_h03"));
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attacklw42(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_l01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack_lw4"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_l02"));
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_attackhard_s01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack02"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack_s4"));
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_cloth_l"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_left_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s02"));
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attacks4hi", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attacks4hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack_s4"));
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_cloth_l"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_left_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s02"));
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attacks4lw", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attacks4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack_s4"));
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_cloth_l"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_left_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_smash_s02"));
    }
    frame(agent.lua_state_agent, 85.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_specialhi", category = ACMD_SOUND , low_priority )]
unsafe fn sound_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_master_special_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_master_special_h01"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_master_special_h02"));
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_h06"));
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_speciallw", category = ACMD_SOUND , low_priority )]
unsafe fn sound_speciallw(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_INT_VOICE_TYPE) == 0 {
        //macros::IS_RANDOM(agent, 2);
        //if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, true)){
            if macros::is_excute(agent) {
                WorkModule::set_int(agent.module_accessor, 1, *FIGHTER_MASTER_STATUS_SPECIAL_LW_INT_VOICE_TYPE);
            }
            /*
            else {
                if macros::is_excute(agent) {
                    WorkModule::set_int(agent.module_accessor, 2, *FIGHTER_MASTER_STATUS_SPECIAL_LW_INT_VOICE_TYPE);
                }
            }*/
        //}
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_INT_VOICE_TYPE) == 1 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_master_special_l01"));
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_l01"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_master_special_l01"), 57);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_l03"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_master_special_l03"), 60);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_special_lw"));
    }   
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_master_special_l06_01"));
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_l01"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_master_special_l01"), 57);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_l03"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_master_special_l03"), 60);
    }   
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_master_special_l06_02"));
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 88.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 143.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_specialsfdash", category = ACMD_SOUND , low_priority )]
unsafe fn sound_specialsfdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_special_s"));
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_special_s03"));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_cloth_m"));
    } 
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_attackdash", category = ACMD_SOUND , low_priority )]
unsafe fn sound_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack02"));
        macros::PLAY_SE(agent, Hash40::new("se_master_attackdash01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn sound_squat(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_squat"));
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_throwf", category = ACMD_SOUND , low_priority )]
unsafe fn sound_throwf2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_attack100"));
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack01"));
        macros::PLAY_SE(agent, Hash40::new("se_master_attack100end"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_attackhard_l01"));
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_throwb", category = ACMD_SOUND , low_priority )]
unsafe fn sound_throwb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_throw_b01"));
    }
    wait(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack01"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_throw_b02"));
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
    frame(agent.lua_state_agent, 72.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}

#[acmd_script( agent = "master", script = "sound_throwhi", category = ACMD_SOUND , low_priority )]
unsafe fn sound_throwhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_master_rnd_attack01"));
        macros::PLAY_SE(agent, Hash40::new("se_master_attackhard_h01"));
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_master_step_right_s"));
    }
}




pub fn install() {
    smashline::install_agent_frames!(
    
    );
    smashline::install_acmd_scripts!(
        master_specialnstart,
        master_specialairnstart
        /*
        master_attack12,
        master_stilt,
        master_dtilt,
        master_ssmashhi,
        master_ssmashs,
        master_ssmashlw,
        master_dsmash2,
        master_sspecial,
        master_sspecialdash,
        master_catch*/
        /*
        game_attacklw4,
        effect_attacklw4,
        sound_attacklw4,
        expression_attacklw4*/
        /*
        game_attackairb,
        effect_attackairb,
        sound_attackairb*/
        /*
        game_throwb,
        effect_throwb,
        sound_throwb*/
        /*
        game_throwf,
        effect_throwf,
        sound_throwf*/
        /* 
        sound_appealhil,
        sound_appealhir,
        sound_appeallwl,
        sound_appeallwr,
        sound_appealsl,
        sound_appealsr,
        sound_attack11,
        sound_attack12,
        sound_attack13,
        sound_attackhi3,
        sound_attackhi4,
        sound_attacklw42,
        sound_attacks3,
        sound_attacks4,
        sound_attacks4hi,
        sound_attacks4lw,
        sound_specialhi,
        sound_speciallw,
        sound_specialsfdash,
        sound_attackdash,
        sound_squat,
        sound_throwb2,
        sound_throwf2,
        sound_throwhi*/
    );
}
