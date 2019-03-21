use std::{
    io::Error as IoError,
    fs::File,
};

struct VideoManagementInfoFiles {
    info_file: File,
    backup_file: File,
    /// If not present, indicates that there are no menus.
    vob_file: Option<File>,
}

impl VideoManagementInfoFiles {
    fn menus(&mut self) -> Result<Menus, IoError> {
        unimplemented!();
    }
}

pub struct Menus {

}

impl Iterator for Menus {
    type Item = Result<Menu, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}

struct AudioManagementInfoFiles {}

pub struct Menu {}

struct Title {}
struct Chapter {}

struct TitlesetFiles {
    titles: Vec<Title>,
    menus: Vec<Menu>,
}

pub struct DvdVideoMedia {
    /// AKA the `VMG`.
    video_management_info: VideoManagementInfoFiles,
    /// AKA the `VMG`.
    audio_management_info: Option<AudioManagementInfoFiles>,
    /// AKA the `VTS`.
    titlesets: Vec<TitlesetFiles>,
}
