use chrono::prelude::*;
use chrono::Duration;
use near_sdk::env;

const FORMATO_FECHA: &str = "%Y-%m-%d %H:%M:%S UTC";

// TODO: Change date time to only dates

pub fn calcular_inicio() -> String {
    let timestamp = env::block_timestamp().to_string()[..10]
        .parse::<i64>()
        .unwrap();

    let parse_inicio = NaiveDateTime::from_timestamp(timestamp, 0);
    let inicio_utc: DateTime<Utc> = DateTime::from_utc(parse_inicio, Utc);

    inicio_utc.to_string()
}

pub fn agregar_dias(fecha: &str, dias: i64) -> String {
    //let formato_fecha = "%Y-%m-%d %H:%M:%S";
    let parse_fecha = NaiveDateTime::parse_from_str(&fecha, FORMATO_FECHA).unwrap();
    let fecha_utc: DateTime<Utc> = DateTime::from_utc(parse_fecha, Utc);

    fecha_utc
        .checked_add_signed(Duration::days(dias))
        .unwrap()
        .to_string()
}

// pub fn prueba_fecha(&self, dias: i64) {
//     let a = &env::block_timestamp().to_string()[..10];
//     let n = a.parse::<i64>().unwrap();
//     let b = NaiveDateTime::from_timestamp(n, 0);

//     let c: DateTime<Utc> = DateTime::from_utc(b, Utc);

//     let e = c.checked_add_signed(Duration::days(dias));
//     env::log(c.to_string().as_bytes());
//     env::log(e.unwrap().to_string().as_bytes());
// }
