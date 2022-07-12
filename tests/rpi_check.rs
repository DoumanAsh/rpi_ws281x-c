#[cfg(target_os = "linux")]
#[test]
fn should_detect_rpi_version() {
    use rpi_ws281x_c::PiInfo;

    let rpi = PiInfo::detect();
    //On non-RPI systems returns None
    assert!(rpi.is_none());
}
