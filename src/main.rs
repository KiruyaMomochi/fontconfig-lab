use std::path::Path;

use similar::{self, ChangeTag, TextDiff};

fn main() {
    // Use the fontconfig C API.
    let ffi_dirs = ffi::dirs();
    // Parse the fonts.conf files directly.
    let parser_dirs = with_parser::dirs();

    let ffi_dirs = ffi_dirs.join("\n");
    let parser_dirs = parser_dirs.join("\n");

    println!("Comparing FFI...Parser:");

    let diff = TextDiff::from_lines(&ffi_dirs, &parser_dirs);
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete if Path::new(&change.to_string()).exists() => "-",
            ChangeTag::Delete => "x",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{} {}", sign, change);
    }
}

/// Use the fontconfig C API.
///
/// The following code are mainly based on [GitHub:yeslogic/fontconfig-rs]. But exposed the `FcConfigGetFontDirs` API.
///
/// References:
/// - [GitHub:neetly/figma-agent-linux](https://github.com/neetly/figma-agent-linux/blob/5e608c8216a3960a9e3554ddd19b37edb5ae0295/crates/fontconfig/src/config.rs#LL39C18-L39C18)
/// - [FcConfigGetFontDirs - Fontconfig Developer Reference](https://www.freedesktop.org/software/fontconfig/fontconfig-devel/fcconfiggetfontdirs.html)
/// - [GitHub:yeslogic/fontconfig-rs]
///
/// GitHub:yeslogic/fontconfig-rs]: https://github.com/yeslogic/fontconfig-rs
mod ffi {
    use fontconfig_sys as sys;
    use fontconfig_sys::ffi_dispatch;
    use std::{
        ffi::{c_char, CStr},
        marker::PhantomData,
    };

    use sys::FcBool;

    #[allow(non_upper_case_globals, dead_code)]
    const FcTrue: FcBool = 1;
    #[allow(non_upper_case_globals, dead_code)]
    const FcFalse: FcBool = 0;

    #[cfg(feature = "dlopen")]
    use sys::statics::{LIB, LIB_RESULT};
    #[cfg(not(feature = "dlopen"))]
    use sys::*;

    /// Wrapper around `FcStrList`
    ///
    /// The wrapper implements `Iterator` so it can be iterated directly, filtered etc.
    /// **Note:** Any entries in the `StrList` that are not valid UTF-8 will be skipped.
    ///
    /// ```
    /// use fontconfig::{Fontconfig, Pattern};
    ///
    /// let fc = Fontconfig::new().expect("unable to init Fontconfig");
    ///
    /// // Find fonts that support japanese
    /// let fonts = fontconfig::list_fonts(&Pattern::new(&fc), None);
    /// let ja_fonts: Vec<_> = fonts
    ///     .iter()
    ///     .filter(|p| p.lang_set().map_or(false, |mut langs| langs.any(|l| l == "ja")))
    ///     .collect();
    /// ```
    pub struct StrList<'a> {
        list: *mut sys::FcStrList,
        _life: PhantomData<&'a sys::FcStrList>,
    }

    impl<'a> StrList<'a> {
        unsafe fn from_raw(_: &Fontconfig, raw_list: *mut sys::FcStrSet) -> Self {
            Self {
                list: raw_list,
                _life: PhantomData,
            }
        }
    }

    impl<'a> Drop for StrList<'a> {
        fn drop(&mut self) {
            unsafe { ffi_dispatch!(LIB, FcStrListDone, self.list) };
        }
    }

    impl<'a> Iterator for StrList<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            let lang_str: *mut sys::FcChar8 =
                unsafe { ffi_dispatch!(LIB, FcStrListNext, self.list) };
            if lang_str.is_null() {
                None
            } else {
                match unsafe { CStr::from_ptr(lang_str as *const c_char) }.to_str() {
                    Ok(s) => Some(s),
                    _ => self.next(),
                }
            }
        }
    }

    /// Handle obtained after Fontconfig has been initialised.
    pub struct Fontconfig {
        config: *mut sys::FcConfig,
    }

    impl Fontconfig {
        /// Initialise Fontconfig and return a handle allowing further interaction with the API.
        ///
        /// If Fontconfig fails to initialise, returns `None`.
        pub fn new() -> Option<Self> {
            #[cfg(feature = "dlopen")]
            if LIB_RESULT.is_err() {
                return None;
            }
            let config = unsafe { ffi_dispatch!(LIB, FcInitLoadConfig,) };
            if !config.is_null() {
                // TODO: I'm not familar with Rust FFI, is this ok?
                Some(Fontconfig { config })
            } else {
                None
            }
        }

        pub fn font_dirs(&self) -> StrList {
            let dirs = unsafe { ffi_dispatch!(LIB, FcConfigGetFontDirs, self.config) };
            unsafe { StrList::from_raw(self, dirs) }
        }
    }

    impl Drop for Fontconfig {
        fn drop(&mut self) {
            unsafe { ffi_dispatch!(LIB, FcConfigDestroy, self.config) };
        }
    }

    pub fn dirs() -> Vec<String> {
        let fc = Fontconfig::new().unwrap();
        fc.font_dirs().map(|s| s.to_string()).collect()
    }
}

