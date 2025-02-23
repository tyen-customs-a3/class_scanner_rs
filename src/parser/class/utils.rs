use log::{trace, debug, info};
use std::path::Path;

pub fn extract_addon_name(path: &Path) -> Option<String> {
    info!("Extracting addon name from path: {:?}", path);
    trace!("Path components: {:?}", path.components().collect::<Vec<_>>());
    
    let addon = path.components()
        .find(|c| {
            if let std::path::Component::Normal(name) = c {
                let name_str = name.to_string_lossy();
                trace!("Checking component '{}' for addon prefix", name_str);
                name_str.starts_with('@')
            } else {
                trace!("Skipping non-normal component: {:?}", c);
                false
            }
        })
        .map(|c| c.as_os_str().to_string_lossy().into_owned());
    
    if let Some(ref name) = addon {
        debug!("Found addon name: {}", name);
    } else {
        debug!("No addon name found in path components");
    }
    
    addon
}