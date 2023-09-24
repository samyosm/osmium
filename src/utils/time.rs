use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{America::Montreal, Tz};
use cmos_rtc::{ReadRTC, Time};

pub fn get_utc_datetime() -> DateTime<Utc> {
    let mut cmos = ReadRTC::new(0x00, 0x00);
    let time: Time = cmos.read();

    Utc.with_ymd_and_hms(
        time.year as i32 + 2000,
        time.month as u32,
        time.day as u32,
        time.hour as u32,
        time.minute as u32,
        time.second as u32,
    )
    .single()
    .unwrap()
}

pub fn get_mtl_datetime() -> DateTime<Tz> {
    get_utc_datetime().with_timezone(&Montreal)
}
