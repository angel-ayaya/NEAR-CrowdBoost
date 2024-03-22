use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::{ env, near_bindgen, AccountId, NearToken, EpochHeight, PanicOnDefault };
use near_sdk::json_types::U128;
use near_sdk::env::{ account_balance };
/*
We import LookupMap and Vector from the NEAR SDK collections. 
LookupMap is used to create a map that associates keys and values (ideal for our tournaments)
while Vector is used to handle ordered lists of elements (such as the participants of each tournament).
*/
use near_sdk::store::{ Vector, LookupMap };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Tournament {
    owner: AccountId,
    tournaments: LookupMap<String, TournamentDetails>,
    tournaments_ids: Vector<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TournamentDetailsSerde  {
    name: String,
    prize_pool: u128, // Usando u128 directamente
    participants: Vector<AccountId>,
    is_active: bool,
    img_url: String,
}

// The original TournamentDetails with NEAR collections
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TournamentDetails {
    pub name: String,
    pub prize_pool: u128,
    pub participants: Vector<AccountId>,
    pub is_active: bool,
    img_url: String,
}

impl TournamentDetails {
    // Convert from Borsh-serialized struct to Serde-serialized struct
    pub fn to_serde(&self) -> TournamentDetailsSerde {
        TournamentDetailsSerde {
            name: self.name.clone(),
            prize_pool: self.prize_pool,
            participants: self.participants.iter().collect(),
            is_active: self.is_active,
        }
    }
}

impl Clone for TournamentDetails {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            prize_pool: self.prize_pool,
            participants: Vector::new(b"p"),
            is_active: self.is_active,
            img_url: self.img_url.clone(), // Add img_url field
        }
    }
}

#[near_bindgen]
impl Tournament {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized.");
        Self {
            owner,
            tournaments: LookupMap::new(b"t"),
            tournaments_ids: Vector::new(b"i"), // Add tournaments_ids field
        }
    }

    // Método para crear torneos.
    // Cualquier persona puede crear un torneo, y se requiere adjuntar el NEAR para el premio.
    #[payable]
    pub fn create_tournament(
        &mut self,
        tournament_id: String,
        name: String,
        prize_pool: U128,
        img_url: String
    ) {
        // Verificar que el torneo no exista ya.
        assert!(
            self.tournaments.get(&tournament_id).is_none(),
            "A tournament with this ID already exists."
        );
        let prize = NearToken::from_yoctonear(prize_pool.0);

        // Asegurar que el depósito adjunto coincida con el prize_pool especificado.
        assert_eq!(
            env::attached_deposit(),
            prize,
            "The attached deposit must match the specified prize pool."
        );

        // Crear el nuevo torneo.
        let tournament_details = TournamentDetails {
            name,
            prize_pool: prize_pool.0, // Asegúrate de manejar esto como u128.
            participants: Vector::new(b"p"),
            is_active: true,
            img_url: img_url,
        };

        // Insertar el torneo en el mapa.
        self.tournaments.insert(tournament_id, tournament_details);
        self.tournaments_ids.push(tournament_id);
    }

    // Metodos de vista
    // Función de vista para obtener todos los torneos
    pub fn get_all_tournaments(&self) -> Vec<(String, TournamentDetails)> {
        let mut tournaments_list: Vec<(String, TournamentDetails)> = Vec::new();

        for i in 0..self.tournaments_ids.len() {
            let key: String = self.tournaments_ids.get(i).unwrap().clone(); // Use .clone() to own the key
            let tournament: TournamentDetails = self.tournaments.get(&key).unwrap().clone(); // Clone TournamentDetails for ownership
            tournaments_list.push((key, tournament));
        }

        tournaments_list
    }
}

// let balance_token: NearToken = env::account_balance(); // Obtiene el saldo como NearToken
// let balance_yoctonear: u128 = balance_token.as_yoctonear(); // Convierte a yoctoNEAR (u128)
