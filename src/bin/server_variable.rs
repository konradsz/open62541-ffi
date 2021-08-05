use anyhow::{anyhow, Result};
use libc::c_void;
use open62541_ffi::open62541;
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

    let mut attr = unsafe { open62541::UA_VariableAttributes_default };
    let value_ptr = &mut attr.value as *mut open62541::UA_Variant;
    let my_integer = 42;
    let my_integer_ptr = &my_integer as *const _ as *const c_void;
    let my_type = unsafe { &open62541::UA_TYPES[open62541::UA_TYPES_INT32 as usize] };

    unsafe { open62541::UA_Variant_setScalarCopy(value_ptr, my_integer_ptr, my_type) };

    unsafe {
        attr.description = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0" as *const u8 as *const i8),
            text: open62541::UA_String_fromChars(b"the answer\0" as *const u8 as *const i8),
        };
        attr.displayName = open62541::UA_LocalizedText {
            locale: open62541::UA_String_fromChars(b"en-US\0" as *const u8 as *const i8),
            text: open62541::UA_String_fromChars(b"the answer\0" as *const u8 as *const i8),
        };
    }

    let my_integer_node_id = open62541::UA_NodeId {
        namespaceIndex: 1,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_STRING,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            string: unsafe {
                open62541::UA_String_fromChars(b"the.answer\0" as *const u8 as *const i8)
            },
        },
    };
    let my_integer_name = open62541::UA_QualifiedName {
        namespaceIndex: 1,
        name: unsafe { open62541::UA_String_fromChars(b"the answer\0" as *const u8 as *const i8) },
    };

    let parent_node_id = open62541::UA_NodeId {
        namespaceIndex: 0,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            numeric: open62541::UA_NS0ID_OBJECTSFOLDER,
        },
    };
    let parent_reference_node_id = open62541::UA_NodeId {
        namespaceIndex: 0,
        identifierType: open62541::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
        identifier: open62541::UA_NodeId__bindgen_ty_1 {
            numeric: open62541::UA_NS0ID_ORGANIZES,
        },
    };

    unsafe {
        open62541::__UA_Server_addNode(
            server,
            open62541::UA_NodeClass_UA_NODECLASS_VARIABLE,
            &my_integer_node_id as *const open62541::UA_NodeId,
            &parent_node_id as *const open62541::UA_NodeId,
            &parent_reference_node_id as *const open62541::UA_NodeId,
            my_integer_name,
            &open62541::UA_NODEID_NULL as *const open62541::UA_NodeId,
            std::mem::transmute(&attr as *const open62541::UA_VariableAttributes),
            &open62541::UA_TYPES[open62541::UA_TYPES_VARIABLEATTRIBUTES as usize],
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }

    unsafe {
        open62541::UA_clear(
            &attr as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_VARIABLEATTRIBUTES as usize],
        );
        open62541::UA_clear(
            &my_integer_node_id as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_NODEID as usize],
        );
        open62541::UA_clear(
            &my_integer_name as *const _ as *mut c_void,
            &open62541::UA_TYPES[open62541::UA_TYPES_QUALIFIEDNAME as usize],
        );
    }

    let retval = unsafe { open62541::UA_Server_run(server, running) };
    if retval != 0 {
        return Err(anyhow!("UA_Server_run returned {}", retval));
    }

    unsafe { open62541::UA_Server_delete(server) };

    Ok(())
}
