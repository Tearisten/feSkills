#![feature(ptr_sub_ptr)]
#![allow(warnings)]

use std::fs::read_to_string;
use std::fs;
use std::ptr::null;

use engage::app::IHubAccessMethods;
use engage::app::ILevelUpSequnece;
use engage::app::ISkillData;
use engage::app::ISkillDataMethods;
use engage::app::IStructBase;
use engage::app::IUnit;
use skyline::install_hook;
use skyline::patching::Patch;
use miniserde::Serialize;
use miniserde::Deserialize;
use miniserde::json;

use engage::app::Unit;
use engage::app::JobData;
use engage::app::SkillData;
use engage::app::LevelUpSequnece;
use engage::app::ProcInst;
use engage::prelude::Il2CppString;
use engage::app::Mess;
use engage::app::IUnitMethods;
use engage::app::IStructData_1Methods;
use engage::app::IJobDataMethods;
use engage::app::ICapabilityBase_1;
use engage::app::IForceMethods;
use engage::app::Force_Type;
use engage::app::GameUserData;
use engage::app::ISingletonClass_1Methods;
use engage::prelude::Cast;
use engage::app::muscle_exercise::level::Level;
use engage::app::mess::Mess_IconCategory;
use engage::app::GameMessage;
use engage::app::IPersonDataMethods;
use engage::app::proc::Proc;
use engage::app::PersonData;
use unity::FromIlInstance;
use engage::app::HubPlayTalkAfter;
use engage::app::HubAccess;
use engage::app::IHubPlayTalkAfterMethods;
use engage::app::IProcInst;
use engage::app::unitpool::UnitPool;
use engage::app::RelianceData_Level;
use engage::app::HubPlayTalk;
use engage::app::IHubPlayTalkMethods;
use engage::app::HubUtil;
use engage::app::myroomreliancesubselect::MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem;
use engage::app::BasicMenu_Result;
use engage::app::MyRoomRelianceSubSelect_MyRoomRelianceCallSequence;
use unity::SystemObject;
use std::sync::Mutex;

static mut LearnList : Container = Container {vals: Vec::new()};

#[derive(Debug, Serialize, Deserialize)]
struct Learnset {
    pid: String,
    english: String,
    skills: Vec<(i32, String)>, // [(level, SID)]
}

#[derive(Debug, Serialize, Deserialize)]
struct Container{
    vals: Vec<Learnset>
}

// static mut xSkill: Il2CppString = Il2CppString::new("test");
// static mut X_SKILL: &Il2CppString = &Il2CppString::new("testx".into());
// static mut Y_SKILL: &Il2CppString = &Il2CppString::new("testy");

static MY_STRING: Mutex<String> = Mutex::new(String::new());

// #[unity::hook("App", "Unit", "GetCapabilityGrow")]
// pub fn GetCapabilityGrow(this: Unit, index: i32, auto_grow: bool , _method_info : u64) -> i32
// {
//     // auto_grow is false when it's the full rate (unit + job)
//     // true when just unit (verify this)
//     let mut val : i32 = call_original!(this, index, auto_grow, _method_info);

//     // adjust display value for growth display mods
//     if !auto_grow && this.has_skill_2("SID_Sorcerer")
//     {
//         let jd: &JobData = &this.get_job();
//         if index == 6
//         {
//             val += jd.get_diff_grow().m_data().get(1) as i32;

//         }
//         else if index == 1
//         {
//             val -= jd.get_diff_grow().m_data().get(1) as i32;
//         }
//     }
    
//     return val.into();
// }

// #[unity::hook("App", "Unit", "LevelUp")]
// pub fn LevelUp(this: &Unit, abort: i32, _method_info : u64)
// {unsafe{
    

//     // normal stuff / job growth skill
//     EditJobGrowths(this);
//     call_original!(this, abort, _method_info);
//     ResetJobGrowths(this);

//     // maybe learn new skill
//     // CheckForNewSkills(this, _method_info);
// }}

// pub fn EditJobGrowths(this: &Unit)
// {
//     // adjust values
//     let act = this.has_skill_2("SID_Sorcerer");
//     if act
//     {    
//         let jd: &JobData = &this.get_job();
//         let grows = jd.get_diff_grow();
//         let change_val = grows.m_data().get(1);
//         let cur_val = grows.m_data().get(6);
//         grows.m_data().set(6, cur_val + change_val);
//         grows.m_data().set(1,0);

//         jd.set_diff_grow(grows);
//     }
// }

