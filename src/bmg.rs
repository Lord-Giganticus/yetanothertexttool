use std::io::SeekFrom;
use std::io::*;
use binrw::prelude::*;
use binrw::Endian;

#[derive(Debug, Default, Clone, Copy)]
pub struct FileHeader {
    pub magic: [u8; 8],
    pub flw1offset: u32,
    pub sectioncount: u32,
    pub padding: [u8; 16]
}

impl FileHeader {
    pub const BE_MAGIC: [u8; 8] = [b'M',b'E',b'S',b'G',b'b',b'm',b'g',b'1'];
    pub const LE_MAGIC: [u8; 8] = [b'G',b'S',b'E',b'M',b'1',b'g',b'm',b'b'];
    #[inline]
    pub fn read<R: BinReaderExt>(reader: &mut R) -> BinResult<(Self, Endian)> {
        let magic : [u8; 8] = reader.read_ne()?;
        let endian = match magic {
            Self::BE_MAGIC => Endian::Big,
            Self::LE_MAGIC => Endian::Little,
            _ => Endian::NATIVE
        };
        let flw1offset = reader.read_type(endian)?;
        let sectioncount = reader.read_type(endian)?;
        let padding = reader.read_ne()?;
        let res = Self {magic, flw1offset, sectioncount, padding};
        Ok((res, endian))
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(u32)]
pub enum SectionMagic {
    #[default]
    INF1 = 826691145,
    DAT1 = 827605316,
    FLW1 = 827804742,
    FLI1 = 826887238
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SectionHeader {
    pub magic: SectionMagic,
    pub size: u32
}

impl SectionHeader {
    pub fn read<R: BinReaderExt>(reader: &mut R, endian: Endian) -> BinResult<Self> {
        let mut result = Self::default();
        let SectionHeader { magic, size } = &mut result;
        let mgc = <u32>::read_le(reader)?;
        *magic = unsafe { std::mem::transmute(mgc) };
        *size = <u32>::read_options(reader, endian, ())?;
        Ok(result)
    }
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
#[brw(repr = u8)]
pub enum MessageType {
    #[default]
    Talk,
    Shout,
    Auto,
    Crash,
    Empty
}

#[derive(Debug, Default, Clone, Copy, BinWrite)]
#[brw(repr = u8)]
pub enum MessageBoxType {
    #[default]
    Normal,
    SignBoard = 4,
}

impl BinRead for MessageBoxType {
    type Args<'a> = ();
    fn read_options<R: Read + Seek>(
            reader: &mut R,
            _: Endian,
            _: Self::Args<'_>,
        ) -> BinResult<Self> {
        let byte = <u8>::read_ne(reader)?;
        match byte {
            0 => Ok(Self::Normal),
            4 => Ok(Self::SignBoard),
            _ => Ok(Self::Normal)
        }
    }
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
#[brw(repr = u8)]
pub enum CameraType {
    #[default]
    Normal,
    CameraId,
    NoCam
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
pub struct INF1Entry {
    pub textaddress: u32,
    pub cameraid: u16,
    pub soundid: u8,
    pub camtype: CameraType,
    pub messagetype: MessageType,
    pub messageboxtype: MessageBoxType,
    pub messageareaid: u8,
    pub padding: u8
}

#[derive(Debug, Clone, Default, BinRead, BinWrite)]
pub struct INF1 {
    pub entrynum: u16,
    pub entrysize: u16, // 0x12
    pub padding: u32,
    #[br(count = entrynum as usize)]
    pub entries: Vec<INF1Entry>
}

#[derive(Debug, Clone, Default)]
pub struct DAT1 {
    pub padding: u8,
    pub data: Vec<u8>,
}

impl DAT1 {
    pub fn read<R: BinReaderExt>(reader: &mut R, end: u64) -> BinResult<Self> {
        let mut result = Self::default();
        result.padding = reader.read_ne()?;
        let size = end - reader.stream_position()?;
        result.data.resize(size as usize, 0);
        reader.read_exact(&mut result.data)?;
        Ok(result)
    }
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
#[brw(repr = u8)]
#[repr(u8)]
pub enum FlowType {
    #[default]
    Text = 1,
    Condition,
    Event
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
pub struct EntryText {
    pub unk: u8,
    pub textid: u16,
    pub nexttextid: u16,
    pub validity: u8,
    pub unk2: u8
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
pub struct EntryCondition {
    pub unk: u8,
    pub conditiontype: u16,
    pub arg: u16,
    pub branchnodeid: u16
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
pub struct EntryEvent {
    pub event_type: u8,
    pub branchnodeid: u16,
    pub arg: u32
}

#[derive(Debug, Clone, Copy)]
pub enum FLW1Entry {
    Text(EntryText),
    Condition(EntryCondition),
    Event(EntryEvent)
}

impl Default for FLW1Entry {
    fn default() -> Self {
        Self::Text(Default::default())
    }
}

impl FLW1Entry {
    pub fn read<R: BinReaderExt>(reader: &mut R, endian: Endian) -> BinResult<Self> {
        let flow_type = FlowType::read_options(reader, endian, ())?;
        match flow_type {
            FlowType::Text => Ok(Self::Text(reader.read_type(endian)?)),
            FlowType::Condition => Ok(Self::Condition(reader.read_type(endian)?)),
            FlowType::Event => Ok(Self::Event(reader.read_type(endian)?))
        }
    }
    #[inline]
    pub const fn flow_type(&self) -> FlowType {
        match self {
            Self::Text(_) => FlowType::Text,
            Self::Condition(_) => FlowType::Condition,
            Self::Event(_) => FlowType::Event
        }
    }
    pub fn write<W: BinWriterExt>(&self, writer: &mut W, endian: Endian) -> BinResult<()> {
        let flow_type = self.flow_type();
        writer.write_type(&flow_type, endian)?;
        match self {
            Self::Text(t) => writer.write_type(t, endian)?,
            Self::Condition(c) => writer.write_type(c, endian)?,
            Self::Event(e) => writer.write_type(e, endian)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct FLW1 {
    pub nodenum: u16,
    pub branchnodenum: u16,
    pub padding: u32,
    pub entries: Vec<FLW1Entry>
}

impl FLW1 {
    pub fn read<R: BinReaderExt>(reader: &mut R, endian: Endian) -> BinResult<Self> {
        let mut result = Self::default();
        let FLW1 { nodenum, branchnodenum, padding, entries } = &mut result;
        *nodenum = reader.read_type(endian)?;
        *branchnodenum = reader.read_type(endian)?;
        *padding = reader.read_type(endian)?;
        entries.reserve_exact(*nodenum as usize);
        for _ in 0..*nodenum {
            entries.push(FLW1Entry::read(reader, endian)?);
        }
        Ok(result)
    }
}

#[derive(Debug, Default, Clone, Copy, BinRead, BinWrite)]
pub struct FLI1 {
    pub entrynum: u16,
    pub entrylength: u8,
    pub padding: [u8; 5]
}

#[derive(Debug, Clone)]
pub enum Section {
    INF1(INF1),
    DAT1(DAT1),
    FLW1(FLW1),
    FLI1(FLI1)
}

impl Default for Section {
    fn default() -> Self {
        Self::INF1(Default::default())
    }
}

impl Section {
    pub fn read<R: BinReaderExt>(reader: &mut R, endian: Endian) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        let section_header = SectionHeader::read(reader, endian)?;
        let mut end = pos + section_header.size as u64;
        end = end + ((end + 31 & !31) - end);
        let res = match section_header.magic {
            SectionMagic::INF1 => Self::INF1(reader.read_type(endian)?),
            SectionMagic::DAT1 => Self::DAT1(DAT1::read(reader, end)?),
            SectionMagic::FLW1 => Self::FLW1(FLW1::read(reader, endian)?),
            SectionMagic::FLI1 => Self::FLI1(reader.read_type(endian)?)
        };
        reader.seek(SeekFrom::Start(end))?;
        Ok(res)
    }
}

#[derive(Debug, Default, Clone)]
pub struct BMG {
    pub header: FileHeader,
    pub sections: Vec<Section>
}

impl BMG {
    pub fn read<R: BinReaderExt>(reader: &mut R) -> BinResult<Self> {
        let mut result = Self::default();
        let BMG {header, sections} = &mut result;
        let endian;
        (*header, endian) = FileHeader::read(reader)?;
        sections.reserve_exact(header.sectioncount as usize);
        for _ in 0..header.sectioncount {
            sections.push(Section::read(reader, endian)?);
        }
        Ok(result)
    }
    #[inline]
    pub fn get_inf1(&self) -> Option<&INF1> {
        for section in &self.sections {
            if let Section::INF1(inf1) = section {
                return Some(inf1);
            }
        }
        None
    }
    #[inline]
    pub fn get_dat1(&self) -> Option<&DAT1> {
        for section in &self.sections {
            if let Section::DAT1(dat1) = section {
                return Some(dat1);
            }
        }
        None
    }
    #[inline]
    pub fn get_flw1(&self) -> Option<&FLW1> {
        for section in &self.sections {
            if let Section::FLW1(flw1) = section {
                return Some(flw1);
            }
        }
        None
    }
    #[inline]
    pub fn get_inf1_mut(&mut self) -> Option<&mut INF1> {
        for section in &mut self.sections {
            if let Section::INF1(inf1) = section {
                return Some(inf1);
            }
        }
        None
    }
    #[inline]
    pub fn get_dat1_mut(&mut self) -> Option<&mut DAT1> {
        for section in &mut self.sections {
            if let Section::DAT1(dat1) = section {
                return Some(dat1);
            }
        }
        None
    }
    #[inline]
    pub fn get_flw1_mut(&mut self) -> Option<&mut FLW1> {
        for section in &mut self.sections {
            if let Section::FLW1(flw1) = section {
                return Some(flw1);
            }
        }
        None
    }

}