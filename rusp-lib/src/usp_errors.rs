/// Gets an USP error message from the error code, returning an empty str for unknown codes
#[must_use]
pub const fn get_err_msg(code: u32) -> &'static str {
    match code {
        7000 => "Message failed",
        7001 => "Message not supported",
        7002 => "Request denied (no reason specified)",
        7003 => "Internal error",
        7004 => "Invalid arguments",
        7005 => "Resources exceeded",
        7006 => "Permission denied",
        7007 => "Invalid configuration",
        7008 => "Invalid path syntax",
        7009 => "Parameter action failed",
        7010 => "Unsupported parameter",
        7011 => "Invalid type",
        7012 => "Invalid value",
        7013 => "Attempt to update non-writeable parameter",
        7014 => "Value conflict",
        7015 => "Operation error",
        7016 => "Object does not exist",
        7017 => "Object could not be created",
        7018 => "Object is not a table",
        7019 => "Attempt to create non-creatable Object",
        7020 => "Object could not be updated",
        7021 => "Required parameter failed",
        7022 => "Command failure",
        7023 => "Command canceled",
        7024 => "Delete failure",
        7025 => "Object exists with duplicate key",
        7026 => "Invalid path",
        7027 => "Invalid command arguments",
        7028 => "Register failure",
        7029 => "Already in use",
        7030 => "Deregister failure",
        7031 => "Path already registered",
        7100 => "Record could not be parsed",
        7101 => "Secure session required",
        7102 => "Secure session not supported",
        7103 => "Segmentation and reassembly not supported",
        7104 => "Invalid Record value",
        7105 => "Session Context terminated",
        7106 => "Session Context not allowed",
        7800..=7999 => "Vendor specific",
        // Includes `7032..=7099 | 7107..=7799` too
        _ => "",
    }
}