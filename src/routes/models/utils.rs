use shortguid::ShortGuid;
use uuid::Uuid;

pub fn get_readable_public_id(uuid_vec: &Vec<u8>) -> String {
    let mut bytes: [u8; 16] = [0; 16];
    let mut i = 0;
    for byte in uuid_vec {
        bytes[i] = *byte;
        i += 1;

        if i >= 16 {
            break;
        }
    }

    let public_uuid = Uuid::from_bytes(bytes);
    ShortGuid::from(public_uuid).to_string()
}
