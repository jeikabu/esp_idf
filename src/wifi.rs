use crate::*;

pub struct Config {
    pub ssid: &'static str,
    pub password: &'static str,
}

pub fn init(config: InitConfig) -> Result<(), error::Error> {
    let retval = unsafe { esp_wifi_init(&config.data) };
    esp_int_into_result(retval)
}

pub struct InitConfig {
    data: wifi_init_config_t,
}

impl Default for InitConfig {
    /// See WIFI_INIT_CONFIG_DEFAULT() in esp_wifi.h
    fn default() -> Self {
        let osi_funcs = unsafe { &mut g_wifi_osi_funcs };
        let wpa_crypto_funcs = unsafe { g_wifi_default_wpa_crypto_funcs };
        let data = wifi_init_config_t {
            event_handler: Some(esp_event_send),
            osi_funcs,
            wpa_crypto_funcs,
            static_rx_buf_num: CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM as i32,
            dynamic_rx_buf_num: CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM as i32,
            tx_buf_type: CONFIG_ESP32_WIFI_TX_BUFFER_TYPE as i32,
            static_tx_buf_num: WIFI_STATIC_TX_BUFFER_NUM as i32,
            dynamic_tx_buf_num: WIFI_DYNAMIC_TX_BUFFER_NUM as i32,
            csi_enable: WIFI_CSI_ENABLED as i32,
            ampdu_rx_enable: WIFI_AMPDU_RX_ENABLED as i32,
            ampdu_tx_enable: WIFI_AMPDU_TX_ENABLED as i32,
            nvs_enable: WIFI_NVS_ENABLED as i32,
            nano_enable: WIFI_NANO_FORMAT_ENABLED as i32,
            tx_ba_win: WIFI_DEFAULT_TX_BA_WIN as i32,
            rx_ba_win: WIFI_DEFAULT_RX_BA_WIN as i32,
            wifi_task_core_id: WIFI_TASK_CORE_ID as i32,
            beacon_max_len: WIFI_SOFTAP_BEACON_MAX_LEN as i32,
            mgmt_sbuf_num: WIFI_MGMT_SBUF_NUM as i32,
            magic: WIFI_INIT_CONFIG_MAGIC as i32,
        };
        Self { data }
    }
}

#[repr(u32)]
pub enum WifiMode {
    /// `WIFI_MODE_NULL`
    NULL = wifi_mode_t_WIFI_MODE_NULL,
    /// `WIFI_MODE_STA`
    STA = wifi_mode_t_WIFI_MODE_STA,
    /// `WIFI_MODE_AP`
    AP = wifi_mode_t_WIFI_MODE_AP,
    /// `WIFI_MODE_APSTA`
    APSTA = wifi_mode_t_WIFI_MODE_APSTA,
}
pub fn set_mode(mode: WifiMode) -> Result<(), error::Error> {
    let retval = unsafe { esp_wifi_set_mode(mode as u32) };
    esp_int_into_result(retval)
}

pub struct ApConfig {
    data: wifi_ap_config_t,
}

pub struct StaConfig {
    data: wifi_sta_config_t,
}

impl StaConfig {
    fn new() -> Self {
        Self {
            data: wifi_sta_config_t {
                ssid: Default::default(),
                password: [0; 64],
                scan_method: wifi_scan_method_t_WIFI_FAST_SCAN,
                bssid_set: false,
                bssid: Default::default(),
                channel: 0,
                listen_interval: 0,
                sort_method: wifi_sort_method_t_WIFI_CONNECT_AP_BY_SIGNAL,
                threshold: wifi_scan_threshold_t {
                    rssi: -127,
                    authmode: wifi_auth_mode_t_WIFI_AUTH_WPA2_PSK,
                },
            },
        }
    }

    pub fn from(config: &Config) -> Result<Self, error::Error> {
        let mut retval = Self::new();
        retval.set_ssid(config.ssid)?;
        retval.set_password(config.password)?;
        Ok(retval)
    }

    pub fn set_ssid(&mut self, ssid: &str) -> Result<(), error::Error> {
        copy_to_slice_nul(&mut self.data.ssid, ssid)
    }

    pub fn set_password(&mut self, password: &str) -> Result<(), error::Error> {
        copy_to_slice_nul(&mut self.data.password, password)
    }
}

fn copy_to_slice_nul(dest: &mut [u8], src: &str) -> Result<(), error::Error> {
    let src = src.as_bytes();
    let len = src.len();
    if len >= dest.len() {
        return Err(error::Error::InvalidArgument);
    }
    dest[..len].copy_from_slice(&src[..len]);
    dest[len] = 0; // nul-terminating 0
    Ok(())
}

#[repr(u32)]
pub enum Interface {
    /// `ESP_IF_WIFI_STA`
    WifiSta = esp_interface_t_ESP_IF_WIFI_STA,
    /// `ESP_IF_WIFI_AP`
    WifiAp = esp_interface_t_ESP_IF_WIFI_AP,
    /// `ESP_IF_ETH`
    Eth = esp_interface_t_ESP_IF_ETH,
}

pub fn set_ap_config(config: ApConfig) -> Result<(), error::Error> {
    let retval = unsafe {
        let mut conf = wifi_config_t { ap: config.data };
        esp_wifi_set_config(Interface::WifiAp as u32, &mut conf)
    };
    esp_int_into_result(retval)
}

pub fn set_sta_config(config: StaConfig) -> Result<(), error::Error> {
    let retval = unsafe {
        let mut conf = wifi_config_t { sta: config.data };
        esp_wifi_set_config(Interface::WifiSta as u32, &mut conf)
    };
    esp_int_into_result(retval)
}

pub fn start() -> Result<(), error::Error> {
    let retval = unsafe { esp_wifi_start() };
    esp_int_into_result(retval)
}

pub fn connect() -> Result<(), error::Error> {
    let retval = unsafe {
        esp_wifi_connect()
    };
    esp_int_into_result(retval)
}
