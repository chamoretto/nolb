use serde_json::json;
use std::cmp::Ordering;

// TODO: чтобы включить работу док-тестов, нужно вынести в крейт с таргетом lib. В кору нести, видимо.
//  Либо поделить этот крейт на lib с кодом и main.rs для запуска
/// Produces multiple strings from a given list of possible values which are similar
/// to the passed in value `v` within a certain confidence by least confidence.
///
/// Example when suggest may be positive:
/// ```
/// const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
/// let bad_input = "comfig"; // let's imagine that the input comes from user
///
/// let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
/// assert_eq!(Some(&"--config".to_string()), suggest.get(0))
/// ```
///
/// Example when suggest may be negative
/// ```
/// const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
/// let bad_input = "something"; // let's imagine that the input comes from user
///
/// let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
/// assert_eq!(None, suggest.get(0))
/// ```
pub(crate) fn calculate_suggestions<T, I>(v: &str, possible_values: I) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    let mut candidates: Vec<(f64, String)> = possible_values
        .into_iter()
        .map(|pv| (strsim::jaro_winkler(v, pv.as_ref()), pv.as_ref().to_owned()))
        .filter(|(confidence, _)| confidence > &0.8)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_, pv)| pv).collect()
}

#[allow(unused_imports)]
// #[cfg(tests)]
mod tests {
    use crate::utility::suggestions::calculate_suggestions;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suggestion_hit() {
        const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
        let bad_input = "comfig"; // let's imagine that the input comes from user

        let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
        assert_eq!(Some(&"--config".to_string()), suggest.get(0))
    }

    #[test]
    fn test_suggestion_miss() {
        const GOOD_VARIANTS: [&str; 2] = ["--config", "--help"];
        let bad_input = "something"; // let's imagine that the input comes from user

        let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
        assert_eq!(None, suggest.get(0))
    }

