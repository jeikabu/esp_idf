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

pub mod freertos {
    use crate::*;

    #[repr(u8)]
    enum QueueType {
        /// `BASE`
        Base				= 0,
        //SET					= 0,
        /// `MUTEX`
        Mutex 				= 1,
        /// `COUNTING_SEMAPHORE`
        CountingSemaphore	= 2,
        /// `BINARY_SEMAPHORE`
        BinarySemaphore	= 3,
        /// `RECURSIVE_MUTEX`
        RecursiveMutex		= 4,
    }

    enum QueueCopyPosition {
        /// `SEND_TO_BACK`
        SendToBack		= 0,
        /// `SEND_TO_FRONT`
        SendToFront		= 1,
        /// `OVERWRITE`
        Overwrite			= 2,
    }

    /// portMAX_DELAY
    pub const PORT_MAX_DELAY: TickType_t = !0;

    pub fn xQueueCreate(ux_queue_length: UBaseType_t, ux_item_size: UBaseType_t) -> QueueHandle_t {
        unsafe {
            xQueueGenericCreate(ux_queue_length, ux_item_size, QueueType::Base as u8)
        }
    }

    pub fn xQueueSendToBackFromISR(x_queue: QueueHandle_t, item_to_queue: *const core::ffi::c_void, higher_priority_task_woken: *mut BaseType_t) -> bool {
        unsafe {
            let retval = xQueueGenericSendFromISR(x_queue, item_to_queue, higher_priority_task_woken, QueueCopyPosition::SendToBack as BaseType_t);
            retval != 0
        }
    }

    pub fn xQueueReceive(x_queue: QueueHandle_t, buffer: *mut core::ffi::c_void, x_ticks_to_wait: TickType_t ) -> bool {
        unsafe {
            let retval = xQueueGenericReceive(x_queue, buffer, x_ticks_to_wait, false as BaseType_t);
            retval != 0
        }
    }
}

pub mod driver {
    // Sets the nth bit
    const fn bit(n: u64) -> u64 {
        0b01 << n
    }

    #[repr(u64)]
    pub enum GpioSel {
        Sel3  = bit(3),
        Sel4  = bit(4),
        Sel5  = bit(5),
        Sel6  = bit(6),
        Sel7  = bit(7),
        Sel8  = bit(8),
        Sel9  = bit(9),
        Sel10 = bit(10),
    }
}