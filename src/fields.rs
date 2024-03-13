use std::fmt;
use std::fmt::{Display, Formatter};
use crate::message_types::MessageType;

#[derive(PartialEq)]
pub struct Field {
    pub number: u8,
    pub name: &'static str,
    pub message_type: MessageType,
    pub translate_enum: fn(&Field, &u8) -> String,
}


impl fmt::Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entry(&self.number)
            .finish()
    }
}

impl Field {
    const fn from(message_type: MessageType, number: u8, name: &'static str) -> Self {
        Field {
            message_type,
            number,
            name,
            translate_enum: |me, value| -> String {
                format!("Cannot translate {}", value)
            },
        }
    }
    const fn from_with_converter(message_type: MessageType, number: u8, name: &'static str, translate_enum: fn(&Field, &u8) -> String) -> Self {
        Field {
            message_type,
            number,
            name,
            translate_enum,
        }
    }
    // file_id (0)
    pub const TYPE_FIELD: Field = Field::from_with_converter(MessageType::FILE_ID, 0, "Type", |me, value| -> String {
        FileType::resolve(value).to_string()
    });
    pub const MANUFACTURER_FIELD: Field = Field::from(MessageType::FILE_ID, 1, "Manufacturer");
    pub const PRODUCT_FIELD: Field = Field::from(MessageType::FILE_ID, 2, "Product");
    pub const SERIAL_NUMBER_FIELD: Field = Field::from(MessageType::FILE_ID, 3, "Serial number");
    pub const TIME_CREATED_FIELD: Field = Field::from(MessageType::FILE_ID, 4, "Time created");
    pub const NUMBER_FIELD: Field = Field::from(MessageType::FILE_ID, 5, "Number");
    pub const PRODUCT_NAME_FIELD: Field = Field::from(MessageType::FILE_ID, 8, "Product name");
    pub const UNKNOWN: Field = Field::from(MessageType::UNKNOWN, 0, "Unknown");

    // capabilities (1)
    pub const LANGUAGES_FIELD: Field = Field::from(MessageType::CAPABILITIES, 0, "Languages");
    pub const SPORTS_FIELD: Field = Field::from(MessageType::CAPABILITIES, 1, "Sports");
    pub const WORKOUTS_SUPPORTED_FIELD: Field = Field::from(MessageType::CAPABILITIES, 21, "Workouts supported");
    pub const CONNECTIVITY_SUPPORTED_FIELD: Field = Field::from(MessageType::CAPABILITIES, 23, "Connectivity supported");

    // device settings (2)
    pub const ACTIVE_TIME_ZONE: Field = Field::from(MessageType::DEVICE_SETTINGS, 0, "Active time zone");
    pub const UTC_OFFSET: Field = Field::from(MessageType::DEVICE_SETTINGS, 1, "UTC offset");
    pub const TIME_OFFSET: Field = Field::from(MessageType::DEVICE_SETTINGS, 2, "Time offset");
    pub const TIME_MODE: Field = Field::from(MessageType::DEVICE_SETTINGS, 4, "Time mode");
    pub const TIME_ZONE_OFFSET: Field = Field::from(MessageType::DEVICE_SETTINGS, 5, "Time zone offset");
    pub const BACKLIGHT_MODE: Field = Field::from(MessageType::DEVICE_SETTINGS, 12, "Backlight mode");
    pub const ACTIVITY_TRACKER_ENABLED: Field = Field::from(MessageType::DEVICE_SETTINGS, 36, "Activity tracker enabled");
    pub const CLOCK_TIME: Field = Field::from(MessageType::DEVICE_SETTINGS, 39, "Clock time");
    pub const PAGES_ENABLED: Field = Field::from(MessageType::DEVICE_SETTINGS, 40, "Pages enabled");
    pub const MOVE_ALERT_ENABLED: Field = Field::from(MessageType::DEVICE_SETTINGS, 46, "Move alert enabled");
    pub const DATE_MODE: Field = Field::from(MessageType::DEVICE_SETTINGS, 47, "Date mode");
    pub const DISPLAY_ORIENTATION: Field = Field::from(MessageType::DEVICE_SETTINGS, 55, "Display orientation");
    pub const MOUNTING_SIDE: Field = Field::from(MessageType::DEVICE_SETTINGS, 56, "Mounting side");
    pub const DEFAULT_PAGE: Field = Field::from(MessageType::DEVICE_SETTINGS, 57, "Default page");
    pub const AUTOSYNC_MIN_STEPS: Field = Field::from(MessageType::DEVICE_SETTINGS, 58, "Autosync min. steps");
    pub const AUTOSYNC_MAX_STEPS: Field = Field::from(MessageType::DEVICE_SETTINGS, 59, "Autosync max. steps");
    pub const LACTATE_THRESHOLD_AUTODETECT_ENABLED: Field = Field::from(MessageType::DEVICE_SETTINGS, 80, "Lactate threshold autodetect enabled");
    pub const BLE_AUTO_UPLOAD_ENABLED: Field = Field::from(MessageType::DEVICE_SETTINGS, 86, "BLE auto upload enabled");
    pub const AUTO_SYNC_FREQUENCY: Field = Field::from(MessageType::DEVICE_SETTINGS, 89, "Auto sync frequency");
    pub const AUTO_ACTIVITY_DETECT: Field = Field::from(MessageType::DEVICE_SETTINGS, 90, "Auto activity detect");
    pub const NUMBER_OF_SCREENS: Field = Field::from(MessageType::DEVICE_SETTINGS, 94, "Number of screens");
    pub const SMART_NOTIFICATION_DISPLAY_ORIENTATION: Field = Field::from(MessageType::DEVICE_SETTINGS, 95, "Smart notification display orientation");
    pub const TAP_INTERFACE: Field = Field::from(MessageType::DEVICE_SETTINGS, 134, "Tap interface");
    pub const TAP_SENSITIVITY: Field = Field::from(MessageType::DEVICE_SETTINGS, 174, "Tap sensitivity");

