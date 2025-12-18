use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("head"), 10.0, 80, 88, 0, 50, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 10.0, 80, 88, 0, 50, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 25.0, 88, 88, 0, 50, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
}

unsafe extern "C" fn sound_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_h03"));
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(agent, 0, 0, Hash40::new("head"), 10.0, 80, 88, 0, 50, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 10.0, 80, 88, 0, 50, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 80, 0, 40, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn sound_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_h03"));
    }
}

unsafe extern "C" fn expression_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent
    .set_costume([16].to_vec())
    .acmd("game_specialhi_cactuar", game_specialhi, Priority::Low)
    .acmd("sound_specialhi_cactuar", sound_specialhi, Priority::Low)
    .acmd("expression_specialhi_cactuar", expression_specialhi, Priority::Low)

    .acmd("game_specialairhi_cactuar", game_specialairhi, Priority::Low)
    .acmd("sound_specialairhi_cactuar", sound_specialairhi, Priority::Low)
    .acmd("expression_specialairhi_cactuar", expression_specialairhi, Priority::Low)
    .install();
}