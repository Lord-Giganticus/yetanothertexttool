use crate::font::*;
use crate::bmg::*;
use std::fmt::Write;


impl INF1 {
    pub fn get_string(&self, index: usize, dat1: &DAT1) -> String {
        let address = self.entries[index].textaddress as usize;
        let data = &dat1.data[address..];
        let mut i = 0;
        let mut string = vec![0u16; 0];
        while i < data.len() {
            let pair = &data[i..(i + 2)];
            if pair == &[0, 0] {
                break;
            }
            if pair[0] == 0x1A {
                i += 1;
                i = dat1.create_event_text(data, &mut string, i);
            } 
            else if pair == &[0x26,0x6A] {
                string.push('\u{266A}' as u16);
                i += 2;
            }
            else {
                string.push(u16::from_ne_bytes(pair.try_into().unwrap_or_default()));
                i += 2;
            }
        }
        String::from_utf16_lossy(&string)
    }
}

impl INF1Entry {
    pub fn get_info(&self) -> String  {
        let messagetype = self.messagetype;
        let messageboxtype = self.messageboxtype;
        let soundid = SoundId::as_string(self.soundid);
        let camtype = self.camtype;
        format!("[type:{messagetype:?}][boxtype:{messageboxtype:?}][sound:{soundid}][cam:{camtype:?}]")
    }
    pub fn has_text(&self, dat1: &DAT1) -> bool {
        let addr = self.textaddress as usize;
        let first = &dat1.data[addr..(addr+2)];
        first != &[0, 0]
    }
    
}