    // file creator (49)
    pub const SOFTWARE_VERSION: Field = Field::from(MessageType::FILE_CREATOR, 0, "Software version");
    pub const HARDWARE_VERSION: Field = Field::from(MessageType::FILE_CREATOR, 1, "Hardware version");

    // sport (12)
    pub const SPORT_SPORT: Field = Field::from_with_converter(MessageType::SPORT, 0, "Sport", |me, value| -> String {
        Sport::resolve(value).to_string()
    });
    pub const SPORT_SUB_SPORT: Field = Field::from(MessageType::SPORT, 1, "Sub-sport");
    pub const SPORT_SPORT_NAME: Field = Field::from(MessageType::SPORT, 3, "Sport name");

    // totals (33)
    pub const TOTALS_MESSAGE_INDEX: Field = Field::from(MessageType::TOTALS, 254, "Totals message index");
    pub const TOTALS_TIMESTAMP: Field = Field::from(MessageType::TOTALS, 253, "Totals timestamp");
    pub const TOTALS_TIMER_TIME: Field = Field::from(MessageType::TOTALS, 0, "Totals timer time");
    pub const TOTALS_DISTANCE: Field = Field::from(MessageType::TOTALS, 1, "Totals distance");
    pub const TOTALS_CALORIES: Field = Field::from(MessageType::TOTALS, 2, "Totals calories");
    pub const TOTALS_SPORT: Field = Field::from(MessageType::TOTALS, 3, "Totals sport");
    pub const TOTALS_ELAPSED_TIME: Field = Field::from(MessageType::TOTALS, 4, "Totals elapsed time");
    pub const TOTALS_SESSIONS: Field = Field::from(MessageType::TOTALS, 5, "Totals sessions");
    pub const TOTALS_ACTIVE_TIME: Field = Field::from(MessageType::TOTALS, 6, "Totals active time");
    pub const TOTALS_SPORT_INDEX: Field = Field::from(MessageType::TOTALS, 9, "Totals sport");

    // activity (34)
    pub const ACTIVITY_TIMESTAMP: Field = Field::from(MessageType::ACTIVITY, 253, "Activity timestamp");
    pub const ACTIVITY_TOTAL_TIMER_TIME: Field = Field::from(MessageType::ACTIVITY, 0, "Activity total timer time");
    pub const ACTIVITY_NUM_SESSIONS: Field = Field::from(MessageType::ACTIVITY, 1, "Activity number of session");
    pub const ACTIVITY_TYPE: Field = Field::from(MessageType::ACTIVITY, 2, "Activity type");
    pub const ACTIVITY_EVENT: Field = Field::from(MessageType::ACTIVITY, 3, "Activity event");
    pub const ACTIVITY_EVENT_TYPE: Field = Field::from(MessageType::ACTIVITY, 4, "Activity event type");
    pub const ACTIVITY_LOCAL_TIMESTAMP: Field = Field::from(MessageType::ACTIVITY, 5, "Activity local timestamp");
    pub const ACTIVITY_EVENT_GROUP: Field = Field::from(MessageType::ACTIVITY, 6, "Activity event group");