// pub fn ResetJobGrowths(this: &Unit)
// {
//     // return values
//     let act = this.has_skill_2("SID_Sorcerer");
//     if act
//     {
//         let jd: &JobData = &this.get_job();
//         let grows = jd.get_diff_grow();
//         let change_val = grows.m_data().get(1);
//         let cur_val = grows.m_data().get(6);
//         grows.m_data().set(6, cur_val - change_val);
//         grows.m_data().set(1,change_val);

//         jd.set_diff_grow(grows);
//     }
// }


// #[skyline::from_offset(0x1A52530)]
// pub fn get_unit_force(this: &Unit, _method_info: u64) -> i32;

// #[unity::from_offset("App", "Unit", "AddToEquipSkillPool")]
// pub fn AddToEquipSkillPool(this: &Unit, sid: &Il2CppString, _method_info: u64);

// #[unity::from_offset("App", "Unit", "IsExistInEquipSkillPool")]
// pub fn IsExistInEquipSkillPool(this: &Unit, sid: &Il2CppString, _method_info: u64) -> bool;

// #[unity::from_offset("App", "Unit", "IsInheritanceEnable")]
// pub fn IsInheritanceEnable(this: &Unit, skill: &SkillData, _method_info: u64) -> bool;


// #[unity::class("App", "LevelUpSequnece")]
// pub struct LevelUpSequnece {
//     pub proc : ProcInstFields,
//     pub pad1 : u64,
//     pub pad2 : u64,
//     pub unit : &'static Unit
// }

// impl Bindable for LevelUpSequnece {}

// impl AsMut<LevelUpSequnece> for LevelUpSequnece {
//     fn as_mut(&mut self) -> &mut ProcInstFields {
//         &mut self.proc
//     }
// }

// #[unity::hook("App", "UnitReliance", "LevelUp")]
// pub fn SupportConv(p1: PersonData, p2: PersonData,_method_info: u64)
// {unsafe{
//     // make sure to check convo skip in my code mods
//     showError("l1".to_string());

// }}



// could use this one if i still wanted the rank popup too, but it's dumb
//#[unity::hook("App", "MyRoomRelianceSubSelect.MyRoomRelianceCallSequence", "LevelUpAfter")] 
#[unity::hook("App", "MyRoomRelianceSubSelect.MyRoomRelianceCallSequence", "LevelUp")]
pub fn MyRoom(this: MyRoomRelianceSubSelect_MyRoomRelianceCallSequence, _method_info: u64)
{unsafe{
    *MY_STRING.lock().unwrap() = "good".to_string(); // this is where we figure out the skill id
    showError("1st".to_string());

    // test pop up that IS WORKING
    call_original!(this, _method_info); //shares layer with vanilla "x and y are rank z"

    ShowSkillPopup("SID_LookoutDuty".into(), this.into());
}}

#[unity::hook("App", "MyRoomRelianceSubSelect.MyRoomRelianceCallSequence", "UpdateAchive")]// working but no proc inst
pub fn SupportConv(this: MyRoomRelianceSubSelect_MyRoomRelianceCallSequence, i1: Unit, i2: Unit, lvl: RelianceData_Level, _method_info: u64)
{unsafe{
    let val = MY_STRING.lock().unwrap().to_string(); // get the skill id we figured out before
    // showError(val);

    i1.add_to_equip_skill_pool(Il2CppString::new("SID_LookoutDuty"));
    i2.add_to_equip_skill_pool(Il2CppString::new("SID_LookoutDuty")); // test SKILL IS WORKING!!!
    call_original!(this, i1,i2,lvl,_method_info); // try above if not working
}}

// not leaving this in because of the menu stuff
#[unity::hook("App", "Unit", "CreateImpl2")]
pub fn RecruitUnit(this: Unit, _method_info: u64)
{unsafe{
    call_original!(this, _method_info);

    if !this.as_ref().as_ptr().is_null()
    {
        // let force = this.get_force().get_force_type();
        // if force == Force_Type::player() || force == Force_Type::ally()
        // {
            // 0 = main menu
            // 3 = turn 1
            // 5 = post map
            // 7 = load map
            // let seq = GameUserData::get_instance().get_sequence();
            // if seq == 3 || seq == 5 || seq == 7
            // {
                if this.m_person().get_asset_force() != Force_Type::enemy()
                {
                    CheckForNewSkills(this, None, _method_info); //not sure this will work idk
                }
            // }
        // }
    }
}}

#[unity::hook("App", "LevelUpSequnece", "LearnJobSkill")]
pub fn LevelUpSeq(this: ProcInst, _method_info: u64)
{unsafe{
    call_original!(this, _method_info);

    let seq: LevelUpSequnece = this.cast();
    
    CheckForNewSkills(seq.m_unit(), Some(this), _method_info);
}}

