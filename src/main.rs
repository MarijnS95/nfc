use libc::*;

const NFC_SOCKPROTO_RAW: i32 = 0;
const NFC_SOCKPROTO_LLCP: i32 = 1;
const NFC_SOCKPROTO_MAX: i32 = 2;

const NFC_PROTO_NFC_DEP: u32 = 5;

#[repr(C)]
struct sockaddr_nfc {
    sa_family: sa_family_t,
    dev_idx: u32,
    target_idx: u32,
    nfc_protocol: u32,
}

fn main() -> Result<(), String> {
    let sock = unsafe { socket(AF_NFC, SOCK_SEQPACKET, NFC_SOCKPROTO_RAW) };
    if sock == -1 {
        return Err(format!("Failed to create sock {}", sock));
    }

    let idx = 0;

    let addr = sockaddr_nfc {
        sa_family: AF_NFC as _,
        dev_idx: idx,
        target_idx: 0, // TODO: Retrieved from netlink...
        nfc_protocol: NFC_PROTO_NFC_DEP,
    };

    let err = unsafe {
        connect(
            sock,
            &addr as *const _ as *const _,
            std::mem::size_of_val(&addr) as _,
        )
    };
    if err != 0 {
        let e = unsafe { *libc::__errno() };
        return Err(format!("Failed to connect {} {} {}", err, e, unsafe {
            std::ffi::CStr::from_ptr(libc::strerror(e))
                .to_str()
                .unwrap()
        }));
    }
    Ok(())
}
