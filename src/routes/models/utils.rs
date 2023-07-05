use chrono::{DateTime, Utc, Duration, NaiveDateTime};
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

pub fn format_relative_timespan(datetime: NaiveDateTime) -> String {
    let difference = Utc::now().naive_utc() - datetime;

    readable_duration(difference)
}

pub fn get_readable_duration(datetime: DateTime<Utc>) -> String {
    let difference = Utc::now() - datetime;

    readable_duration(difference)
}

fn readable_duration(difference: Duration) -> String {
    let minutes = difference.num_minutes();
    if minutes < 60 {
        return format!("{} minutes", minutes);
    }

    let hours = difference.num_hours();
    if hours < 24 {
        return format!("{} hours", hours);
    }

    let days = difference.num_days();
    if days < 7 {
        return format!("{} days", days);
    }

    let weeks = days / 7;
    if weeks < 4 {
        return format!("{} weeks", weeks);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{} months", months);
    }

    let years = months / 12;
    format!("{} years", years)
}
