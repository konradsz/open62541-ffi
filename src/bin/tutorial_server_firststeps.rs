use anyhow::{anyhow, Result};
use open62541_ffi as open62541;
use signal_hook::iterator::Signals;
use std::{
    sync::{atomic::AtomicBool, Arc},
    thread,
};

fn main() -> Result<()> {
    let server = unsafe { open62541::UA_Server_new() };
    let config = unsafe { open62541::UA_Server_getConfig(server) };
    let retval = unsafe {
        open62541::UA_ServerConfig_setMinimalCustomBuffer(config, 4840, std::ptr::null(), 0, 0)
    };
    if retval != 0 {
        return Err(anyhow!(
            "UA_ServerConfig_setMinimalCustomBuffer returned {}",
            retval
        ));
    }

    let mut signals = Signals::new(&[signal_hook::consts::SIGINT, signal_hook::consts::SIGTERM])?;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    thread::spawn(move || {
        if let Some(_) = signals.into_iter().next() {
            running_clone.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    });
    let running = Arc::<AtomicBool>::as_ptr(&running).cast();

    let retval = unsafe { open62541::UA_Server_run(server, running) };
    if retval != 0 {
        return Err(anyhow!("UA_Server_run returned {}", retval));
    }

    unsafe { open62541::UA_Server_delete(server) };

    Ok(())
}