    const ALL_FIELDS: [Field; 57] = [
        Field::TYPE_FIELD,
        Field::MANUFACTURER_FIELD,
        Field::PRODUCT_FIELD,
        Field::SERIAL_NUMBER_FIELD,
        Field::TIME_CREATED_FIELD,
        Field::NUMBER_FIELD,
        Field::PRODUCT_NAME_FIELD,
        Field::LANGUAGES_FIELD,
        Field::SPORTS_FIELD,
        Field::WORKOUTS_SUPPORTED_FIELD,
        Field::ACTIVE_TIME_ZONE,
        Field::UTC_OFFSET,
        Field::TIME_OFFSET,
        Field::TIME_MODE,
        Field::TIME_ZONE_OFFSET,
        Field::BACKLIGHT_MODE,
        Field::ACTIVITY_TRACKER_ENABLED,
        Field::CLOCK_TIME,
        Field::PAGES_ENABLED,
        Field::MOVE_ALERT_ENABLED,
        Field::DATE_MODE,
        Field::DISPLAY_ORIENTATION,
        Field::MOUNTING_SIDE,
        Field::DEFAULT_PAGE,
        Field::AUTOSYNC_MIN_STEPS,
        Field::AUTOSYNC_MAX_STEPS,
        Field::LACTATE_THRESHOLD_AUTODETECT_ENABLED,
        Field::BLE_AUTO_UPLOAD_ENABLED,
        Field::AUTO_SYNC_FREQUENCY,
        Field::AUTO_ACTIVITY_DETECT,
        Field::NUMBER_OF_SCREENS,
        Field::SMART_NOTIFICATION_DISPLAY_ORIENTATION,
        Field::TAP_INTERFACE,
        Field::TAP_SENSITIVITY,
        Field::SOFTWARE_VERSION,
        Field::HARDWARE_VERSION,
        Field::SPORT_SPORT,
        Field::SPORT_SUB_SPORT,
        Field::SPORT_SPORT_NAME,
        Field::TOTALS_MESSAGE_INDEX,
        Field::TOTALS_TIMESTAMP,
        Field::TOTALS_TIMER_TIME,
        Field::TOTALS_DISTANCE,
        Field::TOTALS_CALORIES,
        Field::TOTALS_SPORT,
        Field::TOTALS_ELAPSED_TIME,
        Field::TOTALS_SESSIONS,
        Field::TOTALS_ACTIVE_TIME,
        Field::TOTALS_SPORT_INDEX,
        Field::ACTIVITY_TIMESTAMP,
        Field::ACTIVITY_TOTAL_TIMER_TIME,
        Field::ACTIVITY_NUM_SESSIONS,
        Field::ACTIVITY_TYPE,
        Field::ACTIVITY_EVENT,
        Field::ACTIVITY_EVENT_TYPE,
        Field::ACTIVITY_LOCAL_TIMESTAMP,
        Field::ACTIVITY_EVENT_GROUP,
    ];
    pub(crate) fn parse(i: u8, definition_message_type: &MessageType) -> Field {
        for field in Self::ALL_FIELDS {
            if field.number == i && field.message_type.number == definition_message_type.number {
                return field;
            }
        }
        return Self::UNKNOWN;
    }
}

#[derive(Debug)]
pub enum Sport {
    Generic,
    Running,
    Cycling,
    Transition,
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    Tennis,
    AmericanFootball,
    Training,
    Walking,
    CrossCountrySkiing,
    AlpineSkiing,
    Snowboarding,
    Rowing,
    Mountaineering,
    Hiking,
    Multisport,
    Paddling,
    Flying,
    EBiking,
    Motorcycling,
    Boating,
    Driving,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    Fishing,
    InlineSkating,
    RockClimbing,
    Sailing,
    IceSkating,
    SkyDiving,
    Snowshoeing,
    Snowmobiling,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    WaterSkiing,
    Kayaking,
    Rafting,
    Windsurfing,
    Kitesurfing,
    Tactical,
    Jumpmaster,
    Boxing,
    FloorClimbing,
    Baseball,
    Diving,
    Hiit,
    Racket,
    WheelchairPushWalk,
    WheelchairPushRun,
    Meditation,
    DiscGolf,
    Cricket,
    Rugby,
    Hockey,
    Lacrosse,
    Volleyball,
    WaterTubing,
    Wakesurfing,
    MixedMartialArts,
    Snorkeling,
    Dance,
    JumpRope,
    All,
    Invalid,
}

