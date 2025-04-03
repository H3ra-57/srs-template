use std::{hash, task::ready};

use super::*;


pub const FIGHTER_KIND: i32 = 0x2;
pub const OBJECT_ID: i32 = 0x3;
pub const CURRENT_FRAME: i32 = 0xE;
pub const SUB_STATUS: i32 = 0x15;
pub const SITUATION_KIND: i32 = 0x16;

// Status script
unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_LINK {
        pLVar4 = (L2CValue *)lib::L2CValue::operator[](pLVar7,3);
        lib::L2CValue::L2CValue(&LStack_50,_ITEM_KIND_LINKBOMB);
        uVar1 = lib::L2CValue::as_integer(pLVar4);
        IVar2 = lib::L2CValue::as_integer(&LStack_50);
        let uVar5 = ItemManager::get_num_of_ownered_item(FIGHTER_STATUS_AIR_LASSO_REACH_WORK_FLOAT_CLIFF_POS_Y,uVar1,IVar2);
        lib::L2CValue::L2CValue(&LStack_60,uVar5);
        lib::L2CValue::L2CValue(&LStack_50,0);
        uVar5 = lib::L2CValue::operator<(&LStack_50,(L2CValue *)&LStack_60);
        if ((uVar5 & 1) != 0) {
            lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_ALREADY_GENERATED);
            iVar3 = lib::L2CValue::as_integer(&LStack_50);
            WorkModule::on_flag(fighter.module_accessor,iVar3);
        }
    }*/
    WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_FALL);
    WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_WAIT);
    
    WorkModule::set_int64(fighter.module_accessor,hash40("special_lw"),*FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOT_GROUND);
    WorkModule::set_int64(fighter.module_accessor,hash40("special_air_lw"),*FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOT_AIR);
    
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_MOT_INHERIT);
    WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_ITEM_NO_COUNT);
    
    if fighter.global_table[FIGHTER_KIND] != *FIGHTER_KIND_LINK {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_SET_ITEM_HOLD_ANIM);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_SET_ITEM_HOLD_ANIM);
    }
    if fighter.global_table[FIGHTER_KIND] != *FIGHTER_KIND_YOUNGLINK {
        if fighter.global_table[FIGHTER_KIND] != *FIGHTER_KIND_TOONLINK {
            fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
        }
    }
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let a0 = 0;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    /*if CancelModule::is_enable_cancel(fighter.module_accessor) {
        lib::L2CValue::L2CValue(&LStack_b0,false);
        lua2cpp::L2CFighterCommon::sub_wait_ground_check_common(this,SUB81(&LStack_b0,0));
        lib::L2CValue::L2CValue(&LStack_50,false);
        uVar7 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
        }
        else {
            lua2cpp::L2CFighterCommon::sub_air_check_fall_common(this);
            lib::L2CValue::L2CValue(&LStack_50,false);
            uVar7 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_50);
            if ((uVar7 & 1) != 0) goto LAB_71000199b4;
        }
        lib::L2CValue::L2CValue(return_value,0);
        goto LAB_710001a300;
    }*/
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || !fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
  //LAB_71000199b4:
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
            else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor,*FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_FALL) {
            if MotionModule::is_end(fighter.module_accessor) {
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_FALL);
                lib::L2CValue::L2CValue(&LStack_60,false);
                lua2cpp::L2CFighterBase::change_status(this,(L2CValue)(cVar1 + -0x40),(L2CValue)(cVar1 + -0x50));
                lib::L2CValue::L2CValue(return_value,1);
                goto LAB_710001a300;
            }
        }
        else {
        }
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_WAIT);
        iVar4 = lib::L2CValue::as_integer(&LStack_70);
        bVar3 = WorkModule::is_enable_transition_term(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar3 & 1));
        lib::L2CValue::L2CValue(&LStack_50,false);
        uVar7 = lib::L2CValue::operator==(&LStack_60,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
            bVar3 = MotionModule::is_end(fighter.module_accessor);
            lib::L2CValue::L2CValue(&LStack_80,(bool)(bVar3 & 1));
            lib::L2CValue::L2CValue(&LStack_50,false);
            uVar7 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_50);
            if ((uVar7 & 1) == 0) {
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_STATUS_KIND_WAIT);
                lib::L2CValue::L2CValue(&LStack_60,false);
                lua2cpp::L2CFighterBase::change_status(this,(L2CValue)(cVar1 + -0x40),(L2CValue)(cVar1 + -0x50));
                lib::L2CValue::L2CValue(return_value,1);
                goto LAB_710001a300;
            }
        }
        else {
        }
    }
    FUN_7100012b50(&LStack_60,this);
    bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
    if (bVar2) {
        pLVar8 = (L2CValue *)lib::L2CValue::operator[](&fighter.global_table,0x16);
        lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
        uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) goto LAB_7100019d74;
        lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
        iVar4 = lib::L2CValue::as_integer(&LStack_50);
        KineticModule::change_kinetic(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_c0,_SITUATION_KIND_GROUND);
        lua2cpp::L2CFighterBase::set_situation(this,SUB81(&LStack_c0,0));
        lib::L2CValue::L2CValue(&LStack_50,_GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
        GVar5 = lib::L2CValue::as_integer(&LStack_50);
        GroundModule::correct(fighter.module_accessor,GVar5);
        lib::L2CValue::L2CValue(&LStack_50,0);
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOT_GROUND);
        iVar4 = lib::L2CValue::as_integer(&LStack_70);
        lVar9 = WorkModule::get_int64(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_60,lVar9);
        lib::L2CValue::operator=(&LStack_50,(L2CValue *)&LStack_60);
        lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOTION);
        lVar9 = lib::L2CValue::as_integer(&LStack_50);
        iVar4 = lib::L2CValue::as_integer(&LStack_60);
        WorkModule::set_int64(fighter.module_accessor,lVar9,iVar4);
        FUN_710001a670(this);
        lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_WAIT);
        iVar4 = lib::L2CValue::as_integer(&LStack_50);
        WorkModule::enable_transition_term(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_FALL);
        iVar4 = lib::L2CValue::as_integer(&LStack_50);
        WorkModule::unable_transition_term(fighter.module_accessor,iVar4);
  LAB_7100019f2c:
    }
    else {
  LAB_7100019d74:
        pLVar8 = (L2CValue *)lib::L2CValue::operator[](&fighter.global_table,0x16);
        lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
        uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
            FUN_7100012b50(&LStack_50,this);
            bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
            if (bVar2) {
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_AIR_STOP);
                iVar4 = lib::L2CValue::as_integer(&LStack_50);
                KineticModule::change_kinetic(fighter.module_accessor,iVar4);
                lib::L2CValue::L2CValue(&LStack_c0,SITUATION_KIND_AIR);
                lua2cpp::L2CFighterBase::set_situation(this,SUB81(&LStack_c0,0));
                lib::L2CValue::L2CValue(&LStack_50,GROUND_CORRECT_KIND_AIR);
                GVar5 = lib::L2CValue::as_integer(&LStack_50);
                GroundModule::correct(fighter.module_accessor,GVar5);
                lib::L2CValue::L2CValue(&LStack_50,0);
                lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOT_AIR);
                iVar4 = lib::L2CValue::as_integer(&LStack_70);
                lVar9 = WorkModule::get_int64(fighter.module_accessor,iVar4);
                lib::L2CValue::L2CValue(&LStack_60,lVar9);
                lib::L2CValue::operator=(&LStack_50,(L2CValue *)&LStack_60);
                lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOTION);
                lVar9 = lib::L2CValue::as_integer(&LStack_50);
                iVar4 = lib::L2CValue::as_integer(&LStack_60);
                WorkModule::set_int64(fighter.module_accessor,lVar9,iVar4);
                FUN_710001a670(this);
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_WAIT);
                iVar4 = lib::L2CValue::as_integer(&LStack_50);
                WorkModule::unable_transition_term(fighter.module_accessor,iVar4);
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_BOMB_TRANSITION_TERM_ID_FALL);
                iVar4 = lib::L2CValue::as_integer(&LStack_50);
                WorkModule::enable_transition_term(fighter.module_accessor,iVar4);
                goto LAB_7100019f2c;
            }
        }
    }
    this_00 = &fighter.global_table;
    pLVar8 = (L2CValue *)lib::L2CValue::operator[](this_00,2);
    lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KIND_LINK);
    uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
    if ((uVar7 & 1) != 0) {
        bVar3 = ItemModule::is_have_item(fighter.module_accessor,0);
        lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar3 & 1));
        lib::L2CValue::L2CValue(&LStack_50,true);
        uVar7 = lib::L2CValue::operator==(&LStack_60,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
            lVar9 = -0x50;
        }
        else {
            lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_SET_ITEM_HOLD_ANIM);
            iVar4 = lib::L2CValue::as_integer(&LStack_70);
            bVar3 = WorkModule::is_flag(fighter.module_accessor,iVar4);
            lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar3 & 1));
            bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
            if (!bVar2) goto LAB_710001a04c;
            pLVar8 = (L2CValue *)lib::L2CValue::operator[](this_00,4);
            pFVar10 = (Fighter *)lib::L2CValue::as_pointer(pLVar8);
            app::FighterSpecializer_Link::set_item_hold_anim_for_script(pFVar10);
            lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_SET_ITEM_HOLD_ANIM);
            iVar4 = lib::L2CValue::as_integer(&LStack_50);
            WorkModule::off_flag(fighter.module_accessor,iVar4);
            lVar9 = -0x40;
        }
    }
  LAB_710001a04c:
    pLVar8 = (L2CValue *)lib::L2CValue::operator[](this_00,8);
    lib::L2CValue::L2CValue(&LStack_50,false);
    uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
    if ((uVar7 & 1) != 0) {
        lib::L2CValue::L2CValue(&LStack_60,0);
        lib::L2CValue::L2CValue(&LStack_70,0);
        lib::L2CValue::L2CValue(&LStack_80,_FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);
        pLVar8 = (L2CValue *)lib::L2CValue::operator[](&fighter.global_table,2);
        lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KIND_TOONLINK);
        uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
            pLVar8 = (L2CValue *)lib::L2CValue::operator[](&fighter.global_table,2);
            lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KIND_YOUNGLINK);
            uVar7 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack_50);
            if ((uVar7 & 1) != 0) {
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_YOUNGLINK_GENERATE_ARTICLE_YOUNGLINKBOMB);
                lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_50);
                goto LAB_710001a150;
            }
        }
        else {
            lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_TOONLINK_GENERATE_ARTICLE_TOONLINKBOMB);
            lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_50);
  LAB_710001a150:
        }
        lib::L2CValue::L2CValue(&LStack_90,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB);
        iVar4 = lib::L2CValue::as_integer(&LStack_90);
        bVar3 = WorkModule::is_flag(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar3 & 1));
        lib::L2CValue::operator=(&LStack_70,(L2CValue *)&LStack_50);
        lib::L2CValue::L2CValue(&LStack_90,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_ALREADY_GENERATED);
        iVar4 = lib::L2CValue::as_integer(&LStack_90);
        bVar3 = WorkModule::is_flag(fighter.module_accessor,iVar4);
        lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar3 & 1));
        lib::L2CValue::operator=(&LStack_60,(L2CValue *)&LStack_50);
        lib::L2CValue::L2CValue(&LStack_50,false);
        uVar7 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_50);
        if ((uVar7 & 1) == 0) {
            lib::L2CValue::L2CValue(&LStack_50,false);
            uVar7 = lib::L2CValue::operator==(&LStack_60,(L2CValue *)&LStack_50);
            if ((uVar7 & 1) != 0) {
                iVar4 = lib::L2CValue::as_integer(&LStack_80);
                bVar3 = ArticleModule::is_generatable(fighter.module_accessor,iVar4);
                lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar3 & 1));
                bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
                if (bVar2) {
                    lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_HAVE_ITEM_WORK_MAIN);
                    iVar4 = lib::L2CValue::as_integer(&LStack_80);
                    iVar6 = lib::L2CValue::as_integer(&LStack_50);
                    ArticleModule::generate_article_have_item(fighter.module_accessor,iVar4,iVar6,0x7fb997a80);
                }
                lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_ALREADY_GENERATED);
                iVar4 = lib::L2CValue::as_integer(&LStack_50);
                WorkModule::on_flag(fighter.module_accessor,iVar4);
            }
        }
    }
    lib::L2CValue::L2CValue(return_value,0);
  LAB_710001a300:
    return;
}

