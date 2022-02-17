use crate::types::{Pago, Periodo, Tanda, Usuario};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

mod date_handling;
mod log_generator;
mod types;

const MAX_PAGE_SIZE: u64 = 10;

fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TandaDapp {
    tandas: UnorderedMap<String, Tanda>,
    periodos_tanda: UnorderedMap<String, Vec<Periodo>>,
    usuarios: UnorderedMap<AccountId, Usuario>,
    pagos: UnorderedMap<String, HashMap<String, Vec<Pago>>>,
}

impl Default for TandaDapp {
    fn default() -> Self {
        Self {
            tandas: UnorderedMap::new(b"t".to_vec()),
            periodos_tanda: UnorderedMap::new(b"p".to_vec()),
            usuarios: UnorderedMap::new(b"u".to_vec()),
            pagos: UnorderedMap::new(b"h".to_vec()),
        }
    }
}

#[near_bindgen]
impl TandaDapp {
    #[payable]
    pub fn crear_tanda(
        &mut self,
        nombre_tanda: String,
        num_integrantes: u32,
        monto: u32,
        periodo: u32,
    ) {
        // * Validación de errores
        assert!(
            env::attached_deposit() >= one_near(),
            "Se requiere un pago de al menos 1 NEAR para la creación de la Tanda."
        );

        assert!(
            nombre_tanda != "",
            "El nombre de la Tanda no puede estar vacío."
        );
        assert!(
            num_integrantes >= 2,
            "La Tanda necesita al menos 2 integrantes."
        );

        assert!(monto > 0, "El monto a ahorrar tiene que ser mayor a 0.");
        assert!(periodo > 0, "El periodo no puede ser menor a 1.");

        // * Creación de Tanda

        let tanda = Tanda::new(String::from(&nombre_tanda), num_integrantes, monto, periodo);
        self.tandas.insert(&tanda.id, &tanda);

        // * Registro de usuario y tanda, generación de periodos de tanda.
        self.registrar_usuario(env::predecessor_account_id(), String::from(&tanda.id), true);

        self.generar_periodos(String::from(&tanda.id));

        // * Registro de log
        let msg = format!(
            "{} creó la tanda {}, con id: {}, {} personas ahorrarán {} NEAR cada {} días.",
            env::predecessor_account_id(),
            &nombre_tanda,
            &tanda.id,
            num_integrantes,
            monto,
            periodo
        );
        env::log(msg.as_bytes());
    }

