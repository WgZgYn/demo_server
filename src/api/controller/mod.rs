mod area;
mod device;
mod house;
mod login;
mod signup;
mod template;

pub use self::login::login;
pub use self::signup::signup;
pub use area::add_area;
pub use device::{add_device, show_devices};
pub use house::add_house;
