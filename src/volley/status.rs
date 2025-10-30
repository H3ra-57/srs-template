use super::*;

unsafe extern "C" fn move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle: f32 = 10.0;
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ganon = get_battle_object_from_id(owner_id);
    let ganon_boma = &mut *(*ganon).module_accessor;
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_volley"), hash40("life_s"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_volley"), hash40("speed_s"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_pos_x = PostureModule::pos_x(ganon_boma);
    let owner_pos_y = PostureModule::pos_y(ganon_boma);
    let owner_pos_z = PostureModule::pos_z(ganon_boma);
    let speed_x = angle.to_radians().cos() * speed_max * lr;
    let speed_y = angle.to_radians().sin() * speed_max;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 1.5);
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y + 20.0, z: owner_pos_z});
    
    return 0.into();
}

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = *PostureModule::pos(weapon.module_accessor);
    if life <= 0 {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    /*if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }*/
    
    return 0.into();
}

unsafe extern "C" fn move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    return 0.into();
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE, move_pre);
    agent.status(Init, *WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE, move_init);
    agent.status(Main, *WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE, move_main);
    agent.status(Exec, *WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE, move_exec);
    agent.status(End, *WEAPON_GANON_VOLLEY_STATUS_KIND_MOVE, move_end);
}