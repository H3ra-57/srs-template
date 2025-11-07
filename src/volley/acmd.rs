use super::*;


unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ganon = get_battle_object_from_id(owner_id);
    let ganon_boma = &mut *(*ganon).module_accessor;
    let scale = WorkModule::get_float(ganon_boma, *VOLLEY_SCALE);
    let damage = WorkModule::get_float(ganon_boma, *VOLLEY_DAMAGE);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0 + damage, 82, 30, 0, 67, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, false);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_sscope_bullet_max"), Hash40::new("top"), 0, 0, 0.5, 0, 0, 0, 1.0, false);
    }
}

unsafe extern "C" fn sound_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_superscope_chargeshot_l"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);
    agent.acmd("sound_move", sound_move, Priority::Low);
}