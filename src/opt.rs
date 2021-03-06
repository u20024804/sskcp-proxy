use serde_urlencoded;
use serde_urlencoded::de::Error as DeError;
use serde_urlencoded::ser::Error as SerError;

#[derive(Default, Serialize, Deserialize)]
pub struct PluginOpts {
    pub plugin: Option<String>,
    pub plugin_opts: Option<String>,
    pub mtu: Option<usize>,
    pub nodelay: Option<bool>,
    pub interval: Option<i32>,
    pub resend: Option<i32>,
    pub nc: Option<bool>,
    pub rx_minrto: Option<u32>,
    pub fast_resend: Option<u32>,
}

impl PluginOpts {
    pub fn from_str(opt: &str) -> Result<PluginOpts, DeError> {
        serde_urlencoded::from_str(opt)
    }

    pub fn to_string(&self) -> Result<String, SerError> {
        serde_urlencoded::to_string(self)
    }

    pub fn has_kcp_config(&self) -> bool {
        self.mtu.is_some() || self.has_kcp_nodelay_config() || self.rx_minrto.is_some() || self.fast_resend.is_some()
    }

    pub fn has_kcp_nodelay_config(&self) -> bool {
        self.nodelay.is_some() || self.interval.is_some() || self.resend.is_some() || self.nc.is_some()
    }
}
