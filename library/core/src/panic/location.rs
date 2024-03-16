use crate::fmt;
#[cfg(not(bootstrap))]
use crate::marker::PhantomData;

/// A struct containing information about the location of a panic.
///
/// This structure is created by [`PanicInfo::location()`].
///
/// [`PanicInfo::location()`]: crate::panic::PanicInfo::location
///
/// # Examples
///
/// ```should_panic
/// use std::panic;
///
/// panic::set_hook(Box::new(|panic_info| {
///     if let Some(location) = panic_info.location() {
///         println!("panic occurred in file '{}' at line {}", location.file(), location.line());
///     } else {
///         println!("panic occurred but can't get location information...");
///     }
/// }));
///
/// panic!("Normal panic");
/// ```
///
/// # Comparisons
///
/// Comparisons for equality and ordering are made in file, line, then column priority.
/// Files are compared as strings, not `Path`, which could be unexpected.
/// See [`Location::file`]'s documentation for more discussion.
#[lang = "panic_location"]
#[derive(Copy, Clone, Eq)]
#[cfg_attr(bootstrap, derive(Debug, PartialEq, PartialOrd, Ord, Hash))]
#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg_attr(not(doc), repr(C))]
pub struct Location<'a> {
    #[cfg(bootstrap)]
    file: &'a str,
    line: u32,
    col: u32,
    #[cfg(not(bootstrap))]
    length: u16,
    // The file path is stored inline to the &Location allocated by caller_location().
    // This avoids adding indirection to access the file path through another pointer, and
    // eliminates generating a relocation at compile-time for the file path.
    #[cfg(not(bootstrap))]
    file: [u8; 0],
    #[cfg(not(bootstrap))]
    marker: PhantomData<&'a str>,
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg(not(bootstrap))]
impl crate::fmt::Debug for Location<'_> {
    fn fmt(&self, f: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
        f.debug_struct("Location")
            .field("file", &self.file())
            .field("line", &self.line())
            .field("col", &self.column())
            .finish()
    }
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg(not(bootstrap))]
impl crate::cmp::PartialEq for Location<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.file() == other.file() && self.line == other.line && self.col == other.col
    }
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg(not(bootstrap))]
impl crate::cmp::PartialOrd for Location<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<crate::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg(not(bootstrap))]
impl crate::cmp::Ord for Location<'_> {
    fn cmp(&self, other: &Self) -> crate::cmp::Ordering {
        self.file()
            .cmp(&other.file())
            .then_with(|| self.line().cmp(&other.line()))
            .then_with(|| self.column().cmp(&other.column()))
    }
}

#[stable(feature = "panic_hooks", since = "1.10.0")]
#[cfg(not(bootstrap))]
impl crate::hash::Hash for Location<'_> {
    fn hash<H: crate::hash::Hasher>(&self, state: &mut H) {
        self.file().hash(state);
        self.line.hash(state);
        self.col.hash(state);
    }
}

impl<'a> Location<'a> {
    /// Returns the source location of the caller of this function. If that function's caller is
    /// annotated then its call location will be returned, and so on up the stack to the first call
    /// within a non-tracked function body.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::panic::Location;
    ///
    /// /// Returns the [`Location`] at which it is called.
    /// #[track_caller]
    /// fn get_caller_location() -> &'static Location<'static> {
    ///     Location::caller()
    /// }
    ///
    /// /// Returns a [`Location`] from within this function's definition.
    /// fn get_just_one_location() -> &'static Location<'static> {
    ///     get_caller_location()
    /// }
    ///
    /// let fixed_location = get_just_one_location();
    /// assert_eq!(fixed_location.file(), file!());
    /// assert_eq!(fixed_location.line(), 14);
    /// assert_eq!(fixed_location.column(), 5);
    ///
    /// // running the same untracked function in a different location gives us the same result
    /// let second_fixed_location = get_just_one_location();
    /// assert_eq!(fixed_location.file(), second_fixed_location.file());
    /// assert_eq!(fixed_location.line(), second_fixed_location.line());
    /// assert_eq!(fixed_location.column(), second_fixed_location.column());
    ///
    /// let this_location = get_caller_location();
    /// assert_eq!(this_location.file(), file!());
    /// assert_eq!(this_location.line(), 28);
    /// assert_eq!(this_location.column(), 21);
    ///
    /// // running the tracked function in a different location produces a different value
    /// let another_location = get_caller_location();
    /// assert_eq!(this_location.file(), another_location.file());
    /// assert_ne!(this_location.line(), another_location.line());
    /// assert_ne!(this_location.column(), another_location.column());
    /// ```
    #[must_use]
    #[stable(feature = "track_caller", since = "1.46.0")]
    #[rustc_const_unstable(feature = "const_caller_location", issue = "76156")]
    #[track_caller]
    #[inline]
    pub const fn caller() -> &'static Location<'static> {
        crate::intrinsics::caller_location()
    }

    /// Returns the name of the source file from which the panic originated.
    ///
    /// # `&str`, not `&Path`
    ///
    /// The returned name refers to a source path on the compiling system, but it isn't valid to
    /// represent this directly as a `&Path`. The compiled code may run on a different system with
    /// a different `Path` implementation than the system providing the contents and this library
    /// does not currently have a different "host path" type.
    ///
    /// The most surprising behavior occurs when "the same" file is reachable via multiple paths in
    /// the module system (usually using the `#[path = "..."]` attribute or similar), which can
    /// cause what appears to be identical code to return differing values from this function.
    ///
    /// # Cross-compilation
    ///
    /// This value is not suitable for passing to `Path::new` or similar constructors when the host
    /// platform and target platform differ.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occurred in file '{}'", location.file());
    ///     } else {
    ///         println!("panic occurred but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[must_use]
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    #[rustc_const_unstable(feature = "const_location_fields", issue = "102911")]
    #[inline]
    pub const fn file(&self) -> &str {
        #[cfg(bootstrap)]
        {
            self.file
        }

        #[cfg(not(bootstrap))]
        {
            unsafe {
                crate::str::from_raw_parts(
                    &self.file as *const _ as *const u8,
                    self.length as usize,
                )
            }
        }
    }

    /// Returns the line number from which the panic originated.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occurred at line {}", location.line());
    ///     } else {
    ///         println!("panic occurred but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[must_use]
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    #[rustc_const_unstable(feature = "const_location_fields", issue = "102911")]
    #[inline]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column from which the panic originated.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occurred at column {}", location.column());
    ///     } else {
    ///         println!("panic occurred but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[must_use]
    #[stable(feature = "panic_col", since = "1.25.0")]
    #[rustc_const_unstable(feature = "const_location_fields", issue = "102911")]
    #[inline]
    pub const fn column(&self) -> u32 {
        self.col
    }
}

#[stable(feature = "panic_hook_display", since = "1.26.0")]
impl fmt::Display for Location<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}:{}", self.file(), self.line, self.col)
    }
}
