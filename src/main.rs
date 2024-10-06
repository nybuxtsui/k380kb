use std::env;
use hidapi::{DeviceInfo, HidApi, HidError, HidResult};

const TARGET_USAGE: u16 = 1;
const TARGET_USAGE_PAGE: u16 = 65280;
const K380_VID: u16 = 0x46d;
const K380_PID: u16 = 0xb342;
const K380_SEQ_FKEYS_ON: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];
const K380_SEQ_FKEYS_OFF: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x01, 0x00, 0x00];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd = &K380_SEQ_FKEYS_ON;
    if args.len() == 2 {
        if args[1] == "0" {
            cmd = &K380_SEQ_FKEYS_OFF;
        }
    }

    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices
    for device in api.device_list() {
        if device.vendor_id() == K380_VID
            && device.product_id() == K380_PID
            && device.usage() == TARGET_USAGE
            && device.usage_page() == TARGET_USAGE_PAGE {
            match write(&api, device, cmd) {
                Ok(_) => {
                    if cmd.eq(&K380_SEQ_FKEYS_ON) {
                        println!("设置成功: 激活FKYS");
                    } else {
                        println!("设置成功: 停用FKYS");
                    }
                    break;
                },
                Err(e) => {println!("HID接口出错: {}", e)}
            }
        }
    }
}

fn write(api: &HidApi, device: &DeviceInfo, cmd: &[u8]) -> HidResult<()> {
    let handle = api.open_path(device.path())?;
    let res = handle.write(cmd)?;
    if res != cmd.len() {
        return Err(HidError::HidApiError { message: "写入字节错误".into()});
    }
    Ok(())
}
