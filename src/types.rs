use crate::date_handling;
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
            fecha_inicio: date_handling::calcular_inicio(),
            fecha_final: date_handling::agregar_dias(
                &date_handling::calcular_inicio(),
                (num_integrantes * periodo - 1) as i64,
            ),
            activa: false,
            periodo,
            estado: String::from("Pendiente"),
            integrantes: HashSet::new(),
        }
    }

    pub fn agregar_integrante(&mut self, integrante: AccountId) {
        if u32::try_from(self.integrantes.len()).unwrap() < self.num_integrantes {
            self.integrantes.insert(integrante);
            env::log(
                format!(
                    "Integrante nuevo {} agregado exitosamente.",
                    env::predecessor_account_id()
                )
                .as_bytes(),
            );
        } else {
            env::log(
                format!("La Tanda se encuentra llena, ya no existen lugares disponibles.")
                    .as_bytes(),
            );
        }
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
    pub fn new(inicio: String, fin: String) -> Self {
        Self {
            inicio,
            fin,
            usuario_en_turno: String::from(""),
            pagos_completos: false,
            tanda_pagada: false,
            cantidad_recaudada: 0,
            integrantes_pagados: HashSet::new(),
        }
    }
}

impl Default for Periodo {
    fn default() -> Self {
        Periodo {
            inicio: String::from(""),
            fin: String::from(""),
            usuario_en_turno: String::from(""),
            pagos_completos: false,
            tanda_pagada: false,
            cantidad_recaudada: 0,
            integrantes_pagados: HashSet::new(),
        }
    }
}

// * USUARIO
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Usuario {
    pub cuenta: AccountId,
    pub tandas_creadas: Vec<String>,
    pub tandas_inscritas: Vec<String>,
}

impl Usuario {
    pub fn new(cuenta: AccountId) -> Self {
        Self {
            cuenta,
            tandas_creadas: Vec::<String>::new(),
            tandas_inscritas: Vec::<String>::new(),
        }
    }
}

impl Default for Usuario {
    fn default() -> Self {
        Usuario {
            cuenta: String::from(""),
            tandas_creadas: Vec::<String>::new(),
            tandas_inscritas: Vec::<String>::new(),
        }
    }
}

// * PAGO
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pago {}
