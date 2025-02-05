pub use super::{HwVersion, MetaDataV1, SwVersion, SIZE_METADATA_V1};

use core::mem::transmute;
use heapless::String;

#[derive(Clone)]
struct ImageInfo {
    file_name: String<12>,
    sw_version: SwVersion,
}

impl ImageInfo {
    pub const fn new() -> Self {
        ImageInfo {
            file_name: String::new(),
            sw_version: SwVersion {
                version: [0, 0, 0, 0],
            },
        }
    }

    pub fn file_name(&self) -> &String<12> {
        &self.file_name
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = String::<12>::new();
        let _ = self.file_name.push_str(file_name);
    }

    pub fn sw_version(&self) -> SwVersion {
        self.sw_version
    }

    pub fn set_sw_version(&mut self, sw_version: SwVersion) {
        self.sw_version = sw_version;
    }
}

#[derive(Clone)]
pub struct VersionCheck {
    image_info: ImageInfo,
    hw_version: HwVersion,
}

impl VersionCheck {
    pub fn new(hw_version: HwVersion, sw_version: SwVersion) -> Self {
        let mut image_info = ImageInfo::new();
        image_info.set_sw_version(sw_version);
        VersionCheck {
            image_info,
            hw_version,
        }
    }

    pub fn new_image_name(&self) -> Option<&String<12>> {
        if self.image_info.file_name().len() == 0 {
            None
        } else {
            Some(self.image_info.file_name())
        }
    }

    pub fn new_sw_version(&self) -> SwVersion {
        self.image_info.sw_version
    }

    pub fn analyse(&mut self, file_name: &str, meta_data: &[u8; SIZE_METADATA_V1]) {
        // we have to gnerate struct MetaDataV1 from binary stream, we check magic no so unsafe is ok
        let meta_data = unsafe { transmute::<&[u8; SIZE_METADATA_V1], &MetaDataV1>(meta_data) };
        if meta_data.magic != 0x1c80_73ab_2085_3579 {
            return;
        }
        if meta_data.meta_version != 1 {
            return;
        }
        if !meta_data.hw_version.is_compatible(&self.hw_version) {
            return;
        }
        if meta_data.sw_version <= self.image_info.sw_version() {
            return;
        }
        self.image_info.set_file_name(file_name);
        self.image_info.set_sw_version(meta_data.sw_version);
    }
}

#[cfg(test)]
mod tests {
    use super::{HwVersion, MetaDataV1, SwVersion, VersionCheck};
    const HW_VERSION: HwVersion = HwVersion::from_bytes([1, 3, 1, 0]);
    const SW_VERSION: SwVersion = SwVersion {
        version: [0, 0, 0, 0],
    };

    #[test]
    fn check_magic() {
        let mut meta_data = MetaDataV1::default();
        meta_data.magic = 0;
        meta_data.hw_version = HW_VERSION;
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 1]);

        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name(), None);
    }

    #[test]
    fn check_meta_version() {
        let mut meta_data = MetaDataV1::default();
        meta_data.meta_version = 0;
        meta_data.hw_version = HW_VERSION;
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 1]);

        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name(), None);
    }

    #[test]
    fn check_hw_version() {
        let mut meta_data = MetaDataV1::default();

        meta_data.hw_version = HwVersion::from_bytes([1, 3, 2, 0]);
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 1]);
        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name(), None);

        meta_data.hw_version = HwVersion::from_bytes([1, 3, 1, 1]);
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 1]);
        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test1.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name().unwrap(), "test1.bin");

        meta_data.hw_version = HW_VERSION;
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 2]);
        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test2.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name().unwrap(), "test2.bin");
    }

    #[test]
    fn check_sw_version() {
        let mut meta_data = MetaDataV1::default();

        meta_data.hw_version = HW_VERSION;
        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name(), None);

        meta_data.hw_version = HW_VERSION;
        meta_data.sw_version = SwVersion::from_bytes([1, 0, 0, 1]);
        let mut ulc = VersionCheck::new(HW_VERSION, SW_VERSION);
        ulc.analyse("test1.bin", meta_data.to_bytes());
        assert_eq!(ulc.new_image_name().unwrap(), "test1.bin");
    }
}
