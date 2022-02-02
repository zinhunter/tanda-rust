use std::convert::TryFrom;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId};
use std::collections::HashSet;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Tanda {
    pub id: String,
    pub creador: String,
    pub nombre: String,
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
    fn agregar_integrante(&mut self, integrante: AccountId){

        if u32::try_from(self.integrantes.len()).unwrap() < self.num_integrantes {
            self.integrantes.insert(integrante);
        }
    }

    fn consultar_integrantes(&self)-> &HashSet<AccountId> {
        &self.integrantes
    }
}

impl Default for Tanda {
    fn default() -> Self {
        Tanda {
            id: String::from(""),
            creador: String::from(""),
            nombre: String::from(""),
            num_integrantes: 0,
            monto: 0,
            fecha_inicio: String::from(""),
            fecha_final: String::from(""),
            activa: false,
            periodo: 0,
            estado: String::from(""),
            integrantes: HashSet::new()
        }
    }
}