    #[test]
    fn test_suggestion_da_net() {
        use crate::config::agent::Factors::{One, Three, Two};
        use either::Either;
        use serde::de::DeserializeOwned;
        use serde::{Deserialize, Deserializer, Serialize};
        use serde_json::json;
        use std::collections::HashMap;
        use std::str::FromStr;
        use toml::Value::Integer;

        // const GOOD_VARIANTS: [&str; 2] = ["да", "нет"];
        // let bad_input = "нет, да"; // let's imagine that the input comes from user
        //
        // let suggest = calculate_suggestions(bad_input, GOOD_VARIANTS);
        // assert_eq!(Some(&"да".to_string()), suggest.get(0))

        // value_to_set: None | str | int | float | DateTimePeriod | TimePeriod | CalendarDayPeriod | \
        //         datetime.datetime | datetime.time | list[int | str | float] | dict = None

        // /// Содержит в себе произвольные данные, притом нулевой вложенности.
        // /// Вложенные данные принудительно становятся плоскими, а ключом становится полный путь до них:
        // /// {"something": {"inner": 1}} => {"something.inner": 1}
        // pub struct AnyObject {
        //     inner_storage: HashMap<String, Either<ScenarioVariableValue, Vec<ScenarioVariableValue>>>,
        // }
        //
        // impl AnyObject {
        //     pub fn try_new<D: Serialize>(object: D) -> Self {
        //         let json = serde_json::to_value(&object);
        //     }
        // }
        //
        // impl<'de> Deserialize<'de> for AnyObject {
        //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        //     where
        //         D: Deserializer<'de>,
        //     {
        //         todo!()
        //     }
        // }

        #[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
        #[serde(untagged)]
        pub enum ScenarioVariableValue {
            #[default]
            Null,
            Integer(i64),
            Float(f64),
            // DateTimePeriod(DateTimePeriod),
            // TimePeriod(TimePeriod),
            // CalendarDayPeriod(CalendarDayPeriod),
            DateTime(chrono::NaiveDateTime),
            Time(chrono::NaiveTime),
            ListInt(Vec<i64>),
            ListFloat(Vec<f64>),
            ListAnyObject(Vec<ScenarioVariableValue>),
            AnyObject(HashMap<String, ScenarioVariableValue>),
            ListString(Vec<String>),
            String(String),
        }
        use serde_json::Value;

        let tst = serde_json::from_str::<Value>("213")
            .expect("всё норм")
            .pointer("alala/f");
        let xml_value = quick_xml::Reader::from_str("213");
        let xml_value2 = serde_xml_rs::from_str("123");

        let sample = serde_json::to_string(&serde_json::json!(null)).expect("всё норм");
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample.as_ref()).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = serde_json::to_string(&serde_json::json!("hello")).expect("всё норм");
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample.as_ref()).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = serde_json::to_string(&serde_json::json!(10_i64)).expect("всё норм");
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample.as_ref()).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = serde_json::to_string(&serde_json::json!(7.5_f64)).expect("всё норм");
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample.as_ref()).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = "\"2022-06-02T08:15:08.468123\"";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = "\"08:15:08.468654\"";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = "[123, 456, 789]";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = "[8.7, 9.3]";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = "[\"test\", \"тест\"]";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:?}", parsed);

        let sample = " 
{
	\"rta_response\": {
		\"version_info\": \"5.3.0.b6\",
		\"status\": 0,
		\"act\": 1,
		\"result\": {
			\"branch\": {
				\"subdivision\": \"\",
				\"name\": \"АО \\\"НЭСК\\\" \\\"Геленджикэнергосбыт\\\"\",
				\"address\": \"г. Геленджик, 353460, г. Геленджик, ул. Серафимовича 2\",
				\"phones\": \"3-60-55, 59-07-65\",
				\"current_account\": 40702810430000005000
			},
			\"abonent\": {
				\"is_from_cache\": 0,
				\"account\": 230600000010,
				\"new_account\": 1003010123,
				\"old_account\": 230600000010,
				\"closed\": 0,
				\"balance\": 0,
				\"balance_peni\": 0,
				\"balance_with_ndps\": 0,
				\"balance_with_ndps_peni\": 0,
				\"in_mop_distr\": 0,
				\"can_accept_payment\": 1,
				\"accept_ind_date_upper_limit\": \"\",
				\"service_closed\": 0,
				\"service_beg_date\": \"01.12.2008\",
				\"service_end_date\": \"\",
				\"occupancy\": 1,
				\"registered\": 0,
				\"rooms\": 1,
				\"floor\": 0,
				\"elevator\": 0,
				\"in_mop_distr_max_bill_period\": \"\",
				\"max_bill_period\": \"01.04.2022\",
				\"tariff\": {
					\"value\": 3.67,
					\"value_up\": 3.67,
					\"name\": \"Село\"
				},
				\"socnorma\": 0,
				\"acceptable_ind_dates\": {
					\"comment\": \"По Вашему лицевому счету на 01.06.2022 к расчетам приняты показания по результатам контрольного \
                      съема.\",
					\"ind_date\": {
						\"date\": \"01.07.2022\",
						\"period\": \"Июнь 2022\",
						\"comment\": \"\"
					}
				},
				\"last_KO\": {
					\"date\": \"01.08.2021\",
					\"ind\": 49485
				},
				\"last_pay\": {
					\"date\": \"28.04.2022\",
					\"amount\": 1027.6,
					\"provider\": \"ЕРЦ\",
					\"ind\": 52100
				},
				\"counters\": {
					\"counter\": {
						\"number\": 63073966,
						\"type\": \"ЦЭ6807Б (5 зн.)\",
						\"signs\": 5,
						\"scale\": 0,
						\"zones_qty\": 1,
						\"inst_date\": \"01.01.2009\",
						\"last_ind_date\": \"01.05.2022\",
						\"last_ind_type\": \"Показ. перед-е абонентом\",
						\"last_ind\": 52100,
						\"last_ind_sch_date\": \"01.05.2022\",
						\"last_ind_sch_type\": \"Показ. перед-е абонентом\",
						\"last_ind_sch\": 52100,
						\"last_verification\": \"01.01.2009\",
						\"next_verification\": \"01.01.2025\",
						\"eg_owner\": \"АО \\\"НЭСК-электросети\\\"\",
						\"eg_company\": \"АО \\\"НЭСК-электросети\\\"\"
					}
				},
				\"abonent_date_end\": \"\",
				\"service_status\": \"Включена\",
				\"offon_reazon\": \"\",
				\"sn_path\": \"Геленджикэнергосбыт Кабардинка,с Революционная,ул 22 230600000010\",
				\"sn_path_root\": \"Геленджикэнергосбыт\",
				\"tariff_energy\": \"Газ\",
				\"type_build\": \"Частный дом\",
				\"type_avar_build\": \"Не принадлежит аварийному фонду\",
				\"tarif_place\": \"Село\",
				\"type_el_equipment\": \"Не оборудован\",
				\"otop_period_vkl\": 0,
                \"test_nested_list\": [
                    {
                        \"date\": \"01.07.2022\",
						\"period\": \"Июнь 2022\",
						\"comment\": \"\"
                    },
                    {
                        \"date\": \"01.08.2022\",
						\"period\": \"Июль 2022\",
						\"comment\": \"\"
                    }
                ],
				\"address\": {
					\"settlement\": \"Кабардинка\",
					\"street\": \"Революционная\",
					\"house\": 22,
					\"korpus\": \"\",
					\"litera\": \"\",
					\"apartment\": \"\",
					\"apartment1\": \"\",
					\"string\": \"с. Кабардинка, ул. Революционная, д. 22\",
					\"fias\": \"6a24812b-192d-435a-88f5-67100159c67f\"
				}
			}
		}
	}
}
        ";
        let parsed = serde_json::from_str::<ScenarioVariableValue>(sample).expect("распарсилось");
        println!("{:#?}", parsed);
    }
}

#[test]
fn test123() {
    let a = "123";
    let b = 123;
    let j = serde_json::json!({"key": a, "key2": b, a: b});
    println!("{}", j)
}
