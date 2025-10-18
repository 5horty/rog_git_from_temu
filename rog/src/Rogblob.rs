use crate::rogObject::rogObject;

pub struct RogBlob {
    pub bloddata: Vec<u8>,
}
impl rogObject for RogBlob {
    fn serialize(&self) -> Vec<u8> {
        self.bloddata.clone()
    }
    fn deserialize(&mut self, data: &[u8]) {
        self.bloddata = data.to_vec();
    }
    fn fmt(&self) -> &str {
        "blob"
    }
    fn init(&mut self) {
        self.bloddata.clear();
    }
}
impl RogBlob {
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            bloddata: data.to_vec(),
        }
    }
}