unsafe extern "C" fn FUN_7100012b50 (fighter: &mut L2CFighterCommon) -> L2CValue {
    bVar1 = StatusModule::is_changing(fighter.module_accessor);
    lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue(&LStack_40,false);
    uVar2 = lib::L2CValue::operator==(&LStack_50,(L2CValue *)&LStack_40);
    if ((uVar2 & 1) == 0) {
    LAB_7100012c24:
    LAB_7100012c2c:
        iVar5 = 1;
    }
    else {
        pLVar4 = (L2CValue *)(param_2 + 200);
        pLVar3 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x17);
        lib::L2CValue::L2CValue(&LStack_40,_SITUATION_KIND_GROUND);
        uVar2 = lib::L2CValue::operator==(pLVar3,(L2CValue *)&LStack_40);
        if ((uVar2 & 1) != 0) {
            pLVar3 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x16);
            lib::L2CValue::L2CValue(&LStack_40,SITUATION_KIND_AIR);
            uVar2 = lib::L2CValue::operator==(pLVar3,(L2CValue *)&LStack_40);
            if ((uVar2 & 1) != 0) goto LAB_7100012c24;
        }
        pLVar3 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x17);
        lib::L2CValue::L2CValue(&LStack_40,_SITUATION_KIND_GROUND);
        uVar2 = lib::L2CValue::operator==(pLVar3,(L2CValue *)&LStack_40);
        if ((uVar2 & 1) == 0) {
            pLVar4 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x16);
            lib::L2CValue::L2CValue(&LStack_40,_SITUATION_KIND_GROUND);
            uVar2 = lib::L2CValue::operator==(pLVar4,(L2CValue *)&LStack_40);
            if ((uVar2 & 1) != 0) goto LAB_7100012c2c;
        }
        else {
        }
        iVar5 = 0;
    }
    lib::L2CValue::L2CValue(param_1,iVar5);
    return;
}

