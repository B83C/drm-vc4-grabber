use drm_ffi::result::SystemError;
use drm_sys::{drm_mode_fb_cmd, drm_mode_fb_cmd2};
use drm_sys::*;
use std::os::unix::prelude::RawFd;

ioctl_readwrite!(drm_mode_getfb, DRM_IOCTL_BASE, 0xAD, drm_mode_fb_cmd);
ioctl_readwrite!(drm_mode_getfb2, DRM_IOCTL_BASE, 0xCE, drm_mode_fb_cmd2);


pub fn fb_cmd(fd: RawFd, handle: u32) -> Result<drm_mode_fb_cmd, SystemError> {
    let mut fb = drm_mode_fb_cmd {
        fb_id: handle,
        width: 0,
        height: 0,
        pitch: 0,
        bpp: 0,
        depth: 0,
        handle: 0,
    };

    unsafe {
        drm_mode_getfb(fd, &mut fb)?;
    }

    Ok(fb)
}

pub fn fb_cmd2(fd: RawFd, handle: u32) -> Result<drm_mode_fb_cmd2, SystemError> {
    let mut fb = drm_mode_fb_cmd2 {
        fb_id: handle,
        width: 0,
        height: 0,
        pixel_format: 0,
        flags: 0,
        handles: [0; 4],
        pitches: [0; 4],
        offsets: [0; 4],
        modifier: [0; 4],
    };

    unsafe {
        drm_mode_getfb2(fd, &mut fb)?;
    }

    Ok(fb)
}
