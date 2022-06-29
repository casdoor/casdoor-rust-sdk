use std::collections::BTreeMap;

use serde;
#[derive(Debug, serde::Serialize)]
pub struct NetWorkConfig {
    pub redirect_uri: String,
    pub state: String,
    pub response_type: String,
    pub scope: String,
}

impl NetWorkConfig {
    pub fn into_map(&self) -> BTreeMap<String, String> {
        let mut config = BTreeMap::new();
        config.insert("redirect_uri".to_owned(), self.redirect_uri.clone());
        config.insert("state".to_owned(), self.state.clone());
        config.insert("response_type".to_owned(), self.response_type.clone());
        config.insert("scope".to_owned(), self.scope.clone());
        config
    }
}

impl Default for NetWorkConfig {
    fn default() -> Self {
        NetWorkConfig {
            redirect_uri: "".to_owned(),
            state: "".to_owned(),
            response_type: "code".to_owned(),
            scope: "read".to_owned(),
        }
    }
}

/// The macro to generate a NetWorkConfig, used by get_auth_link method
///
/// Default response type is "code", and default scope is "read"
///
#[macro_export]
macro_rules! networkconfig {
	($redirect_uri:expr, $state:expr) => {
		$crate::networkconfig::NetWorkConfig {
			redirect_uri: $redirect_uri.to_string(),
			state: $state.to_string(),
			.. $crate::networkconfig::NetWorkConfig::default()
		}
	};
	($redirect_uri:expr, $state:expr, $($p:ident = $e:expr)?, *) => {
		$crate::networkconfig::NetWorkConfig {
			redirect_uri: $redirect_uri.to_string(),
			state: $state.to_string(),
			$(
				$p: $e.to_string(),
			)*
			.. $crate::networkconfig::NetWorkConfig::default()
		}
	}
}