/// Parse the fonts.conf directly.
mod with_parser {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use fontconfig_parser::{Error, FontConfig};

    /// Get the home directory of the current user.
    ///
    /// This is equivalent to the $HOME environment variable.
    fn get_home() -> Result<String, std::env::VarError> {
        std::env::var("HOME")
    }

    /// Get $XDG_CONFIG_HOME/fontconfig.
    ///
    /// If $XDG_CONFIG_HOME is either not set or empty, a default equal to $HOME/.config should be used.
    fn get_config_home() -> Result<PathBuf, std::env::VarError> {
        std::env::var("XDG_CONFIG_HOME")
            .or(get_home().map(|home| format!("{}/.config", home)))
            .map(|config_home| Path::new(&config_home).join("fontconfig"))
    }

    /// Load all configuration files.
    ///
    /// See [fonts-conf(5)] for more information.
    ///
    /// [fonts-conf(5)]: https://www.freedesktop.org/software/fontconfig/fontconfig-user.html
    fn load_config() -> Result<FontConfig, Error> {
        let mut config = FontConfig::default();

        // FONTCONFIG_FILE is used to override the default configuration file
        if let Ok(ref path) = std::env::var("FONTCONFIG_FILE") {
            config.merge_config(path)?;
            return Ok(config);
        }

        // System configuration file
        // /etc/fonts/fonts.conf
        let config_file = Path::new("/etc/fonts/fonts.conf");
        if config_file.exists() {
            config.merge_config(&config_file)?;
        }

        // User configuration file
        // $XDG_CONFIG_HOME/fontconfig/fonts.conf
        if let Ok(config_dir) = get_config_home() {
            let config_file = config_dir.join("fonts.conf");
            if config_file.exists() {
                config.merge_config(&config_file)?;
            }
        }

        Ok(config)
    }

    /// Get the list of directories to search for fonts.
    pub fn dirs() -> Vec<String> {
        let config = load_config().unwrap();
        let dirs = config
            .dirs
            .iter()
            .map(|d| d.path.to_str().unwrap())
            .map(|d| {
                // Expand ~ to $HOME, may not work correctly if
                // - user trys to refer another user's home like ~user.
                // - user's home is /, which builds //
                // For better ways see
                // https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
                if d.starts_with('~') {
                    let mut home = std::env::var("HOME").unwrap();
                    home.push_str(d.strip_prefix('~').unwrap());
                    home
                } else {
                    d.to_string()
                }
            });

        let mut result = dirs.collect::<Vec<_>>();
        let mut uniques = HashSet::new();
        result.retain(|d| uniques.insert(d.to_string()));
        result
    }
}
