use crate::types::Tanda;
use chrono::prelude::*;
use chrono::Duration;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc};
use std::convert::TryInto;

mod types;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TandaDapp {
    tandas: UnorderedMap<String, Tanda>,
}

impl Default for TandaDapp {
    fn default() -> Self {
        Self {
            tandas: UnorderedMap::new(b"t".to_vec()),
        }
    }
}

#[near_bindgen]
impl TandaDapp {
    pub fn crear_tanda(
        &mut self,
        nombre_tanda: String,
        num_integrantes: u32,
        monto: u32,
        periodo: u32,
    ) {
        let tanda = Tanda::new(nombre_tanda, num_integrantes, monto, periodo);
        env::log(format!("'{}'", &tanda.id).as_bytes());
        self.tandas.insert(&tanda.id, &tanda);

        //TODO: Añadir registrar_usuario y generar_periodos
    }

    pub fn consultar_tanda(&self, clave: String) -> Option<Tanda> {
        self.tandas.get(&clave)
    }

    pub fn generar_periodos(&mut self, clave: String) {}

    pub fn prueba_fecha(&self, dias: i64) {
        let a = &env::block_timestamp().to_string()[..10];
        let n = a.parse::<i64>().unwrap();
        let b = NaiveDateTime::from_timestamp(n, 0);

        let c: DateTime<Utc> = DateTime::from_utc(b, Utc);

        let e = c.checked_add_signed(Duration::days(dias));
        env::log(c.to_string().as_bytes());
        env::log(e.unwrap().to_string().as_bytes());
    }

    pub fn prueba_periodo(&self) {}
}
