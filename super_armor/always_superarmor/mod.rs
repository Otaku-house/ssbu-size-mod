use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash_script::*;
use smash::app::*;



pub static mut SCALE: [f32; 8] = [1.0; 8];


#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn peach_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]
fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]
fn szerosuit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_id = smash::app::Fighter::get_id_from_entry_id(entry);
        let entry_id = entry as usize;

        let shield = smash::app::lua_bind::ShieldModule::get_attack_mul(module_accessor, *FIGHTER_SHIELD_KIND_GUARD);
        SCALE[entry_id] = 1.0 / shield;

        if SCALE[entry_id] >= 1.0 
        || SCALE[entry_id] < 1.0 {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 50000.0);
            DamageModule::set_reaction_mul(module_accessor, 0.0000001);
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
        
    );
    smashline::install_agent_frames!(
        
        peach_frame,
        daisy_frame,
        zelda_frame,
        rosetta_frame,
        palutena_frame,
        szerosuit_frame,
        kamui_frame,
        reflet_frame,
        lucina_frame,
        master_frame,
        tantan_frame,
        eflame_frame,
        elight_frame
        
    );
}