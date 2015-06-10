use raw;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use {Raw, Result, Processor};

/// A specification of a system on a chip.
pub struct Spec {
    raw: Raw<raw::ParseXML>,
    phantom: PhantomData<raw::ParseXML>,
}

impl Spec {
    /// Load a specification from an XML file.
    pub fn open(path: &Path) -> Result<Spec> {
        if !exists(path) {
            raise!(NotFound, format!("the file {:?} does not exist", path));
        }
        unsafe {
            let raw = not_null!(raw::new_ParseXML());
            raw::ParseXML_parse(raw, path_to_c_str!(path) as *mut _);
            Ok(Spec {
                raw: (raw, debug_not_null!(raw::ParseXML_sys(raw))),
                phantom: PhantomData,
            })
        }
    }

    /// Compute the system of a chip corresponding to the specification.
    pub fn processor<'l>(&'l self) -> Result<Processor<'l>> {
        let raw = unsafe { not_null!(raw::new_Processor(self.raw.0)) };
        Ok(::processor::from_raw((raw, self.raw.1)))
    }

    /// Return the raw specification.
    #[inline(always)]
    pub fn raw(&self) -> &raw::root_system {
        unsafe { &*self.raw.1 }
    }
}

impl Drop for Spec {
    #[inline]
    fn drop(&mut self) {
        unsafe { raw::delete_ParseXML(debug_not_null!(self.raw.0)) };
    }
}

#[inline]
fn exists(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => !metadata.is_dir(),
        Err(_) => false,
    }
}