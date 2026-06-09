use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;




#[acmd_script( agent = "peach", script = "sound_squat", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundsquat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_squat"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attack11", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_swing_s"))
        }
        frame(Frame=38)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attack12", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
	        PLAY_SE(hash40("se_peach_swing_s"))
        }
        frame(Frame=37)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attackhi3", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundutilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_peach_rnd_attack"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_peach_attackhard_h02"))
        }
        frame(Frame=29)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attacks3", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundstilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_peach_rnd_attack"))
            PLAY_SE(hash40("se_peach_attackhard_s01"))
        }
        frame(Frame=44)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attacks4hi", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundssmashhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_attack05"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_s02"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attacks4", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundssmashs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_attack05"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_s01"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attacks4lw", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundssmashlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        frame(Frame=12)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_attack05"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_s03"))
        }
        frame(Frame=45)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attackhi4", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundusmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_attack06"))
        }
        wait(Frames=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_h01"))
        }
        frame(Frame=33)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_attacklw4", category = ACMD_SOUND , low_priority )]
unsafe fn peach_sounddsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=4)
        if(is_excute){
            STOP_SE(hash40("se_common_smash_start_03"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_attack07"))
        }
        wait(Frames=1)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_l01"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_l02"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_l02"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_l02"))
        }
        wait(Frames=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_smash_l02"))
        }
        frame(Frame=56)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_appealhil", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundappealhil(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_peach_wear01"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_appeal01"))
        }
        frame(Frame=48)
        if(is_excute){
            PLAY_SE(hash40("se_peach_wear02"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=61)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_appealhir", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundappealhir(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
        frame(Frame=3)
        if(is_excute){
            PLAY_SE(hash40("se_peach_wear01"))
        }
        frame(Frame=13)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=22)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_appeal01"))
        }
        frame(Frame=48)
        if(is_excute){
            PLAY_SE(hash40("se_peach_wear02"))
        }
        frame(Frame=49)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=61)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_appeallwl", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundappeallwl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_appeal_l01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=3)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_appeal02"))
        }
        frame(Frame=46)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=59)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}
/* 
#[acmd_script( agent = "peach", script = "sound_appeallwr", category = ACMD_SOUND , low_priority )]
unsafe fn peach_soundappeallwr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=50)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
        frame(Frame=5)
        if(is_excute){
            PLAY_SE(hash40("se_peach_appeal_l01"))
        }
        frame(Frame=7)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=3)
        if(is_excute){
            PLAY_SE(hash40("vc_peach_appeal02"))
        }
        frame(Frame=46)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=59)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_left_s"))
        }
    });
}
*/

 

#[acmd_script( agent = "peach", script = "sound_appealsl", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundappealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
	        PLAY_SE(hash40("vc_peach_appeal03"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=43)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_appealsr", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundappealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
	        PLAY_SE(hash40("vc_peach_appeal03"))
        }
        frame(Frame=16)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=43)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_catch", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundcatch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            PLAY_SE(hash40("se_common_swing_03"))
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        wait(Frames=4)
        if(is_excute){
            STOP_SE(hash40("se_common_swing_03"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_throwhi", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundthrowhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        wait(Frames=17)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_peach_rnd_attack"))
            PLAY_SE(hash40("se_common_throw_02"))
        }
        frame(Frame=42)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_throwb", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundthrowb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_common_throw_01"))
        }
        wait(Frames=18)
        if(is_excute){
            PLAY_SEQUENCE(hash40("seq_peach_rnd_attack"))
            PLAY_SE(hash40("se_common_throw_02"))
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
        frame(Frame=52)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

#[acmd_script( agent = "peach", script = "sound_specialsend", category = ACMD_SOUND, low_priority )]
unsafe fn peach_soundspecialslanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            PLAY_SE(hash40("se_peach_special_s04"))
        }
        frame(Frame=30)
        if(is_excute){
            PLAY_SE(hash40("se_peach_step_right_s"))
        }
    });
}

pub fn install() {
    smashline::install_agent_frames!(
        
    );
    smashline::install_acmd_scripts!(
        
        
        
        peach_soundsquat
        /* 
        peach_soundattack11,
        peach_soundattack12,
        peach_soundutilt,
        peach_soundstilt,
        peach_soundusmash,
        peach_soundssmashhi,
        peach_soundssmashs,
        peach_soundssmashlw,
        peach_sounddsmash,
        peach_soundappealhil,
        peach_soundappealhir,
        peach_soundappeallwl,
        peach_soundappeallwr,
        peach_soundappealsl,
        peach_soundappealsr,
        peach_soundcatch,
        peach_soundthrowhi,
        peach_soundthrowb,
        peach_soundspecialslanding
        */
    );
}
