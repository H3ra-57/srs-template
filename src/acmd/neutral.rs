use super::*;

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneer"), 2.0, 361, 130, 0, 55, 3.5, -1.5, 0.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 2.0, 361, 130, 0, 55, 3.5, 2.5, 0.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 2.0, 361, 130, 0, 55, 3.5, 6.0, 0.0, 2.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.1);
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 6.6, 0, -14, 0, 0, 1.1, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 11, 18, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true, 0.7);
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_hip_swing"));
    }
}

unsafe extern "C" fn expression_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent
    .set_costume([16].to_vec())
    .acmd("game_attack13_cactuar", game_attack13, Priority::Low)
    .acmd("effect_attack13_cactuar", effect_attack13, Priority::Low)
    .acmd("sound_attack13_cactuar", sound_attack13, Priority::Low)
    .acmd("expression_attack13_cactuar", expression_attack13, Priority::Low)
    .install();
}