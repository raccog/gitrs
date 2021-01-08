/// A representation of file permissions.
#[derive(Debug)]
pub struct GitFileMode {
    file_type: u8,
    owner_mode: u8,
    group_mode: u8,
    other_mode: u8,
}

impl GitFileMode {
    /// The file type permission byte.
    pub fn file_type(&self) -> u8 {
        self.file_type
    }

    /// The owner mode permission byte.
    pub fn owner_mode(&self) -> u8 {
        self.owner_mode
    }

    /// The group mode permission byte.
    pub fn group_mode(&self) -> u8 {
        self.group_mode
    }

    /// The other mode permission byte.
    pub fn other_mode(&self) -> u8 {
        self.other_mode
    }
}

impl From<u32> for GitFileMode {
    fn from(filemode: u32) -> Self {
        let file_type = (filemode >> 14) as u8;
        let owner_mode = ((filemode & 0x0F00) >> 5) as u8;
        let group_mode = ((filemode & 0x00F0) >> 2) as u8;
        let other_mode = (filemode & 0x000F) as u8;

        Self {
            file_type,
            owner_mode,
            group_mode,
            other_mode,
        }
    }
}

impl Into<u32> for GitFileMode {
    fn into(self) -> u32 {
        let (t, own, grp, oth) = (
            self.file_type as u32,
            self.owner_mode as u32,
            self.group_mode as u32,
            self.other_mode as u32,
        );
        (t << 14) | (own << 5) | (grp << 2) | oth
    }
}

impl From<&str> for GitFileMode {
    fn from(filemode: &str) -> Self {
        assert_eq!(filemode.len(), 6);
        let file_type = filemode[..2].parse::<u8>().unwrap();
        let owner_mode = filemode[2..4].parse::<u8>().unwrap();
        let group_mode = filemode
            .chars()
            .nth(4)
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        let other_mode = filemode
            .chars()
            .nth(5)
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        Self {
            file_type,
            owner_mode,
            group_mode,
            other_mode,
        }
    }
}