pub fn CheckForNewSkills(unit: Unit, proc: Option<ProcInst>, _method_info: u64)
{unsafe{

    let name = unit.get_pid();

    for jsonBlob in LearnList.vals.iter_mut()
    {
        if jsonBlob.pid == name.to_string()
        {
            for skillItem in &jsonBlob.skills
            {
                if skillItem.0 <= (unit.m_level() + unit.m_internal_level() as u8).into()
                {
                    let skData = SkillData::get(skillItem.1.to_string().into());
                    if !skData.as_ref().as_ptr().is_null()
                    {
                        if !unit.is_exist_in_equip_skill_pool(skillItem.1.clone()) //&& IsInheritanceEnable(unit, sk, _method_info)
                        {
                            let mut x = 1;
                            let mut foundBigger = false;
                            let mut foundNext = false;
                            while !foundNext
                            {
                                let child = SkillData::try_get_2(skData.index() + x);
                                if !child.as_ref().as_ptr().is_null()
                                {
                                    if child.get_priority() != 0
                                    {
                                        
                                        if unit.is_exist_in_equip_skill_pool(child.get_sid())
                                        {
                                            foundBigger = true;
                                            foundNext = true;
                                        }
                                    }
                                    else
                                    {
                                        foundNext = true;
                                    }

                                }
                                else
                                {
                                    foundNext = true;
                                }
 
                                x += 1;
                            }
                            if !foundBigger
                            {
                                unit.add_to_equip_skill_pool(Il2CppString::new(skillItem.1.clone()));
                                if let Some(procInst) = proc
                                {
                                    ShowSkillPopup(skillItem.1.clone().into(), procInst);
                                    return; // can only do one skill per level up (if showing popup)
                                }
                            }

                        }
                    }
                    else
                    {
                        skyline::error::show_error(
                            69,
                            "Inherit new skill failed due to bad skill id.\n{}\0",
                            &skillItem.1.to_string()
                        );
                    }
                }
            }
        }
    }
}}

pub fn showError(str: String)
{
    skyline::error::show_error(
    69,
    "Inherit new skill failed due to bad skill id.\n{}\0",
    &str
);
}

pub fn ShowSkillPopup(string: Il2CppString, proc: ProcInst)
{
    let skill = SkillData::get(string);
    if !skill.as_ref().as_ptr().is_null()
    {
        let sid_substring = &skill.get_sid().to_string()[4..];
        let tag:Il2CppString = Mess::create_sprite_tag(Mess_IconCategory::skill(), sid_substring);
        // let val: &str = &("<sprite=\"Skill\" name=\"".to_owned()+sid_substring+"\">");
        // let tag: &Il2CppString = val.to_string().into();
        Mess::set_argument_2(0, tag);
        let skill_name= Mess::get(skill.get_name());
        Mess::set_argument_2(1,skill_name.to_string());
        let mut message= Mess::get("MID_Hub_Inheritance_Skill_Finish");
        GameMessage::create_key_wait(proc,message.to_string());
    }
}

pub fn LoadSkillJson()
{unsafe{
    match fs::read_dir("sd:/engage/mods/PersonalSkillGrowth/json")
    {
        Ok(paths) => {
            for path in paths
            {
                let p: &str = &read_to_string(path.unwrap().path()).unwrap();
                LearnList = json::from_str(p).expect("reason");
            }
        }
        Err(error) =>   {
            skyline::error::show_error(
            1,
            "Error Parsing Personal SKill Growth json folder.\n\0",
            "Do not rename any folders for this mod. \n\0",
            );
        }
    }
}
}


#[skyline::main(name = "PersonalSkillGrowth")]
pub fn main() {
    println!("Personal Skills Growth loaded");

    LoadSkillJson();

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Personal Skill Growth has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Personal Skill Growth has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    
    // old anna skills
    // install_hook!(LevelUp);
    // install_hook!(GetCapabilityGrow);

    install_hook!(RecruitUnit);
    install_hook!(LevelUpSeq);
    install_hook!(SupportConv);
    install_hook!(MyRoom);
    

    // Patch these address to nop parts of functions which are called if SP Cost of a skill = 0
    // Credit to @BadAtGames26
    let addresses = [0x01a35fa4, 0x01a36f34, 0x01a36588, 0x01a38b68, 0x01a38b68, 0x01a35ec8, 0x01a391e8];
    for address in addresses {
        let patch = Patch::in_text(address).nop();
        if patch.is_ok() {
            patch.unwrap();
            println!("Patched address {:x} with NOP", address);
        }
    }
}
