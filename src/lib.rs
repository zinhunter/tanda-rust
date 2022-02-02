use crate::types::{Tanda};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc, env};
use near_sdk::collections::UnorderedMap;
use std::collections::HashSet;

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
        periodo: u32
    ){
        let tanda = Tanda{
            id: env::block_index().to_string(),
            creador: env::predecessor_account_id(),
            nombre: String::from(&nombre_tanda),
            num_integrantes,
            monto,
            fecha_inicio: env::block_timestamp().to_string(),
            fecha_final: env::block_timestamp().to_string(),
            activa: false,
            periodo,
            estado: String::from("Pendiente"),
            integrantes: HashSet::new()
        };
        env::log(format!("'{}'", &tanda.id).as_bytes());
        self.tandas.insert(&tanda.id, &tanda);

        //TO DO:
        //Añadir registrar_usuario y generar_periodos
    }

    pub fn consultar_tanda(&self, clave: String) -> Option<Tanda> {
        self.tandas.get(&clave)
    }
}
