mod open62541;

use anyhow::Result;
use signal_hook::iterator::Signals;
use std::{
    sync::{atomic::AtomicBool, Arc},
    thread,
};

fn main() -> Result<()> {
    let server = unsafe { open62541::UA_Server_new() };
    let config = unsafe { open62541::UA_Server_getConfig(server) };
    unsafe {
        open62541::UA_ServerConfig_setMinimalCustomBuffer(config, 4840, std::ptr::null(), 0, 0)
    };

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
    unsafe { open62541::UA_Server_delete(server) };

    println!("Retval: {}", retval);

    Ok(())
}