unsafe extern "C" fn FUN_710001a670(fighter: &mut L2CFighterCommon) -> L2CValue {
    lib::L2CValue::L2CValue(&LStack_60,0);
    lib::L2CValue::L2CValue(&LStack_80,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_MOT_INHERIT);
    iVar2 = lib::L2CValue::as_integer(&LStack_80);
    bVar1 = WorkModule::is_flag(fighter.module_accessor,iVar2);
    lib::L2CValue::L2CValue(&LStack_70,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue(&LStack_50,false);
    uVar3 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_50);
    if ((uVar3 & 1) == 0) {
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOTION);
        iVar2 = lib::L2CValue::as_integer(&LStack_70);
        lVar4 = WorkModule::get_int64(fighter.module_accessor,iVar2);
        lib::L2CValue::L2CValue(&LStack_50,lVar4);
        lib::L2CValue::operator=(&LStack_60,(L2CValue *)&LStack_50);
        HVar5 = lib::L2CValue::as_hash(&LStack_60);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor,HVar5,-1.0,1.0,0.0,false,false);
    }
    else {
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_LINK_STATUS_WORK_ID_INT_BOMB_MOTION);
        iVar2 = lib::L2CValue::as_integer(&LStack_70);
        lVar4 = WorkModule::get_int64(fighter.module_accessor,iVar2);
        lib::L2CValue::L2CValue(&LStack_50,lVar4);
        lib::L2CValue::operator=(&LStack_60,(L2CValue *)&LStack_50);
        lib::L2CValue::L2CValue(&LStack_50,0.0);
        lib::L2CValue::L2CValue(&LStack_70,1.0);
        lib::L2CValue::L2CValue(&LStack_80,false);
        HVar5 = lib::L2CValue::as_hash(&LStack_60);
        fVar6 = (float)lib::L2CValue::as_number(&LStack_50);
        fVar7 = (float)lib::L2CValue::as_number(&LStack_70);
        bVar1 = lib::L2CValue::as_bool(&LStack_80);
        MotionModule::change_motion(fighter.module_accessor,HVar5,fVar6,fVar7,(bool)(bVar1 & 1),0.0,false,false);
        lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_MOT_INHERIT);
        iVar2 = lib::L2CValue::as_integer(&LStack_50);
        WorkModule::on_flag(fighter.module_accessor,iVar2);
    }
    return;
}

pub fn install() {
    Agent::new("link")
        .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST, special_lw_blast_main) // Status script
        .install();
}
