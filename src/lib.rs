//use crate::log_generator;
use crate::types::{Periodo, Tanda, Usuario};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use std::cmp;
use std::collections::HashSet;
use std::vec::Vec;

mod date_handling;
mod log_generator;
mod types;

const MAX_PAGE_SIZE: u64 = 10;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TandaDapp {
    tandas: UnorderedMap<String, Tanda>,
    periodos_tanda: UnorderedMap<String, Vec<Periodo>>,
    usuarios: UnorderedMap<AccountId, Usuario>,
}

impl Default for TandaDapp {
    fn default() -> Self {
        Self {
            tandas: UnorderedMap::new(b"t".to_vec()),
            periodos_tanda: UnorderedMap::new(b"p".to_vec()),
            usuarios: UnorderedMap::new(b"u".to_vec()),
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

    pub fn registrar_usuario(&mut self, cuenta: AccountId, id_tanda: String, creada: bool) {
        match self.usuarios.get(&cuenta) {
            Some(mut usuario) => {
                match creada {
                    true => usuario.tandas_creadas.push(id_tanda),
                    false => usuario.tandas_inscritas.push(id_tanda),
                };

                self.usuarios.insert(&cuenta, &usuario);
            }
            None => {
                let mut nuevo_usuario: Usuario = Usuario::new(String::from(&cuenta));

                match creada {
                    true => nuevo_usuario.tandas_creadas.push(id_tanda),
                    false => nuevo_usuario.tandas_inscritas.push(id_tanda),
                }

                self.usuarios.insert(&cuenta, &nuevo_usuario);
            }
        }
    }

    pub fn consultar_usuarios(&self) -> Vec<AccountId> {
        self.usuarios.keys_as_vector().to_vec()
    }

    pub fn consultar_tandas_creadas(&self, id_cuenta: Option<String>) -> Vec<String> {
        match self
            .usuarios
            .get(&id_cuenta.unwrap_or(env::predecessor_account_id()))
        {
            Some(usuario) => usuario.tandas_creadas,
            None => Vec::new(),
        }
    }

    pub fn consultar_tandas_inscritas(&self, id_cuenta: Option<String>) -> Vec<String> {
        match self
            .usuarios
            .get(&id_cuenta.unwrap_or(env::predecessor_account_id()))
        {
            Some(usuario) => usuario.tandas_inscritas,
            None => Vec::new(),
        }
    }

    // ! MÉTODO INTERNO
    fn buscar_tandas(&self, lista_tandas: Vec<String>) -> Vec<Tanda> {
        let mut result = Vec::<Tanda>::new();

        for n in 0..lista_tandas.len() {
            result.push(self.tandas.get(&lista_tandas[n]).unwrap())
        }

        result
    }

    pub fn consultar_tandas(&self) -> Vec<Option<Tanda>> {
        let claves_tandas = self.tandas.keys_as_vector().to_vec();
        let num_tandas = cmp::min(MAX_PAGE_SIZE, claves_tandas.len() as u64);
        let start_index = claves_tandas.len() as u64 - num_tandas;

        let mut result = Vec::<Option<Tanda>>::new();

        for n in 0..num_tandas {
            result.push(self.tandas.get(&claves_tandas[(n + start_index) as usize]));
        }

        result
    }

    pub fn consultar_tandas_todas(&self) -> Vec<Tanda> {
        self.tandas.values_as_vector().to_vec()
    }

    pub fn agregar_integrante(&mut self) {
        let id_cuenta = env::predecessor_account_id();
    }

    // pub fn consultar_integrantes(&self) -> HashSet<AccountId> {

    // }
    // // ! MÉTODO INTERNO
    // fn validar_integrante(&self, id_tanda: String, id_cuenta: AccountId){
    //     match self.tandas.get(&id_tanda) {
    //         Some(tanda) => {
    //             tanda.integrantes.
    //         },
    //         None => {}
    //     }
    // }

    pub fn generar_periodos(&mut self, clave: String) {
        match self.tandas.get(&clave) {
            Some(tanda) => {
                match self.periodos_tanda.get(&clave) {
                    Some(_periodos) => {
                        // generar log de que ya estaban inicializados..
                    }
                    None => {
                        //Checar errores...
                        // TODO: Check borrowing
                        let mut vec_periodos = Vec::<Periodo>::new();
                        let mut fecha_inicio = &tanda.fecha_inicio;
                        let mut temp: String;

                        for _n in 0..tanda.num_integrantes {
                            let fecha_final = date_handling::agregar_dias(
                                &fecha_inicio,
                                (tanda.periodo - 1) as i64,
                            );
                            let periodo: Periodo = Periodo::new(
                                String::from(fecha_inicio),
                                String::from(&fecha_final),
                            );

                            vec_periodos.push(periodo);

                            temp = date_handling::agregar_dias(&String::from(fecha_final), 1);
                            fecha_inicio = &temp;
                        }

                        self.periodos_tanda.insert(&clave, &vec_periodos);
                    }
                }
            }
            None => {}
        }
    }

    pub fn consultar_periodos(&self, clave: String) -> Option<Vec<Periodo>> {
        self.periodos_tanda.get(&clave)
    }
}