impl DAT1 {
    pub(self) fn create_event_text(&self, data: &[u8], string: &mut Vec<u16>, position: usize) -> usize {
        let entrysize = data[position] as usize - 1;
        let entrytype = data[position + 1];
        let entryvalue = &data[(position+2)..(position+4)];
        let entryvalue = u16::from_be_bytes(entryvalue.try_into().unwrap_or_default());
        match entrytype {
            1 => {
                match entryvalue {
                    0 => {
                        let waittime = data[position+4];
                        let mut waittime = format!("[waittime:{waittime}]").encode_utf16()
                        .collect::<Vec<_>>();
                        string.append(&mut waittime);
                    },
                    1 => {
                        let mut newline = "[newline]".encode_utf16().collect::<Vec<_>>();
                        string.append(&mut newline);
                    },
                    3 => {
                        let mut center = "[center]".encode_utf16().collect::<Vec<_>>();
                        string.append(&mut center);
                    }
                    _ => {}
                }
            },
            2 => {
                let string_size = entrysize - 5;
                let mut sound = "[sound:".encode_utf16().collect::<Vec<_>>();
                let mut sound_bytes = data[(position + 5)..(position + 5 + string_size)]
                .chunks(2)
                .map(|x| u16::from_ne_bytes(x.try_into().unwrap_or_default()))
                .collect::<Vec<_>>();
                sound.append(&mut sound_bytes);
                sound.push(b']'.into());
                string.append(&mut sound);
            },
            3 => {
                let pic = PicIcon::new(entryvalue);
                if let Some(pic) = pic {
                    let mut pic = pic.as_utf16();
                    string.append(&mut pic);
                } else {
                    let mut pic = format!("{entryvalue}").encode_utf16()
                    .collect::<Vec<_>>();
                    string.append(&mut pic);
                }
            },
            4 => {
                let mut size = match entryvalue {
                    0 => String::from("[fontsize:small]"),
                    1 => String::from("[fontsize:normal]"),
                    2 => String::from("[fontsize:large]"),
                    _ => format!("[fontsize:{entryvalue}]")
                }.encode_utf16().collect::<Vec<_>>();
                string.append(&mut size);
            },
            5 => {
                if entryvalue == 0 {
                    let nametype = data[position + 4];
                    let mut nametype = match nametype {
                        0 => String::from("[playername:normal]"),
                        1 => String::from("[playername:formal]"),
                        2 => String::from("[playername:moustache]"),
                        _ => format!("[playername:{nametype}]")
                    }.encode_utf16().collect::<Vec<_>>();
                    string.append(&mut nametype);
                }
            }
            6 => {
                let arg2 = &data[(position + 4)..(position + 8)];
                let arg3 = &data[(position + 8)..(position + 12)];
                let arg2 = u32::from_be_bytes(arg2.try_into().unwrap_or_default());
                let arg3 = u32::from_be_bytes(arg3.try_into().unwrap_or_default());
                let value = format!("[valint:{},{},{}]", entryvalue, arg2, arg3);
                let mut value = value.encode_utf16().collect::<Vec<_>>();
                string.append(&mut value);
            },
            7 => {
                let arg2 = &data[(position + 4)..(position + 8)];
                let arg3 = &data[(position + 8)..(position + 12)];
                let arg2 = u32::from_be_bytes(arg2.try_into().unwrap_or_default());
                let arg3 = u32::from_be_bytes(arg3.try_into().unwrap_or_default());
                let value = format!("[valstr:{},{},{}]", entryvalue, arg2, arg3);
                let mut value = value.encode_utf16().collect::<Vec<_>>();
                string.append(&mut value);
            },
            255 => {
                if entryvalue == 0 {
                    let colorval = data[position + 4];
                    let color = Color::new(colorval);
                    if let Some(color) = color {
                        let mut color = color.as_utf16();
                        string.append(&mut color);
                    } else {
                        let mut color = format!("{colorval}").encode_utf16()
                        .collect::<Vec<_>>();
                        string.append(&mut color);
                    }
                }
            },
            _ => {}
        }
        position + entrysize
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum MultipleChoice {
    PenguinRace, 
    SwimmingSchool, 
    PenguinRaceAlt, 
    BombTimeAttackLv1, 
    PhantomTeresaRacer, 
    BombTimeAttackLv2, 
    TrialSurfingCoach, 
    TrialSurfingHowTo, 
    DeathPromenadeTeresaRacer, 
    RosettaFinalBattle, 
    CometTico, 
    TransformTico, 
    ChallengeSurfingCoach, 
    TicoShopExchange, 
    TicoShopWhich, 
    KinopioPurple, 
    CometTicoTell, 
    TrialTamakoroHowTo, 
    KnockOnTheDoor, 
    LedPattern
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum ConditionType {
    MultipleChoice,   
    Coded,    
    PlayerNearNpc,  
    SwA,   
    SwB,   
    PlayerStateNoPowerUp,    
    PlayerStateBee, 
    PlayerStateBoo, 
    PowerStarSpawned,   
    AlreadyTalkedScene,  
    PlayerLuigi,   
    GetBranchAstroGalaxyResult,   
    CutsceneActive,    
    AlreadyTalkedSaved,    
    IsMsgLedPattern
}

impl EntryCondition {
    pub fn get_choice(&self) -> String {
        if self.arg <= 19 {
            let choice: MultipleChoice = unsafe{std::mem::transmute(self.arg)};
            format!("{choice:?}")
        } else {
            self.arg.to_string()
        }
    }
    pub fn get_con_type(&self) -> String {
        if self.conditiontype <= 14 {
            let con : ConditionType = unsafe {std::mem::transmute(self.conditiontype)};
            format!("{con:?}")
        } else {
            self.conditiontype.to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EventType {
    NpcEvent, 
    NpcEventAlt, 
    NextText, 
    Unk, 
    Emotion, 
    SwA, 
    SwB, 
    Metamorphosis
}

impl EntryEvent {
    pub fn get_event_type(&self) -> String {
        if self.event_type < 7 {
            let event: EventType = unsafe{std::mem::transmute(self.event_type)};
            format!("{event:?}")
        } else {
            self.event_type.to_string()
        }
    }
}

impl FLW1 {
    pub fn has_flow(&self, id: u16) -> bool {
        'outer: for i in 0..self.nodenum {
            let flow = self.entries[i as usize];
            if let FLW1Entry::Text(text) = flow {
                if id == text.textid {
                    for c in 0..self.nodenum {
                        if c != i {
                            let cflow = self.entries[c as usize];
                            match cflow {
                                FLW1Entry::Text(t) => {
                                    if t.nexttextid == i {
                                        continue 'outer;
                                    }
                                }
                                FLW1Entry::Condition(con) => {
                                    let branch = self.branch_nodes[con.branchnodeid as usize];
                                    let next_branch = self.branch_nodes[con.branchnodeid as usize + 1];
                                    if branch == i || next_branch == i {
                                        continue 'outer;
                                    }
                                },
                                FLW1Entry::Event(eve) => {
                                    let branch = self.branch_nodes[eve.branchnodeid as usize];
                                    if branch == i {
                                        continue 'outer;
                                    }
                                }
                            }
                        }
                    }
                    return true;
                }
            }
        }
        false
    }
    pub fn findnode(&self, id: u16, converted: &mut Vec<bool>) -> Result<String, std::fmt::Error> {
        let mut result = String::new();
        for i in 0..self.nodenum {
            let node = self.entries[i as usize];
            if let FLW1Entry::Text(text) = node {
                if text.textid == id {
                    self.write_flow(i, node, &mut result, converted)?;
                    break;
                }
            }
        }
        Ok(result)
    }
    pub(self) fn write_flow(&self, id: u16, node: FLW1Entry, result: &mut String, converted: &mut Vec<bool>) -> std::fmt::Result {
        if converted[id as usize] {
            return Ok(());
        }
        converted[id as usize] = true;
        write!(result, "[node:{id}][type:")?;
        match node {
            FLW1Entry::Text(text) => {
                let mid = text.textid;
                write!(result, "text][messageid:{mid}][next:")?;
                if text.nexttextid == u16::MAX {
                    writeln!(result, "none]")?;
                } else {
                    let next = text.nexttextid;
                    writeln!(result, "{next}]")?;
                    let next_node = self.entries[next as usize];
                    self.write_flow(next, next_node, result, converted)?;
                }
            },
            FLW1Entry::Condition(con) => {
                write!(result, "condition][type:{}]", con.get_con_type())?;
                if con.conditiontype == 0 {
                    write!(result, "[choice:{}]", con.get_choice())?;
                } else {
                    write!(result, "[arg:{}]", con.arg)?;
                }
                write!(result, "[trueflow:")?;
                let true_id = self.branch_nodes[con.branchnodeid as usize];
                if true_id == u16::MAX {
                    write!(result, "none]")?;
                } else {
                    write!(result, "{true_id}]")?;
                }
                write!(result, "[falseflow:")?;
                let false_id = self.branch_nodes[con.branchnodeid as usize + 1];
                if false_id == u16::MAX {
                    write!(result, "none]")?;
                } else {
                    write!(result, "{false_id}]")?;
                }
                writeln!(result)?;
                if (0..self.nodenum).contains(&true_id) {
                    let true_node = self.entries[true_id as usize];
                    self.write_flow(true_id, true_node, result, converted)?;
                }
                if (0..self.nodenum).contains(&false_id) {
                    let false_node = self.entries[false_id as usize];
                    self.write_flow(false_id, false_node, result, converted)?;
                }
            },
            FLW1Entry::Event(eve) => {
                write!(result, "event][type:{}]", eve.get_event_type())?;
                write!(result, "[arg:{}][next:", eve.arg)?;
                let next = eve.branchnodeid;
                if next == u16::MAX {
                    writeln!(result, "none]")?;
                } else {
                    writeln!(result, "{next}]")?;
                }
                if (0..self.nodenum).contains(&next) {
                    let next_node = self.entries[next as usize];
                    self.write_flow(next, next_node, result, converted)?;
                }
            }
        }
        Ok(())
    }
}