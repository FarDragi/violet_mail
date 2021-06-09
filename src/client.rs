pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GResult<T> = Result<T, GenericError>;

pub(self) mod static_data {
    use lazy_static::lazy_static;
    use std::{any::Any, collections::HashMap, sync::Mutex};

    use super::GResult;

    lazy_static! {
        static ref STORAGE_STATIC: Mutex<StaticData> = StaticData::new();
    }

    pub struct StaticData {
        data: HashMap<String, Box<dyn Any + Send + Sync>>,
    }

    impl StaticData {
        fn new() -> Mutex<Self> {
            let data = Self {
                data: HashMap::new(),
            };

            Mutex::new(data)
        }

        fn set_data_value(&mut self, name: String, data: impl Any + Send + Sync) {
            if let Some(mapped) = self.data.get_mut(&name) {
                *mapped = Box::new(data);
            } else {
                self.data.insert(name, Box::new(data));
            }
        }

        pub fn add_element_to_static(name: String, data: impl Any + Send + Sync) -> GResult<()> {
            let mut locked_data = STORAGE_STATIC.lock().map_err(|why| format!("{:?}", why))?;
            locked_data.set_data_value(name, data);
            Ok(())
        }

        pub fn get_data_elemet_static<T: 'static, F>(name: &str, callback: F) -> GResult<()>
        where
            F: Fn(&T),
        {
            let locked_data = STORAGE_STATIC.lock().map_err(|why| format!("{:?}", why))?;
            let data = locked_data
                .data
                .get(name)
                .ok_or("Key não possui dados")?
                .downcast_ref::<T>()
                .ok_or("Downcast não pode inferir esse tipo")?;

            callback(data);
            Ok(())
        }
    }
}
