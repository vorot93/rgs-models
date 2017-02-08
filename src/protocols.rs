#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    Unspecified,
    Q3M,
    Q3S,
    A2S,
    TEEWORLDSM,
    TEEWORLDSS,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Protocol::Unspecified
    }
}
