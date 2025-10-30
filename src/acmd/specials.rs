use super::*;


unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 30.0, 10.0);
    frame(lua_state, 30.0);
    FT_MOTION_RATE_RANGE(agent, 30.0, 180.0, 100.0);
    frame(lua_state, 180.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_superscope_charge"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, VOLLEY, false, -1);
    }
}

unsafe extern "C" fn effect_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

unsafe extern "C" fn sound_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

unsafe extern "C" fn expression_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("sound_specialn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialn", expression_specialn, Priority::Low);

    agent.acmd("game_specialairn", game_specialn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialn, Priority::Low);
    agent.acmd("sound_specialairn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialairn", expression_specialn, Priority::Low);

    agent.acmd("game_specialnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("effect_specialnshoot", effect_specialnshoot, Priority::Low);
    agent.acmd("sound_specialnshoot", sound_specialnshoot, Priority::Low);
    agent.acmd("expression_specialnshoot", expression_specialnshoot, Priority::Low);

    agent.acmd("game_specialairnshoot", game_specialnshoot, Priority::Low);
    agent.acmd("effect_specialairnshoot", effect_specialnshoot, Priority::Low);
    agent.acmd("sound_specialairnshoot", sound_specialnshoot, Priority::Low);
    agent.acmd("expression_specialairnshoot", expression_specialnshoot, Priority::Low);
}