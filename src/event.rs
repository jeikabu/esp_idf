use crate::*;
use core::convert::{self};

pub fn loop_create_default() -> Result<(), error::Error> {
    unsafe {
        let retval = esp_event_loop_create_default();
        esp_int_into_result(retval)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Ip(IpEvent),
    Wifi(WifiEvent),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventBase {
    Ip,
    Wifi,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IpEvent {
    StaGotIp = ip_event_t_IP_EVENT_STA_GOT_IP as i32,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WifiEvent {
    StaStart = wifi_event_t_WIFI_EVENT_STA_START as i32,
    StaDisconnected = wifi_event_t_WIFI_EVENT_STA_DISCONNECTED as i32,
    Any = ESP_EVENT_ANY_ID,
}

impl Event {
    // TODO: We implement TryFrom so we should automatically get TryInto,
    // but there's some compiler error.  Look into it later.
    fn try_into(&self) -> Result<(esp_event_base_t, i32), error::Error> {
        unsafe {
            match self {
                Self::Ip(event_id) => Ok((IP_EVENT, *event_id as i32)),
                Self::Wifi(event_id) => Ok((WIFI_EVENT, *event_id as i32)),
            }
        }
    }
}

impl convert::TryFrom<(esp_event_base_t, i32)> for Event {
    type Error = error::Error;

    fn try_from(value: (esp_event_base_t, i32)) -> Result<Self, Self::Error> {
        use Event::*;
        match EventBase::try_from(value.0) {
            Ok(EventBase::Ip) => Ok(Ip(IpEvent::try_from(value.1)?)),
            Ok(EventBase::Wifi) => Ok(Wifi(WifiEvent::try_from(value.1)?)),
            Err(_) => Err(error::Error::UnknownEventBase),
        }
    }
}

impl convert::TryFrom<esp_event_base_t> for EventBase {
    type Error = error::Error;

    fn try_from(value: esp_event_base_t) -> Result<Self, Self::Error> {
        unsafe {
            use EventBase::*;
            if value == IP_EVENT {
                Ok(Ip)
            } else if value == WIFI_EVENT {
                Ok(Wifi)
            } else {
                Err(error::Error::UnknownEventBase)
            }
        }
    }
}

impl convert::TryFrom<i32> for IpEvent {
    type Error = error::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value as u32 {
            ip_event_t_IP_EVENT_STA_GOT_IP => Ok(IpEvent::StaGotIp),
            _ => Err(Error::UnknownEventBase),
        }
    }
}

impl convert::TryFrom<i32> for WifiEvent {
    type Error = error::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(non_upper_case_globals)]
        match value as u32 {
            wifi_event_t_WIFI_EVENT_STA_START => Ok(WifiEvent::StaStart),
            wifi_event_t_WIFI_EVENT_STA_DISCONNECTED => Ok(WifiEvent::StaDisconnected),
            _ => Err(Error::UnknownEventBase),
        }
    }
}

pub mod events {
    use super::*;
    pub mod ip {
        use super::*;
        pub const StaGotIp: Event = Event::Ip(IpEvent::StaGotIp);
    }
    pub mod wifi {
        use super::*;
        pub const Any: Event = Event::Wifi(WifiEvent::Any);
    }
}

pub type EventHandler = unsafe extern "C" fn(
    event_handler_arg: *mut ::core::ffi::c_void,
    event_base: esp_event_base_t,
    event_id: i32,
    event_data: *mut ::core::ffi::c_void,
);

pub fn handler_register(event: Event, event_handler: EventHandler) -> Result<(), error::Error> {
    let (event_base, event_id) = event.try_into()?;
    let retval = unsafe {
        esp_event_handler_register(
            event_base,
            event_id,
            Some(event_handler),
            core::ptr::null_mut(),
        )
    };
    esp_int_into_result(retval)
}
