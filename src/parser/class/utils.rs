use log::{trace, debug};
use std::path::Path;

pub fn extract_addon_name(path: &Path) -> Option<String> {
    trace!("Extracting addon name from path: {:?}", path);
    let addon = path.components()
        .find(|c| {
            if let std::path::Component::Normal(name) = c {
                name.to_string_lossy().starts_with('@')
            } else {
                false
            }
        })
        .map(|c| c.as_os_str().to_string_lossy().into_owned());
    
    if let Some(ref name) = addon {
        debug!("Found addon name: {}", name);
    } else {
        debug!("No addon name found in path");
    }
    
    addon
}