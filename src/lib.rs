use anyhow::Result;
use chrono::format::ParseErrorKind;
use chrono::{Local, NaiveDate, NaiveDateTime, ParseError};
use creeper::get_data;

pub enum Gender {
    Male,
    Female,
}

pub struct ProgressInfo {
    spent: u8,
    progress: f64,
    rest: u8,
}

impl ProgressInfo {
    fn generate(spend: f64, expectancy: f64) -> Self {
        todo!()
    }
}

pub fn init(brithday: &str, gender: Option<Gender>, nation: Option<&str>) -> Result<ProgressInfo> {
    let brithday_time = get_brithday_time(brithday)?;
    let now_time = Local::now().naive_utc();
    let data = get_data()?;
    todo!()
}

pub fn get_brithday_time(brithday: &str) -> Result<NaiveDate> {
    if brithday.contains("-") {
        Ok(NaiveDate::parse_from_str(brithday, "%Y-%m-%d")?)
    } else if brithday.len() == 8 {
        Ok(NaiveDate::parse_from_str(brithday, "%Y%m%d")?)
    } else {
        let brithday_timestamp = brithday.parse::<i64>()?;

        if let Some(navie_date) = NaiveDateTime::from_timestamp_millis(brithday_timestamp) {
            Ok(navie_date.date())
        } else {
            panic!("Error: brithday must be '%Y-%m-%d', '%Y%m%d' or millis timestamp!");
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_brithday_time() -> Result<()> {
        // %Y-%m-%d
        let test_time = "2024-01-17";
        let result = get_brithday_time("2024-01-17")?;
        assert_eq!(result, NaiveDate::parse_from_str(test_time, "%Y-%m-%d")?);

        // %Y%m%d
        let test_time = "20240117";
        let result = get_brithday_time(test_time)?;
        assert_eq!(result, NaiveDate::parse_from_str(test_time, "%Y%m%d")?);

        // timestamp
        let test_time: &str = "1705481218207";
        let result = get_brithday_time(test_time)?;
        assert_eq!(
            result,
            NaiveDateTime::from_timestamp_millis(test_time.parse::<i64>()?)
                .unwrap()
                .date()
        );
        Ok(())
    }
}

// You spent x days, completed x% of life progress, still have x days left. enjoy!
