use std::{
    env,
    ffi::CString,
    os::unix::ffi::OsStringExt,
    process,
    ptr,
};

pub mod wrapper;

fn main() {
    unsafe {
        // Nasty stuff to convert to C arguments
        let mut args = env::args_os().map(|arg_os| {
            CString::new(arg_os.into_vec()).unwrap().into_raw()
        }).collect::<Vec<_>>();
        let mut argc = args.len() as i32;
        args.push(ptr::null_mut());
        let mut argv = args.as_mut_ptr();

        // Allow mutter to handle arguments
        let ctx = meta_sys::meta_get_option_context();
        let mut error = ptr::null_mut();
        if glib_sys::g_option_context_parse(
            ctx,
            &mut argc,
            &mut argv,
            &mut error
        ) == glib_sys::GFALSE {
            glib_sys::g_printerr(
                b"%s: %s\n\0".as_ptr() as _,
                args[0],
                (*error).message
            );
            process::exit(1);
        }
        glib_sys::g_option_context_free(ctx);

        // Run mutter
        meta_sys::meta_plugin_manager_set_plugin_type(wrapper::cosmic_plugin_get_type());
        meta_sys::meta_set_wm_name(b"COSMIC\0".as_ptr() as _);
        meta_sys::meta_init();
        meta_sys::meta_register_with_session();
        process::exit(meta_sys::meta_run());
    }
}