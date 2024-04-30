use super::data::Data;

#[derive(Default)]
pub struct Registers {
    v1: Option<Data>,
    v2: Option<Data>,
    res: Option<Data>,
    params: Vec<Data>,
    ret: Vec<Data>,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load(&mut self, value: Data) -> Result<(), ()> {
        if self.res.is_some() {
            if self.v1.is_none() {
                std::mem::swap(&mut self.v1, &mut self.res);
                self.v2 = Some(value);

                return Ok(());
            } else {
                return Err(());
            }
        }

        if self.v1.is_none() {
            self.v1 = Some(value);
        } else if self.v2.is_none() {
            self.v2 = Some(value);
        } else {
            return Err(());
        }

        Ok(())
    }

    pub fn unload(&mut self) -> Option<Data> {
        self.res.take()
    }

    pub fn set_params(&mut self, params: Vec<Data>) -> Result<(), ()> {
        if !self.params.is_empty() {
            return Err(());
        }

        self.params = params;
        Ok(())
    }

    pub fn get_return(&mut self) -> Vec<Data> {
        std::mem::take(&mut self.ret)
    }
}
