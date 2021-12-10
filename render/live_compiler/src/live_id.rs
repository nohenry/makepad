#![allow(dead_code)]

use{
    std::{
        collections::HashMap,
        sync::Once,
        fmt,
        cmp::Ordering,
    }
};


impl LiveIdMap {
    pub fn add(&mut self, val: &str) {
        self.id_to_string.insert(LiveId::from_str_unchecked(val), val.to_string());
    }
    
    pub fn contains(&mut self, val: &str) -> bool {
        self.id_to_string.contains_key(&LiveId::from_str_unchecked(val))
    }
    
    pub fn with<F, R>(f: F) -> R
    where
    F: FnOnce(&mut Self) -> R,
    {
        static mut IDMAP: Option<LiveIdMap> = None;
        static ONCE: Once = Once::new();
        ONCE.call_once( || unsafe {
            let mut map = LiveIdMap {
                id_to_string: HashMap::new()
            };
            // pre-seed list for debugging purposes
            let fill = [
                "true",
                "false",
                "use",
                "!=",
                "!",
                "&&",
                "*=",
                "*",
                "+=",
                "+",
                ",",
                "-=",
                "->",
                "-",
                "..",
                ".",
                "/=",
                "/",
                "::",
                ":",
                ";",
                "<=",
                "<",
                "==",
                "=",
                ">=",
                "=>",
                ">",
                "?",
                "tracks",
                "state",
                "state_id",
                "user",
                "play",
                "ended"
            ];
            for item in &fill {
                map.add(item);
            }
            IDMAP = Some(map)
        });
        f(unsafe {IDMAP.as_mut().unwrap()})
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, Hash, PartialEq)]
pub struct LiveFileId(pub u16);

impl LiveFileId {
    pub fn index(index: usize) -> LiveFileId {LiveFileId(index as u16)}
    pub fn to_index(&self) -> usize {self.0 as usize}
}

//TODO FIX THIS THING TO BE N LEVELS OF MODULES
#[derive(Default, Clone, Eq, Hash, Debug, Copy, PartialEq)]
pub struct LiveModuleId(pub LiveId, pub LiveId);

impl LiveModuleId {
    pub fn from_str(module_path: &str) -> Result<Self,
    String> {
        // ok lets split off the first 2 things from module_path
        let bytes = module_path.as_bytes();
        let len = bytes.len();
        // we have to find the first :
        let mut crate_id = LiveId(0);
        let mut i = 0;
        while i < len {
            if bytes[i] == ':' as u8 {
                crate_id = LiveId::from_str(std::str::from_utf8(&bytes[0..i]).unwrap()) ?;
                i += 2;
                break
            }
            i += 1;
        }
        if i == len { // module_path is only one thing
            return Ok(LiveModuleId(LiveId(0), LiveId::from_str(std::str::from_utf8(&bytes[0..len]).unwrap()) ?));
        }
        return Ok(LiveModuleId(crate_id, LiveId::from_str(std::str::from_utf8(&bytes[i..len]).unwrap()) ?));
    }

}

impl fmt::Display for LiveModuleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}
/*
#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, Copy, PartialEq)]
pub struct LocalPtr(pub usize);
*/
#[derive(Clone, Debug, Eq, Hash, Copy, Ord, PartialOrd, PartialEq)]
pub struct LivePtr {
    pub file_id: LiveFileId,
    pub index: u32,
}

impl LivePtr{
    pub fn node_index(&self)->usize{
        self.index as usize
    }
    
    pub fn with_index(&self, index:usize)->Self{
        Self{file_id:self.file_id, index:index as u32}
    }

    pub fn from_index(file_id:LiveFileId, index:usize)->Self{
        Self{file_id, index:index as u32}
    }

}

impl fmt::Display for LivePtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.file_id.0, self.index)
    }
}

#[derive(Clone, Default, Eq, Hash, Copy, PartialEq)]
pub struct LiveId(pub u64);

impl LiveId {
    pub fn empty() -> Self {
        Self (0)
    }
    
    // doing this cuts the hashsize but yolo.
    pub fn with_num(&self, num:u16)->Self{
        Self(self.0&0xffff_ffff_ffff_0000 | (num as u64))
    }
    
    pub fn mask_num(&self)->Self{
        Self(self.0&0xffff_ffff_ffff_0000)
    }
    
    pub fn get_num(&self)->u16{
        (self.0&0xffff) as u16
    }
    
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    
    pub fn is_capitalised(&self) -> bool {
        self.0&0x8000_0000_0000_0000 != 0
    }
    
    
    // from https://nullprogram.com/blog/2018/07/31/
    // i have no idea what im doing with start value and finalisation.
    pub const fn from_bytes(id_bytes: &[u8], start: usize, end: usize) -> Self {
        //let id_len = id_bytes.len();
        let mut x = 0xd6e8_feb8_6659_fd93u64;
        let mut i = start;
        while i < end {
            x = x.overflowing_add(id_bytes[i] as u64).0;
            x ^= x >> 32;
            x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
            x ^= x >> 32;
            x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
            x ^= x >> 32;
            i += 1;
        }
        // use high bit to mark id as capitalised
        if id_bytes[0] >= 'A' as u8 && id_bytes[0] <= 'Z' as u8{
            return Self(x|0x8000_0000_0000_0000)
        }
        else{
            return Self(x&0x7fff_ffff_ffff_ffff)
        }
    }
    
    // merges 2 ids in a nonsymmetric fashion
    /*
    pub const fn add_id(&self, id: LiveId) -> Self {
        //let id_len = id_bytes.len();
        let mut x = id.0;
        x = x.overflowing_add(self.0).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
        x ^= x >> 32;
        return Self (x) // leave the first bit
    }*/
    
    pub const fn from_str_unchecked(id_str: &str) -> Self {
        let bytes = id_str.as_bytes();
        Self::from_bytes(bytes, 0, bytes.len())
    }
    
    pub fn from_str(id_str: &str) -> Result<Self, String> {
        let id = Self::from_str_unchecked(id_str);
        LiveIdMap::with( | idmap | {
            if let Some(stored) = idmap.id_to_string.get(&id) {
                if stored != id_str {
                    return Err(stored.clone())
                }
            }
            else {
                idmap.id_to_string.insert(id, id_str.to_string());
            }
            return Ok(id)
        })
    }
    
    pub fn as_string<F, R>(&self, f: F) -> R
    where F: FnOnce(Option<&String>) -> R
    {
        LiveIdMap::with( | idmap | {
            return f(idmap.id_to_string.get(self))
        })
    }
}

impl Ord for LiveId {
    fn cmp(&self, other: &LiveId) -> Ordering {
        LiveIdMap::with( | idmap | {
            if let Some(id1) = idmap.id_to_string.get(self) {
                if let Some(id2) = idmap.id_to_string.get(other) {
                    return id1.cmp(id2)
                }
            }
            return Ordering::Equal
        })
    }
}

impl PartialOrd for LiveId {
    fn partial_cmp(&self, other: &LiveId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl fmt::Debug for LiveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for LiveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == LiveId::empty() {
            write!(f, "0")
        }
        else {
            self.as_string( | string | {
                if let Some(id) = string {
                    write!(f, "{}", id)
                }
                else {
                    write!(f, "IdNotFound {:016x}", self.0)
                }
            })
        }
    }
}

impl fmt::LowerHex for LiveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


pub struct LiveIdMap {
    id_to_string: HashMap<LiveId, String>,
}

