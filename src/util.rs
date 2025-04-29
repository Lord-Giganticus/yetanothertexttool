use crate::font::*;
use crate::bmg::*;


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

