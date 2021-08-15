// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from ../../gir-files (@ 8e47c67)
// DO NOT EDIT

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
use std::fmt;

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
use bitflags::bitflags;
#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
use glib::translate::*;
#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
use nice_sys as ffi;

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
bitflags! {
    #[doc(alias = "NiceAgentOption")]
    pub struct AgentOption: u32 {
        #[doc(alias = "NICE_AGENT_OPTION_REGULAR_NOMINATION")]
        const REGULAR_NOMINATION = ffi::NICE_AGENT_OPTION_REGULAR_NOMINATION as u32;
        #[doc(alias = "NICE_AGENT_OPTION_RELIABLE")]
        const RELIABLE = ffi::NICE_AGENT_OPTION_RELIABLE as u32;
        #[doc(alias = "NICE_AGENT_OPTION_LITE_MODE")]
        const LITE_MODE = ffi::NICE_AGENT_OPTION_LITE_MODE as u32;
        #[doc(alias = "NICE_AGENT_OPTION_ICE_TRICKLE")]
        const ICE_TRICKLE = ffi::NICE_AGENT_OPTION_ICE_TRICKLE as u32;
        #[doc(alias = "NICE_AGENT_OPTION_SUPPORT_RENOMINATION")]
        const SUPPORT_RENOMINATION = ffi::NICE_AGENT_OPTION_SUPPORT_RENOMINATION as u32;
        #[doc(alias = "NICE_AGENT_OPTION_CONSENT_FRESHNESS")]
        const CONSENT_FRESHNESS = ffi::NICE_AGENT_OPTION_CONSENT_FRESHNESS as u32;
    }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
impl fmt::Display for AgentOption {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <Self as fmt::Debug>::fmt(self, f)
  }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl IntoGlib for AgentOption {
  type GlibType = ffi::NiceAgentOption;

  fn into_glib(self) -> ffi::NiceAgentOption {
    self.bits()
  }
}

#[cfg(any(feature = "v0_1_15", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
#[doc(hidden)]
impl FromGlib<ffi::NiceAgentOption> for AgentOption {
  unsafe fn from_glib(value: ffi::NiceAgentOption) -> Self {
    Self::from_bits_truncate(value)
  }
}