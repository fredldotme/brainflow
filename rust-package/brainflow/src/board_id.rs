#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive)]
pub enum BoardId {
    PlaybackFileBoard = -3,
    StreamingBoard = -2,
    SyntheticBoard = -1,
    CytonBoard = 0,
    GanglionBoard = 1,
    CytonDaisyBoard = 2,
    GaleaBoard = 3,
    GanglionWifiBoard = 4,
    CytonWifiBoard = 5,
    CytonDaisyWifiBoard = 6,
    BrainbitBoard = 7,
    UnicornBoard = 8,
    CallibriEegBoard = 9,
    CallibriEmgBoard = 10,
    CallibriEcgBoard = 11,
    FasciaBoard = 12,
    Notion1Board = 13,
    Notion2Board = 14,
    IronbciBoard = 15,
    GforceProBoard = 16,
    Freeeeg32Board = 17,
    BrainbitBledBoard = 18,
    GforceDualBoard = 19,
    GaleaSerialBoard = 20,
    MuseSBledBoard = 21,
    Muse2BledBoard = 22,
    CrownBoard = 23,
    AntNeuroEe410Board = 24,
    AntNeuroEe411Board = 25,
    AntNeuroEe430Board = 26,
    AntNeuroEe211Board = 27,
    AntNeuroEe212Board = 28,
    AntNeuroEe213Board = 29,
    AntNeuroEe214Board = 30,
    AntNeuroEe215Board = 31,
    AntNeuroEe221Board = 32,
    AntNeuroEe222Board = 33,
    AntNeuroEe223Board = 34,
    AntNeuroEe224Board = 35,
    AntNeuroEe225Board = 36,
    EnophoneBoard = 37,
}
