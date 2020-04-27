use crate::*;

type MessageId = i32;

pub struct Client {
    handle: esp_mqtt_client_handle_t,
}

unsafe fn as_nul_terminated(string: &str, bytes: &mut [u8]) {
    if (string.len() + 1) > bytes.len() {
        panic!();
    }
    let len = core::cmp::min(string.len(), bytes.len()) - 1;
    for (dst, src) in bytes.iter_mut().take(len).zip(string.bytes().take(len)) {
        *dst = src;
    }
    bytes[len] = 0;
}

const STR_BUFFER_SIZE: usize = 128;
impl Client {
    pub fn init(config: &esp_mqtt_client_config_t) -> Result<(), Error> {
        Ok(())
    }

    pub fn start(&self) -> Result<(), Error> {
        let retval = unsafe { esp_mqtt_client_start(self.handle) };
        esp_int_into_result(retval)
    }

    // pub fn reconnect(&self) -> Result<(), EspError> {
    //     let retval = unsafe {
    //         idf::esp_mqtt_client_reconnect(self.handle)
    //     };
    //     esp_int_into_result(retval)
    // }

    // pub fn disconnect(&self) -> Result<(), EspError> {
    //     let retval = unsafe {
    //         idf::esp_mqtt_client_disconnect(self.handle)
    //     };
    //     esp_int_into_result(retval)
    // }

    pub fn stop(&self) -> Result<(), Error> {
        let retval = unsafe { esp_mqtt_client_stop(self.handle) };
        esp_int_into_result(retval)
    }

    pub fn subscribe(&self, topic: &str, qos: i32) -> Option<MessageId> {
        let retval = unsafe {
            let mut buffer = [0u8; STR_BUFFER_SIZE];
            as_nul_terminated(topic, &mut buffer);
            esp_mqtt_client_subscribe(self.handle, buffer.as_ptr() as *const _, qos)
        };
        if retval == -1 {
            None
        } else {
            Some(retval)
        }
    }

    pub fn unsubscribe(&self, topic: &str) -> Option<MessageId> {
        let retval = unsafe {
            let mut buffer = [0u8; STR_BUFFER_SIZE];
            as_nul_terminated(topic, &mut buffer);
            esp_mqtt_client_unsubscribe(self.handle, buffer.as_ptr() as *const _)
        };
        if retval == -1 {
            None
        } else {
            Some(retval)
        }
    }

    pub fn publish(&self, topic: &str, data: &[u8], qos: i32, retain: i32) -> Option<MessageId> {
        let retval = unsafe {
            let mut buffer = [0u8; STR_BUFFER_SIZE];
            as_nul_terminated(topic, &mut buffer);
            esp_mqtt_client_publish(
                self.handle,
                buffer.as_ptr() as *const _,
                data.as_ptr() as *const _,
                data.len() as i32,
                qos,
                retain,
            )
        };
        if retval == -1 {
            None
        } else {
            Some(retval)
        }
    }

    pub fn publish_empty(&self, topic: &str, qos: i32, retain: i32) -> Option<MessageId> {
        let retval = unsafe {
            let mut buffer = [0u8; STR_BUFFER_SIZE];
            as_nul_terminated(topic, &mut buffer);
            esp_mqtt_client_publish(
                self.handle,
                buffer.as_ptr() as *const _,
                core::ptr::null(),
                0,
                qos,
                retain,
            )
        };
        if retval == -1 {
            None
        } else {
            Some(retval)
        }
    }

    pub fn set_config(&self, config: &esp_mqtt_client_config_t) -> Result<(), Error> {
        let retval = unsafe { esp_mqtt_set_config(self.handle, config) };
        esp_int_into_result(retval)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _retval = unsafe { esp_mqtt_client_destroy(self.handle) };
    }
}
