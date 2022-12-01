#[derive(Debug)]
pub enum ElectroCaed {
    Authorizations,
}

impl ElectroCaed {
    pub fn get(&self) -> &'static str {
        match self {
            ElectroCaed::Authorizations => "authorization",
        }
    }
}
