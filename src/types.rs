use chrono::prelude::*;
use chrono::Duration;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};
use std::collections::HashSet;
use std::convert::TryFrom;

// * TANDA
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Tanda {
    pub id: String,
    pub creador: String,
    pub nombre_tanda: String,
    pub num_integrantes: u32,
    pub monto: u32,
    pub fecha_inicio: String,
    pub fecha_final: String,
    pub activa: bool,
    pub periodo: u32,
    pub estado: String,
    pub integrantes: HashSet<AccountId>,
}

impl Tanda {
    pub fn new(nombre: String, num_integrantes: u32, monto: u32, periodo: u32) -> Self {
        Self {
            id: env::block_index().to_string(),
            creador: env::predecessor_account_id(),
            nombre_tanda: String::from(&nombre),
            num_integrantes,
            monto,
            fecha_inicio: env::block_timestamp().to_string(),
            fecha_final: env::block_timestamp().to_string(),
            activa: false,
            periodo,
            estado: String::from("Pendiente"),
            integrantes: HashSet::new(),
        }
    }

    pub fn agregar_integrante(&mut self, integrante: AccountId) {
        if u32::try_from(self.integrantes.len()).unwrap() < self.num_integrantes {
            self.integrantes.insert(integrante);
        }
    }

    pub fn consultar_integrantes(&self) -> &HashSet<AccountId> {
        &self.integrantes
    }
}

impl Default for Tanda {
    fn default() -> Self {
        Tanda {
            id: String::from(""),
            creador: String::from(""),
            nombre_tanda: String::from(""),
            num_integrantes: 0,
            monto: 0,
            fecha_inicio: String::from(""),
            fecha_final: String::from(""),
            activa: false,
            periodo: 0,
            estado: String::from(""),
            integrantes: HashSet::new(),
        }
    }
}

// * PERIODO
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Periodo {
    pub inicio: String,
    pub fin: String,
    pub usuario_en_turno: AccountId,
    pub pagos_completos: bool,
    pub tanda_pagada: bool,
    pub cantidad_recaudada: u32,
    pub integrantes_pagados: HashSet<AccountId>,
}

impl Periodo {
    pub fn new(&self, inicio: String, periodo: i64) -> Self {
        Self {
            inicio: String::from(&inicio),
            fin: self.calcular_fin(&inicio, periodo),
            usuario_en_turno: String::from(""),
            pagos_completos: false,
            tanda_pagada: false,
            cantidad_recaudada: 0,
            integrantes_pagados: HashSet::new(),
        }
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

    fn calcular_fin(&self, inicio: &str, periodo: i64) -> String {
        let parse_inicio = NaiveDateTime::parse_from_str(&inicio, "%Y-%m-%d %H:%M:%S").unwrap();
        let inicio_utc: DateTime<Utc> = DateTime::from_utc(parse_inicio, Utc);

        inicio_utc
            .checked_add_signed(Duration::days(periodo))
            .unwrap()
            .to_string()
    }
}
