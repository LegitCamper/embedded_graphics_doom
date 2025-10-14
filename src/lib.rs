#![feature(generic_const_exprs)]
#![no_std]

use alloc::boxed::Box;
use embedded_graphics::{
    Pixel,
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point},
};
use oncelock::OnceCell;

extern crate alloc;

#[unsafe(no_mangle)]
static mut DG_ScreenBuffer: *const u8 = core::ptr::null();

static mut DRAW_CALLBACK: OnceCell<&'static (dyn Fn() + Sync)> = OnceCell::new();

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RGBA {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

unsafe extern "C" {
    fn D_DoomMain();
    fn doomgeneric_Tick();
    fn M_FindResponseFile();

    pub static colors: [RGBA; 256];

}

pub fn tick() {
    unsafe { doomgeneric_Tick() };
}

pub struct ScreenBuffer<const RESX: usize, const RESY: usize, const SIZE: usize>(pub [u8; SIZE]);

impl<const RESX: usize, const RESY: usize, const SIZE: usize> ScreenBuffer<RESX, RESY, SIZE> {
    const NA: () = assert!(SIZE == RESX * RESY);

    pub const fn new() -> Self {
        Self([0_u8; SIZE])
    }
}

#[unsafe(no_mangle)]
extern "C" fn DG_SetWindowTitle() {}

pub fn create<const RESX: usize, const RESY: usize, const SIZE: usize>(
    screenbuffer: &ScreenBuffer<RESX, RESY, SIZE>,
) {
    unsafe {
        M_FindResponseFile();

        DG_ScreenBuffer = screenbuffer.0.as_ptr();

        D_DoomMain();
    }
}