impl Display for Sport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Sport {
    pub fn resolve(enum_value: &u8) -> Self {
        return match enum_value {
            0 => Sport::Generic,
            1 => Sport::Running,
            2 => Sport::Cycling,
            3 => Sport::Transition,
            4 => Sport::FitnessEquipment,
            5 => Sport::Swimming,
            6 => Sport::Basketball,
            7 => Sport::Soccer,
            8 => Sport::Tennis,
            9 => Sport::AmericanFootball,
            10 => Sport::Training,
            11 => Sport::Walking,
            12 => Sport::CrossCountrySkiing,
            13 => Sport::AlpineSkiing,
            14 => Sport::Snowboarding,
            15 => Sport::Rowing,
            16 => Sport::Mountaineering,
            17 => Sport::Hiking,
            18 => Sport::Multisport,
            19 => Sport::Paddling,
            20 => Sport::Flying,
            21 => Sport::EBiking,
            22 => Sport::Motorcycling,
            23 => Sport::Boating,
            24 => Sport::Driving,
            25 => Sport::Golf,
            26 => Sport::HangGliding,
            27 => Sport::HorsebackRiding,
            28 => Sport::Hunting,
            29 => Sport::Fishing,
            30 => Sport::InlineSkating,
            31 => Sport::RockClimbing,
            32 => Sport::Sailing,
            33 => Sport::IceSkating,
            34 => Sport::SkyDiving,
            35 => Sport::Snowshoeing,
            36 => Sport::Snowmobiling,
            37 => Sport::StandUpPaddleboarding,
            38 => Sport::Surfing,
            39 => Sport::Wakeboarding,
            40 => Sport::WaterSkiing,
            41 => Sport::Kayaking,
            42 => Sport::Rafting,
            43 => Sport::Windsurfing,
            44 => Sport::Kitesurfing,
            45 => Sport::Tactical,
            46 => Sport::Jumpmaster,
            47 => Sport::Boxing,
            48 => Sport::FloorClimbing,
            49 => Sport::Baseball,
            53 => Sport::Diving,
            62 => Sport::Hiit,
            64 => Sport::Racket,
            65 => Sport::WheelchairPushWalk,
            66 => Sport::WheelchairPushRun,
            67 => Sport::Meditation,
            69 => Sport::DiscGolf,
            71 => Sport::Cricket,
            72 => Sport::Rugby,
            73 => Sport::Hockey,
            74 => Sport::Lacrosse,
            75 => Sport::Volleyball,
            76 => Sport::WaterTubing,
            77 => Sport::Wakesurfing,
            80 => Sport::MixedMartialArts,
            82 => Sport::Snorkeling,
            83 => Sport::Dance,
            84 => Sport::JumpRope,
            254 => Sport::All,
            255 => Sport::Invalid,
            _ => Sport::Invalid,
        };
    }
}

#[derive(Debug)]
enum FileType {
    DEVICE,
    SETTINGS,
    SPORT,
    ACTIVITY,
    WORKOUT,
    COURSE,
    SCHEDULES,
    WEIGHT,
    TOTALS,
    GOALS,
    BLOOD_PRESSURE,
    MONITORING_A,
    ACTIVITY_SUMMARY,
    MONITORING_DAILY,
    MONITORING_B,
    SEGMENT,
    SEGMENT_LIST,
    EXD_CONFIGURATION,
    MFG_RANGE_MIN,
    MFG_RANGE_MAX,
    INVALID,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FileType {
    pub fn resolve(enum_value: &u8) -> FileType {
        return match enum_value {
            1 => FileType::DEVICE,
            2 => FileType::SETTINGS,
            3 => FileType::SPORT,
            4 => FileType::ACTIVITY,
            5 => FileType::WORKOUT,
            6 => FileType::COURSE,
            7 => FileType::SCHEDULES,
            9 => FileType::WEIGHT,
            10 => FileType::TOTALS,
            11 => FileType::GOALS,
            14 => FileType::BLOOD_PRESSURE,
            15 => FileType::MONITORING_A,
            20 => FileType::ACTIVITY_SUMMARY,
            28 => FileType::MONITORING_DAILY,
            32 => FileType::MONITORING_B,
            34 => FileType::SEGMENT,
            35 => FileType::SEGMENT_LIST,
            40 => FileType::EXD_CONFIGURATION,
            0xF7 => FileType::MFG_RANGE_MIN,
            0xFE => FileType::MFG_RANGE_MAX,
            255 => FileType::INVALID,
            _ => FileType::INVALID
        };
    }
}
