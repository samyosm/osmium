use alloc::{boxed::Box, string::String};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono::{LocalResult, Utc};
use chrono_tz::America::Montreal;
use cmos_rtc::{ReadRTC, Time};

use crate::println;

use super::command::Command;

pub struct TimeCommand {}
impl Command for TimeCommand {
    fn parse(_: alloc::vec::Vec<&str>) -> alloc::boxed::Box<dyn Command>
    where
        Self: Sized,
    {
        Box::new(Self {})
    }

    fn execute(&self) {
        let mut cmos = ReadRTC::new(0x00, 0x00);
        let time: Time = cmos.read();

        let utc_time = Utc
            .with_ymd_and_hms(
                time.year as i32 + 2000,
                time.month as u32,
                time.day as u32,
                time.hour as u32,
                time.minute as u32,
                time.second as u32,
            )
            .single()
            .unwrap();

        let my_time = utc_time.with_timezone(&Montreal);

        println!("{my_time}");
    }

    fn name() -> String
    where
        Self: Sized,
    {
        String::from("time")
    }
    // add code here
}
