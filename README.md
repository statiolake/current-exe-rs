# current-exe-rs
Rust library to get current executable's file.

## what are the differences between `std::env::current_exe()`?

This library reads symlinks on Windows.

If your binary needs access to the current executable's directory, you normally
use `std::env::current_exe()` function for that. However, this function doesn't
read symlinks, so if the command you called is a symlink, this function
misreport the symlink's directory. Therefore, previously you can't apply such
project a technique gathering symlinks into the one directory added to %PATH%
variable. If the number of such project grows, this leads to an incredible
number of paths in %PATH% variable.
