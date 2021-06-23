use std::str;

pub type Address = u8;
pub type Data = Vec<u8>;

pub type CRC = [u8; 2];

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum ChecksumType {
    SimpleChecksum,
    CRCChecksum,
}

impl ChecksumType {
    pub fn from_str(checksum_type: String) -> ChecksumType {
        match checksum_type.as_ref() {
            "simple" => ChecksumType::SimpleChecksum,
            "crc" => ChecksumType::CRCChecksum,
            _ => panic!("Invalid cheksum type in config: {:?}", checksum_type),
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum HeaderType {
    FactorySetup,
    SimplePoll,
    AddressPoll,
    AddressClash,
    AddressChange,
    AddressRandom,
    RequestPollingPriority,
    RequestStatus,
    RequestVariableSet,
    RequestManufacturerId,
    RequestEquipmentCategoryId,
    RequestProductCode,
    RequestDatabaseVersion,
    RequestSerialNumber,
    RequestSoftwareRevision,
    TestSolenoids,
    OperateMotors,
    TestOutputLines,
    ReadInputLines,
    ReadOptoStates,
    ReadLastCreditOrErrorCode,
    IssueGuardCode,
    LatchOutputLines,
    PerformSelfcheck,
    ModifyInhibitStatus,
    RequestInhibitStatus,
    ReadBufferedCreditOrErrorCodes,
    ModifyMasterInhibitStatus,
    RequestMasterInhibitStatus,
    RequestInsertionCounter,
    RequestAcceptCounter,
    DispenseCoins,
    DispenseChange,
    ModifySorterOverrideStatus,
    RequestSorterOverrideStatus,
    OneshotCredit,
    EnterNewPINNumber,
    EnterPINNumber,
    RequestPayoutHighLowStatus,
    RequestDataStorageAvailability,
    ReadDataBlock,
    WriteDataBlock,
    RequestOptionFlags,
    RequestCoinPosition,
    PowerManagementControl,
    ModifySorterPaths,
    RequestSorterPaths,
    ModifyPayoutAbsoluteCount,
    RequestPayoutAbsoluteCount,
    EmptyPayout,
    RequestAuditInformationBlock,
    MeterControl,
    DisplayControl,
    TeachModeControl,
    RequestTeachStatus,
    UploadCoinData,
    ConfigurationToEEPROM,
    CountersToEEPROM,
    CalculateROMChecksum,
    RequestCreationDate,
    RequestLastModificationDate,
    RequestRejectCounter,
    RequestFraudCounter,
    RequestBuildCode,
    KeypadControl,
    RequestPayoutStatus,
    ModifyDefaultSorterPath,
    RequestDefaultSorterPath,
    ModifyPayoutCapacity,
    RequestPayoutCapacity,
    ModifyCoinId,
    RequestCoinId,
    UploadWindowData,
    DownloadCalibrationInfo,
    ModifySecuritySetting,
    RequestSecuritySetting,
    ModifyBankSelect,
    RequestBankSelect,
    HandheldFunction,
    RequestAlarmCounter,
    ModifyPayoutFloat,
    RequestPayoutFloat,
    RequestThermistorReading,
    EmergencyStop,
    RequestHopperCoin,
    RequestBaseYear,
    RequestAddressMode,
    RequestHopperDispenseCount,
    DispenseHopperCoins,
    RequestHopperStatus,
    ModifyVariableSet,
    EnableHopper,
    TestHopper,
    ModifyInhibitAndOverrideRegisters,
    PumpRNG,
    RequestCipherKey,
    ReadBufferedBillEvents,
    ModifyBillId,
    RequestBillId,
    RequestCountryScalingFactor,
    RequestBillPosition,
    RouteBill,
    ModifyBillOperatingMode,
    RequestBillOperatingMode,
    TestLamps,
    RequestIndividualAcceptCounter,
    RequestIndividualErrorCounter,
    ReadOptoVoltages,
    PerformStackerCycle,
    OperateBidirectionalMotors,
    RequestCurrencyRevision,
    UploadBillTables,
    BeginBillTableUpgrade,
    FinishBillTableUpgrade,
    RequestFirmwareUpgradeCapability,
    UploadFirmware,
    BeginFirmwareUpgrade,
    FinishFirmwareUpgrade,
    SwitchEncryptionCode,
    StoreEncryptionCode,
    SetAcceptLimit,
    DispenseHopperValue,
    RequestHopperPollingValue,
    EmergencyStopValue,
    RequestHopperCoinValue,
    RequestIndexedHopperDispenseCount,
    ReadBarcodeData,
    RequestMoneyIn,
    RequestMoneyOut,
    ClearMoneyCounters,
    PayMoneyOut,
    VerifyMoneyOut,
    RequestActivityRegister,
    RequestErrorStatus,
    PurgeHopper,
    ModifyHopperBalance,
    RequestHopperBalance,
    ModifyCashboxValue,
    RequestCashboxValue,
    ModifyRealTimeClock,
    RequestRealTimeClock,
    RequestUSBId,
    SwitchBaudRate,
    ReadEncryptedEvents,
    RequestEncryptionSupport,
    SwitchEncryptionKey,
    RequestEncryptedHopperStatus,
    RequestEncryptedMonetaryId,
    RequestCommsRevision,
    ClearCommsStatusVariables,
    RequestCommsStatusVariables,
    ResetDevice,
    Reply,
    Unknown(u8),
}

impl HeaderType {
    #[allow(dead_code)]
    pub fn from_u8(n: u8) -> HeaderType {
        match n {
            255 => HeaderType::FactorySetup,
            254 => HeaderType::SimplePoll,
            253 => HeaderType::AddressPoll,
            252 => HeaderType::AddressClash,
            251 => HeaderType::AddressChange,
            250 => HeaderType::AddressRandom,
            249 => HeaderType::RequestPollingPriority,
            248 => HeaderType::RequestStatus,
            247 => HeaderType::RequestVariableSet,
            246 => HeaderType::RequestManufacturerId,
            245 => HeaderType::RequestEquipmentCategoryId,
            244 => HeaderType::RequestProductCode,
            243 => HeaderType::RequestDatabaseVersion,
            242 => HeaderType::RequestSerialNumber,
            241 => HeaderType::RequestSoftwareRevision,
            240 => HeaderType::TestSolenoids,
            239 => HeaderType::OperateMotors,
            238 => HeaderType::TestOutputLines,
            237 => HeaderType::ReadInputLines,
            236 => HeaderType::ReadOptoStates,
            235 => HeaderType::ReadLastCreditOrErrorCode,
            234 => HeaderType::IssueGuardCode,
            233 => HeaderType::LatchOutputLines,
            232 => HeaderType::PerformSelfcheck,
            231 => HeaderType::ModifyInhibitStatus,
            230 => HeaderType::RequestInhibitStatus,
            229 => HeaderType::ReadBufferedCreditOrErrorCodes,
            228 => HeaderType::ModifyMasterInhibitStatus,
            227 => HeaderType::RequestMasterInhibitStatus,
            226 => HeaderType::RequestInsertionCounter,
            225 => HeaderType::RequestAcceptCounter,
            224 => HeaderType::DispenseCoins,
            223 => HeaderType::DispenseChange,
            222 => HeaderType::ModifySorterOverrideStatus,
            221 => HeaderType::RequestSorterOverrideStatus,
            220 => HeaderType::OneshotCredit,
            219 => HeaderType::EnterNewPINNumber,
            218 => HeaderType::EnterPINNumber,
            217 => HeaderType::RequestPayoutHighLowStatus,
            216 => HeaderType::RequestDataStorageAvailability,
            215 => HeaderType::ReadDataBlock,
            214 => HeaderType::WriteDataBlock,
            213 => HeaderType::RequestOptionFlags,
            212 => HeaderType::RequestCoinPosition,
            211 => HeaderType::PowerManagementControl,
            210 => HeaderType::ModifySorterPaths,
            209 => HeaderType::RequestSorterPaths,
            208 => HeaderType::ModifyPayoutAbsoluteCount,
            207 => HeaderType::RequestPayoutAbsoluteCount,
            206 => HeaderType::EmptyPayout,
            205 => HeaderType::RequestAuditInformationBlock,
            204 => HeaderType::MeterControl,
            203 => HeaderType::DisplayControl,
            202 => HeaderType::TeachModeControl,
            201 => HeaderType::RequestTeachStatus,
            200 => HeaderType::UploadCoinData,
            199 => HeaderType::ConfigurationToEEPROM,
            198 => HeaderType::CountersToEEPROM,
            197 => HeaderType::CalculateROMChecksum,
            196 => HeaderType::RequestCreationDate,
            195 => HeaderType::RequestLastModificationDate,
            194 => HeaderType::RequestRejectCounter,
            193 => HeaderType::RequestFraudCounter,
            192 => HeaderType::RequestBuildCode,
            191 => HeaderType::KeypadControl,
            190 => HeaderType::RequestPayoutStatus,
            189 => HeaderType::ModifyDefaultSorterPath,
            188 => HeaderType::RequestDefaultSorterPath,
            187 => HeaderType::ModifyPayoutCapacity,
            186 => HeaderType::RequestPayoutCapacity,
            185 => HeaderType::ModifyCoinId,
            184 => HeaderType::RequestCoinId,
            183 => HeaderType::UploadWindowData,
            182 => HeaderType::DownloadCalibrationInfo,
            181 => HeaderType::ModifySecuritySetting,
            180 => HeaderType::RequestSecuritySetting,
            179 => HeaderType::ModifyBankSelect,
            178 => HeaderType::RequestBankSelect,
            177 => HeaderType::HandheldFunction,
            176 => HeaderType::RequestAlarmCounter,
            175 => HeaderType::ModifyPayoutFloat,
            174 => HeaderType::RequestPayoutFloat,
            173 => HeaderType::RequestThermistorReading,
            172 => HeaderType::EmergencyStop,
            171 => HeaderType::RequestHopperCoin,
            170 => HeaderType::RequestBaseYear,
            169 => HeaderType::RequestAddressMode,
            168 => HeaderType::RequestHopperDispenseCount,
            167 => HeaderType::DispenseHopperCoins,
            166 => HeaderType::RequestHopperStatus,
            165 => HeaderType::ModifyVariableSet,
            164 => HeaderType::EnableHopper,
            163 => HeaderType::TestHopper,
            162 => HeaderType::ModifyInhibitAndOverrideRegisters,
            161 => HeaderType::PumpRNG,
            160 => HeaderType::RequestCipherKey,
            159 => HeaderType::ReadBufferedBillEvents,
            158 => HeaderType::ModifyBillId,
            157 => HeaderType::RequestBillId,
            156 => HeaderType::RequestCountryScalingFactor,
            155 => HeaderType::RequestBillPosition,
            154 => HeaderType::RouteBill,
            153 => HeaderType::ModifyBillOperatingMode,
            152 => HeaderType::RequestBillOperatingMode,
            151 => HeaderType::TestLamps,
            150 => HeaderType::RequestIndividualAcceptCounter,
            149 => HeaderType::RequestIndividualErrorCounter,
            148 => HeaderType::ReadOptoVoltages,
            147 => HeaderType::PerformStackerCycle,
            146 => HeaderType::OperateBidirectionalMotors,
            145 => HeaderType::RequestCurrencyRevision,
            144 => HeaderType::UploadBillTables,
            143 => HeaderType::BeginBillTableUpgrade,
            142 => HeaderType::FinishBillTableUpgrade,
            141 => HeaderType::RequestFirmwareUpgradeCapability,
            140 => HeaderType::UploadFirmware,
            139 => HeaderType::BeginFirmwareUpgrade,
            138 => HeaderType::FinishFirmwareUpgrade,
            137 => HeaderType::SwitchEncryptionCode,
            136 => HeaderType::StoreEncryptionCode,
            135 => HeaderType::SetAcceptLimit,
            134 => HeaderType::DispenseHopperValue,
            133 => HeaderType::RequestHopperPollingValue,
            132 => HeaderType::EmergencyStopValue,
            131 => HeaderType::RequestHopperCoinValue,
            130 => HeaderType::RequestIndexedHopperDispenseCount,
            129 => HeaderType::ReadBarcodeData,
            128 => HeaderType::RequestMoneyIn,
            127 => HeaderType::RequestMoneyOut,
            126 => HeaderType::ClearMoneyCounters,
            125 => HeaderType::PayMoneyOut,
            124 => HeaderType::VerifyMoneyOut,
            123 => HeaderType::RequestActivityRegister,
            122 => HeaderType::RequestErrorStatus,
            121 => HeaderType::PurgeHopper,
            120 => HeaderType::ModifyHopperBalance,
            119 => HeaderType::RequestHopperBalance,
            118 => HeaderType::ModifyCashboxValue,
            117 => HeaderType::RequestCashboxValue,
            116 => HeaderType::ModifyRealTimeClock,
            115 => HeaderType::RequestRealTimeClock,
            114 => HeaderType::RequestUSBId,
            113 => HeaderType::SwitchBaudRate,
            112 => HeaderType::ReadEncryptedEvents,
            111 => HeaderType::RequestEncryptionSupport,
            110 => HeaderType::SwitchEncryptionKey,
            109 => HeaderType::RequestEncryptedHopperStatus,
            108 => HeaderType::RequestEncryptedMonetaryId,
            4 => HeaderType::RequestCommsRevision,
            3 => HeaderType::ClearCommsStatusVariables,
            2 => HeaderType::RequestCommsStatusVariables,
            1 => HeaderType::ResetDevice,
            0 => HeaderType::Reply,
            _ => HeaderType::Unknown(n),
        }
    }

    #[allow(dead_code)]
    pub fn to_u8(&self) -> u8 {
        match *self {
            HeaderType::FactorySetup => 255,
            HeaderType::SimplePoll => 254,
            HeaderType::AddressPoll => 253,
            HeaderType::AddressClash => 252,
            HeaderType::AddressChange => 251,
            HeaderType::AddressRandom => 250,
            HeaderType::RequestPollingPriority => 249,
            HeaderType::RequestStatus => 248,
            HeaderType::RequestVariableSet => 247,
            HeaderType::RequestManufacturerId => 246,
            HeaderType::RequestEquipmentCategoryId => 245,
            HeaderType::RequestProductCode => 244,
            HeaderType::RequestDatabaseVersion => 243,
            HeaderType::RequestSerialNumber => 242,
            HeaderType::RequestSoftwareRevision => 241,
            HeaderType::TestSolenoids => 240,
            HeaderType::OperateMotors => 239,
            HeaderType::TestOutputLines => 238,
            HeaderType::ReadInputLines => 237,
            HeaderType::ReadOptoStates => 236,
            HeaderType::ReadLastCreditOrErrorCode => 235,
            HeaderType::IssueGuardCode => 234,
            HeaderType::LatchOutputLines => 233,
            HeaderType::PerformSelfcheck => 232,
            HeaderType::ModifyInhibitStatus => 231,
            HeaderType::RequestInhibitStatus => 230,
            HeaderType::ReadBufferedCreditOrErrorCodes => 229,
            HeaderType::ModifyMasterInhibitStatus => 228,
            HeaderType::RequestMasterInhibitStatus => 227,
            HeaderType::RequestInsertionCounter => 226,
            HeaderType::RequestAcceptCounter => 225,
            HeaderType::DispenseCoins => 224,
            HeaderType::DispenseChange => 223,
            HeaderType::ModifySorterOverrideStatus => 222,
            HeaderType::RequestSorterOverrideStatus => 221,
            HeaderType::OneshotCredit => 220,
            HeaderType::EnterNewPINNumber => 219,
            HeaderType::EnterPINNumber => 218,
            HeaderType::RequestPayoutHighLowStatus => 217,
            HeaderType::RequestDataStorageAvailability => 216,
            HeaderType::ReadDataBlock => 215,
            HeaderType::WriteDataBlock => 214,
            HeaderType::RequestOptionFlags => 213,
            HeaderType::RequestCoinPosition => 212,
            HeaderType::PowerManagementControl => 211,
            HeaderType::ModifySorterPaths => 210,
            HeaderType::RequestSorterPaths => 209,
            HeaderType::ModifyPayoutAbsoluteCount => 208,
            HeaderType::RequestPayoutAbsoluteCount => 207,
            HeaderType::EmptyPayout => 206,
            HeaderType::RequestAuditInformationBlock => 205,
            HeaderType::MeterControl => 204,
            HeaderType::DisplayControl => 203,
            HeaderType::TeachModeControl => 202,
            HeaderType::RequestTeachStatus => 201,
            HeaderType::UploadCoinData => 200,
            HeaderType::ConfigurationToEEPROM => 199,
            HeaderType::CountersToEEPROM => 198,
            HeaderType::CalculateROMChecksum => 197,
            HeaderType::RequestCreationDate => 196,
            HeaderType::RequestLastModificationDate => 195,
            HeaderType::RequestRejectCounter => 194,
            HeaderType::RequestFraudCounter => 193,
            HeaderType::RequestBuildCode => 192,
            HeaderType::KeypadControl => 191,
            HeaderType::RequestPayoutStatus => 190,
            HeaderType::ModifyDefaultSorterPath => 189,
            HeaderType::RequestDefaultSorterPath => 188,
            HeaderType::ModifyPayoutCapacity => 187,
            HeaderType::RequestPayoutCapacity => 186,
            HeaderType::ModifyCoinId => 185,
            HeaderType::RequestCoinId => 184,
            HeaderType::UploadWindowData => 183,
            HeaderType::DownloadCalibrationInfo => 182,
            HeaderType::ModifySecuritySetting => 181,
            HeaderType::RequestSecuritySetting => 180,
            HeaderType::ModifyBankSelect => 179,
            HeaderType::RequestBankSelect => 178,
            HeaderType::HandheldFunction => 177,
            HeaderType::RequestAlarmCounter => 176,
            HeaderType::ModifyPayoutFloat => 175,
            HeaderType::RequestPayoutFloat => 174,
            HeaderType::RequestThermistorReading => 173,
            HeaderType::EmergencyStop => 172,
            HeaderType::RequestHopperCoin => 171,
            HeaderType::RequestBaseYear => 170,
            HeaderType::RequestAddressMode => 169,
            HeaderType::RequestHopperDispenseCount => 168,
            HeaderType::DispenseHopperCoins => 167,
            HeaderType::RequestHopperStatus => 166,
            HeaderType::ModifyVariableSet => 165,
            HeaderType::EnableHopper => 164,
            HeaderType::TestHopper => 163,
            HeaderType::ModifyInhibitAndOverrideRegisters => 162,
            HeaderType::PumpRNG => 161,
            HeaderType::RequestCipherKey => 160,
            HeaderType::ReadBufferedBillEvents => 159,
            HeaderType::ModifyBillId => 158,
            HeaderType::RequestBillId => 157,
            HeaderType::RequestCountryScalingFactor => 156,
            HeaderType::RequestBillPosition => 155,
            HeaderType::RouteBill => 154,
            HeaderType::ModifyBillOperatingMode => 153,
            HeaderType::RequestBillOperatingMode => 152,
            HeaderType::TestLamps => 151,
            HeaderType::RequestIndividualAcceptCounter => 150,
            HeaderType::RequestIndividualErrorCounter => 149,
            HeaderType::ReadOptoVoltages => 148,
            HeaderType::PerformStackerCycle => 147,
            HeaderType::OperateBidirectionalMotors => 146,
            HeaderType::RequestCurrencyRevision => 145,
            HeaderType::UploadBillTables => 144,
            HeaderType::BeginBillTableUpgrade => 143,
            HeaderType::FinishBillTableUpgrade => 142,
            HeaderType::RequestFirmwareUpgradeCapability => 141,
            HeaderType::UploadFirmware => 140,
            HeaderType::BeginFirmwareUpgrade => 139,
            HeaderType::FinishFirmwareUpgrade => 138,
            HeaderType::SwitchEncryptionCode => 137,
            HeaderType::StoreEncryptionCode => 136,
            HeaderType::SetAcceptLimit => 135,
            HeaderType::DispenseHopperValue => 134,
            HeaderType::RequestHopperPollingValue => 133,
            HeaderType::EmergencyStopValue => 132,
            HeaderType::RequestHopperCoinValue => 131,
            HeaderType::RequestIndexedHopperDispenseCount => 130,
            HeaderType::ReadBarcodeData => 129,
            HeaderType::RequestMoneyIn => 128,
            HeaderType::RequestMoneyOut => 127,
            HeaderType::ClearMoneyCounters => 126,
            HeaderType::PayMoneyOut => 125,
            HeaderType::VerifyMoneyOut => 124,
            HeaderType::RequestActivityRegister => 123,
            HeaderType::RequestErrorStatus => 122,
            HeaderType::PurgeHopper => 121,
            HeaderType::ModifyHopperBalance => 120,
            HeaderType::RequestHopperBalance => 119,
            HeaderType::ModifyCashboxValue => 118,
            HeaderType::RequestCashboxValue => 117,
            HeaderType::ModifyRealTimeClock => 116,
            HeaderType::RequestRealTimeClock => 115,
            HeaderType::RequestUSBId => 114,
            HeaderType::SwitchBaudRate => 113,
            HeaderType::ReadEncryptedEvents => 112,
            HeaderType::RequestEncryptionSupport => 111,
            HeaderType::SwitchEncryptionKey => 110,
            HeaderType::RequestEncryptedHopperStatus => 109,
            HeaderType::RequestEncryptedMonetaryId => 108,
            HeaderType::RequestCommsRevision => 4,
            HeaderType::ClearCommsStatusVariables => 3,
            HeaderType::RequestCommsStatusVariables => 2,
            HeaderType::ResetDevice => 1,
            HeaderType::Reply => 0,
            HeaderType::Unknown(n) => n,
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum CoinAcceptorError {
    RejectCoin,
    InhibitedCoin,
    MultipleWindow,
    WakeUpTimeout,
    ValidationTimeout,
    CreditSensorTimeout,
    SorterOptoTimeout,
    SecondCloseCoinError,
    AcceptGateNotReady,
    CreditSensorNotReady,
    SorterNotReady,
    RejectCoinNotCleared,
    ValidationSensorNotReady,
    CreditSensorBlocked,
    SorterOptoBlocked,
    CreditSequenceError,
    CoinGoingBackwards,
    CoinTooFastCreditSensor,
    CoinTooSlowCreditSensor,
    CoinOnStringActive,
    DCEOptoTimeout,
    DCEOptoNotSeen,
    CreditSensorReachedEarly,
    RejectCoinRepeatedly,
    RejectSlug,
    RejectSensorBlocked,
    GamesOverload,
    MaxCoinMeterPulsesExceeded,
    AcceptGateOpenNotClosed,
    AcceptGateClosedNotOpen,
    ManifoldOptoTimeout,
    ManifoldOptoBlocked,
    ManifoldNotReady,
    SecurityStatusChanged,
    MotorException,
    SwallowedCoin,
    CoinTooFastValidationSensor,
    CoinTooSlowValidationSensor,
    CoinIncorrectlySorted,
    ExternalLightAttack,
    InhibitedCoinType1,
    InhibitedCoinType2,
    InhibitedCoinType3,
    InhibitedCoinType4,
    InhibitedCoinType5,
    InhibitedCoinType6,
    DataBlockRequest,
    FlightDeckOpen,
    UnspecifiedAlarm,
    Unknown(u8),
}

impl CoinAcceptorError {
    #[allow(dead_code)]
    pub fn from_u8(n: u8) -> CoinAcceptorError {
        match n {
            1 => CoinAcceptorError::RejectCoin,
            2 => CoinAcceptorError::InhibitedCoin,
            3 => CoinAcceptorError::MultipleWindow,
            4 => CoinAcceptorError::WakeUpTimeout,
            5 => CoinAcceptorError::ValidationTimeout,
            6 => CoinAcceptorError::CreditSensorTimeout,
            7 => CoinAcceptorError::SorterOptoTimeout,
            8 => CoinAcceptorError::SecondCloseCoinError,
            9 => CoinAcceptorError::AcceptGateNotReady,
            10 => CoinAcceptorError::CreditSensorNotReady,
            11 => CoinAcceptorError::SorterNotReady,
            12 => CoinAcceptorError::RejectCoinNotCleared,
            13 => CoinAcceptorError::ValidationSensorNotReady,
            14 => CoinAcceptorError::CreditSensorBlocked,
            15 => CoinAcceptorError::SorterOptoBlocked,
            16 => CoinAcceptorError::CreditSequenceError,
            17 => CoinAcceptorError::CoinGoingBackwards,
            18 => CoinAcceptorError::CoinTooFastCreditSensor,
            19 => CoinAcceptorError::CoinTooSlowCreditSensor,
            20 => CoinAcceptorError::CoinOnStringActive,
            21 => CoinAcceptorError::DCEOptoTimeout,
            22 => CoinAcceptorError::DCEOptoNotSeen,
            23 => CoinAcceptorError::CreditSensorReachedEarly,
            24 => CoinAcceptorError::RejectCoinRepeatedly,
            25 => CoinAcceptorError::RejectSlug,
            26 => CoinAcceptorError::RejectSensorBlocked,
            27 => CoinAcceptorError::GamesOverload,
            28 => CoinAcceptorError::MaxCoinMeterPulsesExceeded,
            29 => CoinAcceptorError::AcceptGateOpenNotClosed,
            30 => CoinAcceptorError::AcceptGateClosedNotOpen,
            31 => CoinAcceptorError::ManifoldOptoTimeout,
            32 => CoinAcceptorError::ManifoldOptoBlocked,
            33 => CoinAcceptorError::ManifoldNotReady,
            34 => CoinAcceptorError::SecurityStatusChanged,
            35 => CoinAcceptorError::MotorException,
            36 => CoinAcceptorError::SwallowedCoin,
            37 => CoinAcceptorError::CoinTooFastValidationSensor,
            38 => CoinAcceptorError::CoinTooSlowValidationSensor,
            39 => CoinAcceptorError::CoinIncorrectlySorted,
            40 => CoinAcceptorError::ExternalLightAttack,
            128 => CoinAcceptorError::InhibitedCoinType1,
            129 => CoinAcceptorError::InhibitedCoinType2,
            130 => CoinAcceptorError::InhibitedCoinType3,
            131 => CoinAcceptorError::InhibitedCoinType4,
            132 => CoinAcceptorError::InhibitedCoinType5,
            133 => CoinAcceptorError::InhibitedCoinType6,
            253 => CoinAcceptorError::DataBlockRequest,
            254 => CoinAcceptorError::FlightDeckOpen,
            255 => CoinAcceptorError::UnspecifiedAlarm,
            _ => CoinAcceptorError::Unknown(n),
        }
    }

    #[allow(dead_code)]
    pub fn to_u8(&self) -> u8 {
        match *self {
            CoinAcceptorError::RejectCoin => 1,
            CoinAcceptorError::InhibitedCoin => 2,
            CoinAcceptorError::MultipleWindow => 3,
            CoinAcceptorError::WakeUpTimeout => 4,
            CoinAcceptorError::ValidationTimeout => 5,
            CoinAcceptorError::CreditSensorTimeout => 6,
            CoinAcceptorError::SorterOptoTimeout => 7,
            CoinAcceptorError::SecondCloseCoinError => 8,
            CoinAcceptorError::AcceptGateNotReady => 9,
            CoinAcceptorError::CreditSensorNotReady => 10,
            CoinAcceptorError::SorterNotReady => 11,
            CoinAcceptorError::RejectCoinNotCleared => 12,
            CoinAcceptorError::ValidationSensorNotReady => 13,
            CoinAcceptorError::CreditSensorBlocked => 14,
            CoinAcceptorError::SorterOptoBlocked => 15,
            CoinAcceptorError::CreditSequenceError => 16,
            CoinAcceptorError::CoinGoingBackwards => 17,
            CoinAcceptorError::CoinTooFastCreditSensor => 18,
            CoinAcceptorError::CoinTooSlowCreditSensor => 19,
            CoinAcceptorError::CoinOnStringActive => 20,
            CoinAcceptorError::DCEOptoTimeout => 21,
            CoinAcceptorError::DCEOptoNotSeen => 22,
            CoinAcceptorError::CreditSensorReachedEarly => 23,
            CoinAcceptorError::RejectCoinRepeatedly => 24,
            CoinAcceptorError::RejectSlug => 25,
            CoinAcceptorError::RejectSensorBlocked => 26,
            CoinAcceptorError::GamesOverload => 27,
            CoinAcceptorError::MaxCoinMeterPulsesExceeded => 28,
            CoinAcceptorError::AcceptGateOpenNotClosed => 29,
            CoinAcceptorError::AcceptGateClosedNotOpen => 30,
            CoinAcceptorError::ManifoldOptoTimeout => 31,
            CoinAcceptorError::ManifoldOptoBlocked => 32,
            CoinAcceptorError::ManifoldNotReady => 33,
            CoinAcceptorError::SecurityStatusChanged => 34,
            CoinAcceptorError::MotorException => 35,
            CoinAcceptorError::SwallowedCoin => 36,
            CoinAcceptorError::CoinTooFastValidationSensor => 37,
            CoinAcceptorError::CoinTooSlowValidationSensor => 38,
            CoinAcceptorError::CoinIncorrectlySorted => 39,
            CoinAcceptorError::ExternalLightAttack => 40,
            CoinAcceptorError::InhibitedCoinType1 => 128,
            CoinAcceptorError::InhibitedCoinType2 => 129,
            CoinAcceptorError::InhibitedCoinType3 => 130,
            CoinAcceptorError::InhibitedCoinType4 => 131,
            CoinAcceptorError::InhibitedCoinType5 => 132,
            CoinAcceptorError::InhibitedCoinType6 => 133,
            CoinAcceptorError::DataBlockRequest => 253,
            CoinAcceptorError::FlightDeckOpen => 254,
            CoinAcceptorError::UnspecifiedAlarm => 255,
            CoinAcceptorError::Unknown(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BillEvent {
    BillTypeValidatedAndSent1,
    BillTypeValidatedAndSent2,
    BillTypeValidatedAndSent3,
    BillTypeValidatedAndSent4,
    BillTypeValidatedAndSent5,
    BillTypeValidatedAndSent6,
    BillTypeValidatedAndHeld1,
    BillTypeValidatedAndHeld2,
    BillTypeValidatedAndHeld3,
    BillTypeValidatedAndHeld4,
    BillTypeValidatedAndHeld5,
    BillTypeValidatedAndHeld6,
    MasterInhibitActive,
    BillReturnedFromEscrow,
    InvalidBillValidation,
    InvalidBillTransport,
    InhibitedBillSerial,
    InhibitedBillDIP,
    BillJammedInTransportUnsafe,
    BillJammedInStacker,
    BillPulledBackwards,
    BillTamper,
    StackerOK,
    StackerRemoved,
    StackerInserted,
    StackerFaulty,
    StackerFull,
    StackerJammed,
    BillJammedInTransportSafe,
    OptoFraudDetected,
    StringFraudDetected,
    AntiStringMechFaulty,
    BarcodeDetected,
    UnknownBillTypeStacked,
    Unknown(u8, u8),
}

impl BillEvent {
    #[allow(dead_code)]
    pub fn from_u8(n: (u8, u8)) -> BillEvent {
        match n {
            (1, 0) => BillEvent::BillTypeValidatedAndSent1,
            (2, 0) => BillEvent::BillTypeValidatedAndSent2,
            (3, 0) => BillEvent::BillTypeValidatedAndSent3,
            (4, 0) => BillEvent::BillTypeValidatedAndSent4,
            (5, 0) => BillEvent::BillTypeValidatedAndSent5,
            (6, 0) => BillEvent::BillTypeValidatedAndSent6,
            (1, 1) => BillEvent::BillTypeValidatedAndHeld1,
            (2, 1) => BillEvent::BillTypeValidatedAndHeld2,
            (3, 1) => BillEvent::BillTypeValidatedAndHeld3,
            (4, 1) => BillEvent::BillTypeValidatedAndHeld4,
            (5, 1) => BillEvent::BillTypeValidatedAndHeld5,
            (6, 1) => BillEvent::BillTypeValidatedAndHeld6,
            (0, 0) => BillEvent::MasterInhibitActive,
            (0, 1) => BillEvent::BillReturnedFromEscrow,
            (0, 2) => BillEvent::InvalidBillValidation,
            (0, 3) => BillEvent::InvalidBillTransport,
            (0, 4) => BillEvent::InhibitedBillSerial,
            (0, 5) => BillEvent::InhibitedBillDIP,
            (0, 6) => BillEvent::BillJammedInTransportUnsafe,
            (0, 7) => BillEvent::BillJammedInStacker,
            (0, 8) => BillEvent::BillPulledBackwards,
            (0, 9) => BillEvent::BillTamper,
            (0, 10) => BillEvent::StackerOK,
            (0, 11) => BillEvent::StackerRemoved,
            (0, 12) => BillEvent::StackerInserted,
            (0, 13) => BillEvent::StackerFaulty,
            (0, 14) => BillEvent::StackerFull,
            (0, 15) => BillEvent::StackerJammed,
            (0, 16) => BillEvent::BillJammedInTransportSafe,
            (0, 17) => BillEvent::OptoFraudDetected,
            (0, 18) => BillEvent::StringFraudDetected,
            (0, 19) => BillEvent::AntiStringMechFaulty,
            (0, 20) => BillEvent::BarcodeDetected,
            (0, 21) => BillEvent::UnknownBillTypeStacked,
            (a, b) => BillEvent::Unknown(a, b),
        }
    }

    #[allow(dead_code)]
    pub fn to_u8(&self) -> (u8, u8) {
        match *self {
            BillEvent::BillTypeValidatedAndSent1 => (1, 0),
            BillEvent::BillTypeValidatedAndSent2 => (2, 0),
            BillEvent::BillTypeValidatedAndSent3 => (3, 0),
            BillEvent::BillTypeValidatedAndSent4 => (4, 0),
            BillEvent::BillTypeValidatedAndSent5 => (5, 0),
            BillEvent::BillTypeValidatedAndSent6 => (6, 0),
            BillEvent::BillTypeValidatedAndHeld1 => (1, 1),
            BillEvent::BillTypeValidatedAndHeld2 => (2, 1),
            BillEvent::BillTypeValidatedAndHeld3 => (3, 1),
            BillEvent::BillTypeValidatedAndHeld4 => (4, 1),
            BillEvent::BillTypeValidatedAndHeld5 => (5, 1),
            BillEvent::BillTypeValidatedAndHeld6 => (6, 1),
            BillEvent::MasterInhibitActive => (0, 0),
            BillEvent::BillReturnedFromEscrow => (0, 1),
            BillEvent::InvalidBillValidation => (0, 2),
            BillEvent::InvalidBillTransport => (0, 3),
            BillEvent::InhibitedBillSerial => (0, 4),
            BillEvent::InhibitedBillDIP => (0, 5),
            BillEvent::BillJammedInTransportUnsafe => (0, 6),
            BillEvent::BillJammedInStacker => (0, 7),
            BillEvent::BillPulledBackwards => (0, 8),
            BillEvent::BillTamper => (0, 9),
            BillEvent::StackerOK => (0, 10),
            BillEvent::StackerRemoved => (0, 11),
            BillEvent::StackerInserted => (0, 12),
            BillEvent::StackerFaulty => (0, 13),
            BillEvent::StackerFull => (0, 14),
            BillEvent::StackerJammed => (0, 15),
            BillEvent::BillJammedInTransportSafe => (0, 16),
            BillEvent::OptoFraudDetected => (0, 17),
            BillEvent::StringFraudDetected => (0, 18),
            BillEvent::AntiStringMechFaulty => (0, 19),
            BillEvent::BarcodeDetected => (0, 20),
            BillEvent::UnknownBillTypeStacked => (0, 21),
            BillEvent::Unknown(a, b) => (a, b),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Payload {
    pub header: HeaderType,
    pub data: Data,
}

#[allow(dead_code)]
impl Payload {
    pub fn encode(&self) -> Vec<u8> {
        let mut data_temp = self.data.clone();
        data_temp.insert(0, self.header.to_u8());
        data_temp
    }

    pub fn sum(&self) -> u16 {
        let payload = self.encode();
        let mut payload_sum = 0u16;
        for num in &payload {
            payload_sum += *num as u16;
        }
        payload_sum
    }

    pub fn as_str(&self) -> Result<String, str::Utf8Error> {
        let res = str::from_utf8(&self.data);
        match res {
            Ok(result) => Ok(String::from(result)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ErrorType {
    PartialMessage,
    ChecksumError,
    NotAReply,
    NoResponse,
    ParseError,
}

#[derive(Debug)]
pub struct Message {
    pub destination: Address,
    pub length: u8,
    pub source: Address,
    pub payload: Payload,
    pub checksum_type: ChecksumType,
}

impl Message {
    pub fn new(destination: Address, source: Address, payload: Payload, checksum_type: ChecksumType) -> Message {
        let length = payload.data.len();
        Message {
            destination: destination,
            length: length as u8,
            source: source,
            payload: payload,
            checksum_type: checksum_type,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut temp = Vec::<u8>::new();
        temp.push(self.destination);
        temp.push(self.length);

        match self.checksum_type {
            ChecksumType::SimpleChecksum => {
                temp.push(self.source);
                temp.append(&mut self.payload.encode());
                temp.push(self.calc_checksum());
            }
            ChecksumType::CRCChecksum => {
                let crc = self.calc_own_crc();
                temp.push(crc[0]);
                temp.append(&mut self.payload.encode());
                temp.push(crc[1]);
            }
        }

        temp
    }

    pub fn decode(raw: &mut Vec<u8>) -> Result<Message, ErrorType> {

        // debug!("Decoding raw: {:?}", raw);

        let msg_length = raw.len() as u16;

        if msg_length < 2 {
            return Err(ErrorType::PartialMessage);
        }

        let data_length = raw[1];
        let expected_msg_length = data_length as u16 + 5;

        if msg_length < expected_msg_length {
            return Err(ErrorType::PartialMessage);
        };

        let mut raw_msg: Vec<u8> = raw.drain(0..expected_msg_length as usize).collect();
        // don't touch raw after this, it's the next message

        let checksum_type: ChecksumType;
        let source: u8;

        if Message::validate_checksum(&raw_msg) {
            checksum_type = ChecksumType::SimpleChecksum;
            source = raw_msg[2];
        } else {
            if Message::validate_crc(&raw_msg) {
                checksum_type = ChecksumType::CRCChecksum;
                source = 1; // Source address is always 1 in CRC mode
            } else {
                return Err(ErrorType::ChecksumError);
            }
        }

        let mut raw_data = raw_msg.split_off(4);

        // remove checksum
        raw_data.pop();

        let destination = raw_msg[0];
        let header_int = raw_msg[3];
        let payload = Payload {
            header: HeaderType::from_u8(header_int),
            data: raw_data,
        };

        Ok(Message {
               destination: destination,
               length: data_length,
               source: source,
               payload: payload,
               checksum_type: checksum_type,
           })
    }

    pub fn calc_checksum(&self) -> u8 {

        let payload_sum = self.payload.sum();

        let sum: u16 = payload_sum + self.destination as u16 + self.length as u16 + self.source as u16;

        let checksum = sum % 256;

        if checksum == 0 {
            0u8
        } else {
            (256 - checksum) as u8
        }
    }

    pub fn calc_own_crc(&self) -> CRC {

        let mut data = Vec::<u8>::new();

        data.push(self.destination);
        data.push(self.length);
        data.append(&mut self.payload.encode());

        Message::calc_crc(&data)
    }

    pub fn calc_crc(data: &Vec<u8>) -> CRC {
        let poly = 0x1021;
        let mut crc = 0u16;

        for byte in data {
            crc ^= (*byte as u16) << 8 & 0xffff;
            for _ in 0..8 {
                if (crc & 0x8000) != 0 {
                    crc = ((crc << 1) ^ poly) & 0xffff;
                } else {
                    crc <<= 1;
                    crc &= 0xffff;
                }
            }
        }
        [(crc & 0xff) as u8, (crc >> 8 & 0xff) as u8]
    }

    pub fn validate_checksum(raw: &Vec<u8>) -> bool {
        if raw.is_empty() {
            error!("Validate checksum called on empty message!");
            return false;
        }

        let mut sum = 0u16;
        for num in raw {
            sum += *num as u16;
        }
        let rem = sum % 256;
        rem == 0
    }

    pub fn validate_crc(raw: &Vec<u8>) -> bool {
        if raw.is_empty() {
            error!("Validate CRC called on empty message!");
            return false;
        }

        let mut data = raw.clone();
        let crc: [u8; 2] = [data.remove(2), data.pop().unwrap()];

        crc == Message::calc_crc(&data)
    }
}
