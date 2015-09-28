extern crate libc;

use libc::*;
use std::*;

pub static BLADERF_SERIAL_LENGTH: usize = 33;

#[repr(C)]
pub enum bladerf_backend {
    BLADERF_BACKEND_ANY,    
    BLADERF_BACKEND_LINUX,  
    BLADERF_BACKEND_LIBUSB, 
    BLADERF_BACKEND_CYPRESS, 
    BLADERF_BACKEND_DUMMY = 100,
}

pub struct Serial([uint8_t; 33]);

#[repr(C)]
pub struct bladerf_devinfo {
    pub backend: bladerf_backend,
    pub serial: Serial,
    pub usb_bus: uint8_t,            
    pub usb_addr: uint8_t,           
    pub instance: libc::c_uint
}

impl fmt::Debug for bladerf_devinfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serial: UNIMPLEMENTED, bus: {}, address: {})", self.usb_bus, self.usb_addr)
    }
}

#[link(name = "bladerf")]
extern {
	fn bladerf_get_device_list(devices: &*mut [bladerf_devinfo]) -> libc::c_int;
	fn bladerf_free_device_list(devices: *mut [bladerf_devinfo]);
    fn bladerf_set_usb_reset_on_open (enabled: bool);
}

pub fn get_device_list() -> Result<isize, isize> {

	unsafe{ 
		let devices: *mut [bladerf_devinfo] = mem::uninitialized();

		let n = bladerf_get_device_list(&devices) as isize;

		println!("Found {} device(s)", n);

		for i in 0..n {
			println!("{:?}", (*devices)[i]);
		}

		bladerf_free_device_list(devices);

		if n >= 0 {
			Ok(n)
		} else {
			Err(n)
		}
	}
}

pub fn set_usb_reset_on_open(enabled: bool) {
    unsafe{ 
    	bladerf_set_usb_reset_on_open(enabled); 
    } 
}

#[test]
fn discovery() {
	match get_device_list() {
		Ok(devices) => {
			println!("Discovered {} devices", devices);
		},
		Err(code) => {
			println!("Error {} listing devices", code);
			assert!(false);
		}
	}
}
