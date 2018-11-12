use env; // This is causing the bug

include!(concat!(env!("NON_EXISTENT"), "/data.rs"));