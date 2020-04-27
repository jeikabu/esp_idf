#![no_std]

pub mod error;
pub mod event;
pub mod mqtt;
pub mod wifi;

pub use esp_idf_sys as sys;

use core::convert::TryFrom;
use error::{Error, EspError};
use esp_idf_sys::*;

pub fn esp_int_into_result(value: i32) -> Result<(), Error> {
    if value == 0 {
        Ok(())
    } else if let Ok(error) = EspError::try_from(value) {
        Err(Error::EspIdf { error })
    } else {
        Err(Error::EnumFromIntError { value: value })
    }
}

pub mod nvs {
    use crate::*;
    pub fn init() -> Result<(), error::Error> {
        unsafe {
            let retval = nvs_flash_init() as u32;
            if retval == ESP_ERR_NVS_NO_FREE_PAGES || retval == ESP_ERR_NVS_NEW_VERSION_FOUND {
                let retval = nvs_flash_erase();
                esp_int_into_result(retval)?;
                let retval = nvs_flash_init();
                esp_int_into_result(retval)
            } else {
                esp_int_into_result(retval as i32)
            }
        }
    }
}

pub mod tcpip {
    use crate::*;
    pub fn init() {
        unsafe {
            tcpip_adapter_init();
        }
    }
}
