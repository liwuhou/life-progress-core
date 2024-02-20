use anyhow::Result;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use lifespan_crawler::{get_data, shave_round, CountryInfo};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Gender {
    Male,
    Female,
}

pub struct ProgressInfo {
    pub spent: u64,
    pub progress: f64,
    pub rest: u64,
    pub rest_progress: f64,
}

impl ProgressInfo {
    fn generate(spent_day: u64, gender: Option<Gender>, country_info: CountryInfo) -> Self {
        let CountryInfo { all, female, male } = country_info;
        let lifetime_year = match gender {
            Some(Gender::Male) => male,
            Some(Gender::Female) => female,
            _ => all,
        };

        let progress = shave_round(spent_day as f64 / (lifetime_year * 365.0), Some(4)) * 100.0;

        ProgressInfo {
            spent: spent_day,
            progress,
            rest_progress: 100.0 - progress,
            rest: (lifetime_year * 365.0 - spent_day as f64) as u64,
        }
    }
}

pub fn init(birthday: &str, gender: Option<Gender>, nation: Option<&str>) -> Result<ProgressInfo> {
    let birthday_time = get_birthday_time(birthday)?;
    get_progress_info(birthday_time, gender, nation)
}

pub fn get_progress_info(
    birthday: NaiveDate,
    gender: Option<Gender>,
    nation: Option<&str>,
) -> Result<ProgressInfo> {
    let now_time = Local::now().naive_utc();
    let rest_duration = now_time.date() - birthday;
    if rest_duration < Duration::zero() {
        panic!("Error: birthday must be '%Y-%m-%d', '%Y%m%d' or millis timestamp!");
    }
    let data = get_data()?;
    let country_info = get_life_time(nation, &data);

    Ok(ProgressInfo::generate(
        rest_duration.num_days() as u64,
        gender,
        country_info,
    ))
}

pub fn search_nation(search: &str) -> Result<Vec<((String, CountryInfo), (i64, Vec<usize>))>> {
    let search = search.trim();
    let nation_data = get_data()?;
    let nation_list: Vec<&String> = nation_data.keys().collect();
    let matcher = SkimMatcherV2::default();

    Ok(nation_list
        .iter()
        .filter_map(|nation| {
            matcher
                .fuzzy_indices(*nation, search)
                .map(|v| ((String::from(*nation), nation_data[*nation].clone()), v))
        })
        .collect())
}

pub fn view_nation(search: &str) -> Option<CountryInfo> {
    let data = get_data().ok()?;
    data.get(search).map(|info| info.clone())
}

pub fn get_birthday_time(birthday: &str) -> Result<NaiveDate> {
    if birthday.contains("-") {
        Ok(NaiveDate::parse_from_str(birthday, "%Y-%m-%d")?)
    } else if birthday.len() == 8 {
        Ok(NaiveDate::parse_from_str(birthday, "%Y%m%d")?)
    } else {
        let birthday_timestamp = birthday.parse::<i64>()?;
        if let Some(navie_date) = NaiveDateTime::from_timestamp_millis(birthday_timestamp) {
            Ok(navie_date.date())
        } else {
            panic!("Error: birthday must be '%Y-%m-%d', '%Y%m%d' or millis timestamp!");
        }
    }
}

fn get_life_time(country_name: Option<&str>, map: &HashMap<String, CountryInfo>) -> CountryInfo {
    if let Some(country_name) = country_name {
        if let Some(info) = map.get(country_name) {
            return info.clone();
        }
    }

    map.get("Common").unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_birthday_time() -> Result<()> {
        // %Y-%m-%d
        let test_time = "2024-01-17";
        let result = get_birthday_time("2024-01-17")?;
        assert_eq!(result, NaiveDate::parse_from_str(test_time, "%Y-%m-%d")?);

        // %Y%m%d
        let test_time = "20240117";
        let result = get_birthday_time(test_time)?;
        assert_eq!(result, NaiveDate::parse_from_str(test_time, "%Y%m%d")?);

        // timestamp
        let test_time: &str = "1705481218207";
        let result = get_birthday_time(test_time)?;
        assert_eq!(
            result,
            NaiveDateTime::from_timestamp_millis(test_time.parse::<i64>()?)
                .unwrap()
                .date()
        );
        Ok(())
    }
    #[test]
    fn test_init() -> Result<()> {
        let progress = init(
            "19941210",
            Some(Gender::Male),
            Some("People's Republic of China"),
        )?;

        println!(
            "You spent {} days, completed {}% of life progress, still have {} days left. enjoy!",
            progress.spent, progress.progress, progress.rest
        );

        Ok(())
    }
}

// You spent x days, completed x% of life progress, still have x days left. enjoy!