    pub fn consultar_tanda(&self, clave: String) -> Option<Tanda> {
        assert!(clave != "", "El campo de clave no debe estar vacío.");
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

    pub fn agregar_integrante(&mut self, clave: String) {
        assert!(clave != "", "El campo de clave no debe estar vacío.");
        let id_cuenta = env::predecessor_account_id();

        let valido = self.validar_integrante(String::from(&clave), String::from(&id_cuenta));

        assert!(
            !valido,
            "El usuario {} ya es integrante de esta tanda.",
            &id_cuenta
        );

        let tanda = self.tandas.get(&clave);
        assert!(tanda.is_some(), "La tanda no existe.");
        tanda.unwrap().agregar_integrante(String::from(&id_cuenta));
    }

    pub fn consultar_integrantes(&self, clave: String) -> HashSet<AccountId> {
        assert!(clave != "", "El campo de clave no debe estar vacío.");

        let tanda = self.tandas.get(&clave);
        assert!(tanda.is_some(), "La tanda no existe.");

        tanda.unwrap().integrantes
    }

    // ! MÉTODO INTERNO
    fn validar_integrante(&self, id_tanda: String, id_cuenta: AccountId) -> bool {
        match self.tandas.get(&id_tanda) {
            Some(tanda) => tanda.integrantes.contains(&id_cuenta),
            None => false,
        }
    }

    #[payable]
    pub fn agregar_integrante_pago(&mut self, clave: String) -> bool {
        // * Validaciones
        let tanda_check = self.tandas.get(&clave);
        assert!(tanda_check.is_some(), "La tanda no existe.");
        let tanda = tanda_check.unwrap();
        let monto = env::attached_deposit();

        assert!(
            monto == one_near().checked_mul(tanda.monto as u128).unwrap(),
            "Sólo se pueden realizar pagos por la cantidad establecida en la Tanda ({} NEAR).",
            tanda.monto
        );

        let id_cuenta = env::predecessor_account_id();
        let valido = self.validar_integrante(String::from(&tanda.id), String::from(&id_cuenta));

        assert!(
            valido,
            "El usuario {} no es integrante de esta tanda.",
            &id_cuenta
        );

        assert!(
            self.periodos_tanda.get(&clave).is_some(),
            "Los periodos para esta tanda no están inicializados."
        );

        let mut periodos = self.periodos_tanda.get(&clave).unwrap();
        let indice = self.validar_periodo(String::from(&clave), Some(String::from(&id_cuenta)));

        assert!(
            indice >= 0,
            "El usuario {} no puede realizar pagos. 
            Ya se realizaron todos los pagos correspondientes a esta Tanda",
            &id_cuenta
        );

        // * Registro en periodos
        periodos[indice as usize]
            .integrantes_pagados
            .insert(String::from(&id_cuenta));

        let monto_a_sumar = monto.checked_div(one_near()).unwrap().to_string();
        periodos[indice as usize].cantidad_recaudada += monto_a_sumar.parse::<u32>().unwrap();

        self.periodos_tanda.insert(&clave, &periodos);

        self.validar_pago_tanda(String::from(&clave), indice);

        // * Registro en historial de pagos
        let new_payment = Pago::new(tanda.monto, date_handling::calcular_inicio());

        match self.pagos.get(&clave) {
            Some(mut historial) => {
                if historial.get_mut(&id_cuenta).is_some() {
                    let hist_user = historial.get_mut(&id_cuenta).unwrap();
                    let mut hist = self.pagos.get(&clave).unwrap();

                    hist_user.push(new_payment);
                    hist.insert(String::from(&id_cuenta), hist_user.to_vec());

                    self.pagos.insert(&clave, &hist);
                    env::log(format!("Nuevo pago añadido exitosamente.").as_bytes());

                    true
                } else {
                    let mut hist = self.pagos.get(&clave).unwrap();
                    let mut new_vec = Vec::<Pago>::new();

                    new_vec.push(new_payment);
                    hist.insert(String::from(&id_cuenta), new_vec);
                    self.pagos.insert(&clave, &hist);

                    env::log(format!("El primer pago fue registrado correctamente.").as_bytes());

                    true
                }
            }
            None => {
                env::log(format!("El historial de pagos está vacío.").as_bytes());

                let mut payment_map = HashMap::<String, Vec<Pago>>::new();
                let mut payment_vec = Vec::<Pago>::new();

                payment_vec.push(new_payment);
                payment_map.insert(id_cuenta, payment_vec);

                self.pagos.insert(&clave, &payment_map);

                env::log(format!("Se inicializó exitosamente el registro de pagos.").as_bytes());

                true
            }
        }
    }

    pub fn validar_pago_tanda(&mut self, clave: String, indice: i32) -> bool {
        assert!(self.tandas.get(&clave).is_some(), "La tanda no existe.");

        assert!(
            self.periodos_tanda.get(&clave).is_some(),
            "Los periodos para esta tanda no están inicializados."
        );

        let tanda = self.tandas.get(&clave).unwrap();
        let mut periodos = self.periodos_tanda.get(&clave).unwrap();
        let i = indice as usize;
        let cantidad_a_pagar = tanda.monto * tanda.num_integrantes;

        if periodos[i].cantidad_recaudada == cantidad_a_pagar
            && periodos[i].integrantes_pagados.len() as u32 == tanda.num_integrantes
        {
            periodos[i].pagos_completos = true;
            self.periodos_tanda.insert(&clave, &periodos);

            true
        } else {
            false
        }
    }

    pub fn validar_periodo(&self, clave: String, id_cuenta: Option<String>) -> i32 {
        assert!(
            self.periodos_tanda.get(&clave).is_some(),
            "Los periodos para esta tanda no están inicializados."
        );

        let periodos = self.periodos_tanda.get(&clave).unwrap();
        let cuenta = id_cuenta.unwrap_or(env::predecessor_account_id());

        for n in 0..periodos.len() {
            if !periodos[n].integrantes_pagados.contains(&cuenta) {
                return n as i32;
            }
        }

        -1
    }

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
