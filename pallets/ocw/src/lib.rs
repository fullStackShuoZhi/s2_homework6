#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;


use serde::{Deserialize, Deserializer};

#[frame_support::pallet]
pub mod pallet {
    use frame_support::log;
    use frame_support::inherent::Vec;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        offchain::{
            http, Duration,
        },
    };

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// 写入OffChain的数据类型
    pub type OffChainDataType = BoundedVec<u8, ConstU32<4>>;

    const ON_CHAIN_T0_OFF_CHAIN_INDEX: &[u8] = b"kictto:data_index";

    #[derive(Debug, Encode, Decode, Default)]
    struct OffChainData(OffChainDataType);

    #[derive(Deserialize, Encode, Decode)]
    struct WeatherInfoNow {
        #[serde(deserialize_with = "de_string_to_bytes")]
        temp: Vec<u8>,
        #[serde(deserialize_with = "de_string_to_bytes")]
        humidity: Vec<u8>,
        #[serde(deserialize_with = "de_string_to_bytes")]
        text: Vec<u8>,
    }

    #[derive(Deserialize, Encode, Decode)]
    struct WeatherInfo {
        #[serde(deserialize_with = "de_string_to_bytes", rename(deserialize = "updateTime"))]
        update_time: Vec<u8>,
        // #[serde(deserialize_with = "de_vec_to_bounded_vec")]
        // now: BoundedVec<WeatherInfoNow, ConstU32<100>>,
        now: WeatherInfoNow,
    }

    fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(de)?;
        Ok(s.as_bytes().to_vec())
    }

    use core::{convert::TryInto, fmt};
    use serde::{Deserialize, Deserializer};

    impl fmt::Debug for WeatherInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{{ updateTime: {}, now.text: {}, now.humidity: {} ,now.temp: {} }}",
                sp_std::str::from_utf8(&self.update_time).map_err(|_| fmt::Error)?,
                sp_std::str::from_utf8(&self.now.text).map_err(|_| fmt::Error)?,
                sp_std::str::from_utf8(&self.now.humidity).map_err(|_| fmt::Error)?,
                sp_std::str::from_utf8(&self.now.temp).map_err(|_| fmt::Error)?,
            )
        }
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // #[pallet::storage]
    // #[pallet::getter(fn something)]
    // pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        OffChainDataStored { data: OffChainDataType, who: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn save_data_to_off_chain(origin: OriginFor<T>, data: OffChainDataType) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let to_store_data = OffChainData(data.clone());
            sp_io::offchain_index::set(&ON_CHAIN_T0_OFF_CHAIN_INDEX, &to_store_data.encode());
            log::info!("data set:{:?}",&to_store_data);
            Self::deposit_event(Event::OffChainDataStored { data, who });
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OffChainWorker ==> RUN");
            if let Some(data_stored) =
                sp_runtime::offchain::storage::StorageValue::persistent(ON_CHAIN_T0_OFF_CHAIN_INDEX)
                    .get::<OffChainData>()
                    .unwrap_or_else(|_| {
                        log::info!("OffChainWorker ==> 无法从OffChainStorage读取数据");
                        None
                    }) {
                log::info!("OffChainWorker ==> 读取成功:{:?}",data_stored.0)
            }


            if let Ok(info) = Self::fetch_weather_info() {
                log::info!("OCW ==> Weather Info: {:?}", info);
            } else {
                log::info!("OCW ==> Error while fetch github info!");
            }
        }
    }

    impl<T: Config> Pallet<T> {
        fn fetch_weather_info() -> Result<WeatherInfo, http::Error> {
            // prepare for send request
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request =
                http::Request::get("https://helmet.wayxtech.com/api/common/taiShunWeather");
            let pending = request
                .deadline(deadline).send().map_err(|_| http::Error::IoError)?;
            let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown);
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;

            // parse the response str
            let gh_info: WeatherInfo =
                serde_json::from_str(body_str).map_err(|e| {
                    log::error!("Deserialize Fail,{:?}",e);
                    http::Error::Unknown
                })?;

            Ok(gh_info)
        }
    }
}
