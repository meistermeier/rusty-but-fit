use std::collections::HashMap;

use crate::data_types::{BaseType, Value};
use crate::fields::Field;
use crate::Message;

pub struct MessageDefinition {
    pub message_type: MessageType,
    pub fields: Vec<FieldDefinition>,
}

impl MessageDefinition {
    pub fn read(&self, current_position: &usize, buffer: &Vec<u8>) -> (Message, usize) {
        let mut position = current_position.clone();
        let mut data_map = HashMap::new();
        for field_definition in self.fields.iter().clone() {
            let end = position + (field_definition.size as usize);
            let data = &buffer[position..end];
            let value = ((field_definition.base_type).read)(&field_definition.base_type, data);
            let data_field = &field_definition.field;
            position += field_definition.size as usize;
            data_map.insert(data_field.clone(), value.clone());
        }
        (
            Message {
                message_type: self.message_type.clone(),
                data: data_map,
            },
            position,
        )
    }
}

pub struct FieldDefinition {
    pub field: Field,
    pub number: u8, // still here for debugging purposes
    pub size: u8,
    pub base_type: BaseType,
}

pub struct MessageType {
    pub number: u16,
    pub name: &'static str,
    known_fields: &'static [Field],
}

impl PartialEq for MessageType {
    fn eq(&self, other: &Self) -> bool {
        self.number.eq(&other.number)
    }
}

impl Clone for MessageType {
    fn clone(&self) -> Self {
        MessageType {
            number: self.number,
            name: self.name,
            known_fields: self.known_fields.clone(),
        }
    }
}

impl MessageType {
    pub const FILE_ID: MessageType = MessageType {
        number: 0,
        name: "File Id",
        known_fields: &[Field::PRODUCT_MANUFACTURER],
    };
    pub const CAPABILITIES: MessageType = MessageType {
        number: 1,
        name: "Capabilities",
        known_fields: &[],
    };
    pub const DEVICE_SETTINGS: MessageType = MessageType {
        number: 2,
        name: "Device settings",
        known_fields: &[],
    };
    pub const USER_PROFILE: MessageType = MessageType {
        number: 3,
        name: "User profile",
        known_fields: &[],
    };
    pub const HRM_PROFILE: MessageType = MessageType {
        number: 4,
        name: "HRM profile",
        known_fields: &[],
    };
    pub const SDM_PROFILE: MessageType = MessageType {
        number: 5,
        name: "SDM profile",
        known_fields: &[],
    };
    pub const BIKE_PROFILE: MessageType = MessageType {
        number: 6,
        name: "Bike profile",
        known_fields: &[],
    };
    pub const ZONES_TARGET: MessageType = MessageType {
        number: 7,
        name: "Zones target",
        known_fields: &[],
    };
    pub const HR_ZONE: MessageType = MessageType {
        number: 8,
        name: "HR zone",
        known_fields: &[],
    };
    pub const POWER_ZONE: MessageType = MessageType {
        number: 9,
        name: "Power zone",
        known_fields: &[],
    };
    pub const MET_ZONE: MessageType = MessageType {
        number: 10,
        name: "MET zone",
        known_fields: &[],
    };
    pub const SPORT: MessageType = MessageType {
        number: 12,
        name: "Sport",
        known_fields: &[],
    };
    pub const GOAL: MessageType = MessageType {
        number: 15,
        name: "Goal",
        known_fields: &[],
    };
    pub const SESSION: MessageType = MessageType {
        number: 18,
        name: "Session",
        known_fields: &[],
    };
    pub const LAP: MessageType = MessageType {
        number: 19,
        name: "Lap",
        known_fields: &[],
    };
    pub const RECORD: MessageType = MessageType {
        number: 20,
        name: "Record",
        known_fields: &[],
    };
    pub const EVENT: MessageType = MessageType {
        number: 21,
        name: "Event",
        known_fields: &[],
    };
    pub const DEVICE_INFO: MessageType = MessageType {
        number: 23,
        name: "Device info",
        known_fields: &[],
    };
    pub const WORKOUT: MessageType = MessageType {
        number: 26,
        name: "Workout",
        known_fields: &[],
    };
    pub const WORKOUT_STEP: MessageType = MessageType {
        number: 27,
        name: "Workout step",
        known_fields: &[],
    };
    pub const SCHEDULE: MessageType = MessageType {
        number: 28,
        name: "Schedule",
        known_fields: &[],
    };
    pub const WEIGHT_SCALE: MessageType = MessageType {
        number: 30,
        name: "Weight scale",
        known_fields: &[],
    };
    pub const COURSE: MessageType = MessageType {
        number: 31,
        name: "Course",
        known_fields: &[],
    };
    pub const COURSE_POINT: MessageType = MessageType {
        number: 32,
        name: "Course point",
        known_fields: &[],
    };
    pub const TOTALS: MessageType = MessageType {
        number: 33,
        name: "Totals",
        known_fields: &[],
    };
    pub const ACTIVITY: MessageType = MessageType {
        number: 34,
        name: "Activity",
        known_fields: &[],
    };
    pub const SOFTWARE: MessageType = MessageType {
        number: 35,
        name: "Software",
        known_fields: &[],
    };
    pub const FILE_CAPABILITIES: MessageType = MessageType {
        number: 37,
        name: "File capabilities",
        known_fields: &[],
    };
    pub const MESSAGE_CAPABILITIES: MessageType = MessageType {
        number: 38,
        name: "Message capabilities",
        known_fields: &[],
    };
    pub const FIELD_CAPABILITIES: MessageType = MessageType {
        number: 39,
        name: "Field capabilities",
        known_fields: &[],
    };
    pub const FILE_CREATOR: MessageType = MessageType {
        number: 49,
        name: "File creator",
        known_fields: &[],
    };
    pub const BLOOD_PRESSURE: MessageType = MessageType {
        number: 51,
        name: "Blood pressure",
        known_fields: &[],
    };
    pub const SPEED_ZONE: MessageType = MessageType {
        number: 53,
        name: "Speed zone",
        known_fields: &[],
    };
    pub const MONITORING: MessageType = MessageType {
        number: 55,
        name: "Monitoring",
        known_fields: &[],
    };
    pub const TRAINING_FILE: MessageType = MessageType {
        number: 72,
        name: "Training file",
        known_fields: &[],
    };
    pub const HRV: MessageType = MessageType {
        number: 78,
        name: "HRV",
        known_fields: &[],
    };
    pub const ANT_RX: MessageType = MessageType {
        number: 80,
        name: "ANT rx",
        known_fields: &[],
    };
    pub const ANT_TX: MessageType = MessageType {
        number: 81,
        name: "ANT tx",
        known_fields: &[],
    };
    pub const ANT_CHANNEL_ID: MessageType = MessageType {
        number: 82,
        name: "ANT channel id",
        known_fields: &[],
    };
    pub const LENGTH: MessageType = MessageType {
        number: 101,
        name: "Length",
        known_fields: &[],
    };
    pub const MONITORING_INFO: MessageType = MessageType {
        number: 103,
        name: "Monitoring info",
        known_fields: &[],
    };
    pub const PAD: MessageType = MessageType {
        number: 105,
        name: "Pad",
        known_fields: &[],
    };
    pub const SLAVE_DEVICE: MessageType = MessageType {
        number: 106,
        name: "Slave device",
        known_fields: &[],
    };
    pub const CONNECTIVITY: MessageType = MessageType {
        number: 127,
        name: "Connectivity",
        known_fields: &[],
    };
    pub const WEATHER_CONDITIONS: MessageType = MessageType {
        number: 128,
        name: "Weather conditions",
        known_fields: &[],
    };
    pub const WEATHER_ALERT: MessageType = MessageType {
        number: 129,
        name: "Weather alert",
        known_fields: &[],
    };
    pub const CADENCE_ZONE: MessageType = MessageType {
        number: 131,
        name: "Cadence zone",
        known_fields: &[],
    };
    pub const HR: MessageType = MessageType {
        number: 132,
        name: "HR",
        known_fields: &[],
    };
    pub const SEGMENT_LAP: MessageType = MessageType {
        number: 142,
        name: "Segment lap",
        known_fields: &[],
    };
    pub const MEMO_GLOB: MessageType = MessageType {
        number: 145,
        name: "Memo glob",
        known_fields: &[],
    };
    pub const SEGMENT_ID: MessageType = MessageType {
        number: 148,
        name: "Segment id",
        known_fields: &[],
    };
    pub const SEGMENT_LEADERBOARD_ENTRY: MessageType = MessageType {
        number: 149,
        name: "Segment leaderboard entry",
        known_fields: &[],
    };
    pub const SEGMENT_POINT: MessageType = MessageType {
        number: 150,
        name: "Segment point",
        known_fields: &[],
    };
    pub const SEGMENT_FILE: MessageType = MessageType {
        number: 151,
        name: "Segment file",
        known_fields: &[],
    };
    pub const WORKOUT_SESSION: MessageType = MessageType {
        number: 158,
        name: "Workout session",
        known_fields: &[],
    };
    pub const WATCHFACE_SETTINGS: MessageType = MessageType {
        number: 159,
        name: "Watchface settings",
        known_fields: &[],
    };
    pub const GPS_METADATA: MessageType = MessageType {
        number: 160,
        name: "GPS Metadata",
        known_fields: &[],
    };
    pub const CAMERA_EVENT: MessageType = MessageType {
        number: 161,
        name: "Camera event",
        known_fields: &[],
    };
    pub const TIMESTAMP_CORRELATION: MessageType = MessageType {
        number: 162,
        name: "Timestamp correlation",
        known_fields: &[],
    };
    pub const GYROSCOPE_DATA: MessageType = MessageType {
        number: 164,
        name: "Gyroscope data",
        known_fields: &[],
    };
    pub const ACCELEROMETER_DATA: MessageType = MessageType {
        number: 165,
        name: "Accelerometer data",
        known_fields: &[],
    };
    pub const THREE_D_SENSOR_CALIBRATION: MessageType = MessageType {
        number: 167,
        name: "3D sensor calibration",
        known_fields: &[],
    };
    pub const VIDEO_FRAME: MessageType = MessageType {
        number: 169,
        name: "Video frame",
        known_fields: &[],
    };
    pub const OBD_II_DATA: MessageType = MessageType {
        number: 174,
        name: "OBD II data",
        known_fields: &[],
    };
    pub const NMEA_SENTENCE: MessageType = MessageType {
        number: 177,
        name: "NMEA sentence",
        known_fields: &[],
    };
    pub const AVIATION_ATTITUDE: MessageType = MessageType {
        number: 178,
        name: "Aviation attitude",
        known_fields: &[],
    };
    pub const VIDEO: MessageType = MessageType {
        number: 184,
        name: "Video",
        known_fields: &[],
    };
    pub const VIDEO_TITLE: MessageType = MessageType {
        number: 185,
        name: "Video title",
        known_fields: &[],
    };
    pub const VIDEO_DESCRIPTION: MessageType = MessageType {
        number: 186,
        name: "Video description",
        known_fields: &[],
    };
    pub const VIDEO_CLIP: MessageType = MessageType {
        number: 187,
        name: "Video clip",
        known_fields: &[],
    };
    pub const OHR_SETTINGS: MessageType = MessageType {
        number: 188,
        name: "OHR settings",
        known_fields: &[],
    };
    pub const EXD_SCREEN_CONFIGURATION: MessageType = MessageType {
        number: 200,
        name: "EXD screen configuration",
        known_fields: &[],
    };
    pub const EXD_DATA_FIELD_CONFIGURATION: MessageType = MessageType {
        number: 201,
        name: "EXD data field configuration",
        known_fields: &[],
    };
    pub const EXD_DATA_CONCEPT_CONFIGURATION: MessageType = MessageType {
        number: 202,
        name: "EXD data concept configuration",
        known_fields: &[],
    };
    pub const FIELD_DESCRIPTION: MessageType = MessageType {
        number: 206,
        name: "Field description",
        known_fields: &[],
    };
    pub const DEVELOPER_DATA_ID: MessageType = MessageType {
        number: 207,
        name: "Developer data id",
        known_fields: &[],
    };
    pub const MAGNETOMETER_DATA: MessageType = MessageType {
        number: 208,
        name: "Magnetometer data",
        known_fields: &[],
    };
    pub const BAROMETER_DATA: MessageType = MessageType {
        number: 209,
        name: "Barometer data",
        known_fields: &[],
    };
    pub const ONE_D_SENSOR_CALIBRATION: MessageType = MessageType {
        number: 210,
        name: "1D sensor calibration",
        known_fields: &[],
    };
    pub const TIME_IN_ZONE: MessageType = MessageType {
        number: 216,
        name: "Time in zone",
        known_fields: &[],
    };
    pub const SET: MessageType = MessageType {
        number: 225,
        name: "Set",
        known_fields: &[],
    };
    pub const STRESS_LEVEL: MessageType = MessageType {
        number: 227,
        name: "Stress level",
        known_fields: &[],
    };
    pub const DIVE_SETTINGS: MessageType = MessageType {
        number: 258,
        name: "Dive settings",
        known_fields: &[],
    };
    pub const DIVE_GAS: MessageType = MessageType {
        number: 259,
        name: "Dive gas",
        known_fields: &[],
    };
    pub const DIVE_ALARM: MessageType = MessageType {
        number: 262,
        name: "Dive alarm",
        known_fields: &[],
    };
    pub const EXERCISE_TITLE: MessageType = MessageType {
        number: 264,
        name: "Exercise title",
        known_fields: &[],
    };
    pub const DIVE_SUMMARY: MessageType = MessageType {
        number: 268,
        name: "Dive summary",
        known_fields: &[],
    };
    pub const JUMP: MessageType = MessageType {
        number: 285,
        name: "Jump",
        known_fields: &[],
    };
    pub const SPLIT: MessageType = MessageType {
        number: 312,
        name: "Split",
        known_fields: &[],
    };
    pub const SPLIT_SUMMARY: MessageType = MessageType {
        number: 313,
        name: "Split summary",
        known_fields: &[],
    };
    pub const CLIMB_PRO: MessageType = MessageType {
        number: 317,
        name: "Climb pro",
        known_fields: &[],
    };
    pub const DEVICE_AUX_BATTERY_INFO: MessageType = MessageType {
        number: 375,
        name: "Device AUX battery info",
        known_fields: &[],
    };
    pub const MFG_RANGE_MIN: MessageType = MessageType {
        number: 0xFF00,
        name: "MFG range min",
        known_fields: &[],
    };
    pub const MFG_RANGE_MAX: MessageType = MessageType {
        number: 0xFFFE,
        name: "MFG range max",
        known_fields: &[],
    };
    pub const UNDOCUMENTED_AUDIO: MessageType = MessageType {
        number: 147,
        name: "Audio (undocumented)",
        known_fields: &[],
    };
    pub const UNKNOWN: MessageType = MessageType {
        number: 1024,
        name: "Unknown",
        known_fields: &[],
    };

    pub fn get_message_type(&self) -> MessageType {
        Self::FILE_ID
    }

    pub fn resolve(i: u16) -> MessageType {
        match i {
            0 => Self::FILE_ID,
            1 => Self::CAPABILITIES,
            2 => Self::DEVICE_SETTINGS,
            3 => Self::USER_PROFILE,
            4 => Self::HRM_PROFILE,
            5 => Self::SDM_PROFILE,
            6 => Self::BIKE_PROFILE,
            7 => Self::ZONES_TARGET,
            8 => Self::HR_ZONE,
            9 => Self::POWER_ZONE,
            10 => Self::MET_ZONE,
            12 => Self::SPORT,
            15 => Self::GOAL,
            18 => Self::SESSION,
            19 => Self::LAP,
            20 => Self::RECORD,
            21 => Self::EVENT,
            23 => Self::DEVICE_INFO,
            26 => Self::WORKOUT,
            27 => Self::WORKOUT_STEP,
            28 => Self::SCHEDULE,
            30 => Self::WEIGHT_SCALE,
            31 => Self::COURSE,
            32 => Self::COURSE_POINT,
            33 => Self::TOTALS,
            34 => Self::ACTIVITY,
            35 => Self::SOFTWARE,
            37 => Self::FILE_CAPABILITIES,
            38 => Self::MESSAGE_CAPABILITIES,
            39 => Self::FIELD_CAPABILITIES,
            49 => Self::FILE_CREATOR,
            51 => Self::BLOOD_PRESSURE,
            53 => Self::SPEED_ZONE,
            55 => Self::MONITORING,
            72 => Self::TRAINING_FILE,
            78 => Self::HRV,
            80 => Self::ANT_RX,
            81 => Self::ANT_TX,
            82 => Self::ANT_CHANNEL_ID,
            101 => Self::LENGTH,
            103 => Self::MONITORING_INFO,
            105 => Self::PAD,
            106 => Self::SLAVE_DEVICE,
            127 => Self::CONNECTIVITY,
            128 => Self::WEATHER_CONDITIONS,
            129 => Self::WEATHER_ALERT,
            131 => Self::CADENCE_ZONE,
            132 => Self::HR,
            142 => Self::SEGMENT_LAP,
            145 => Self::MEMO_GLOB,
            148 => Self::SEGMENT_ID,
            149 => Self::SEGMENT_LEADERBOARD_ENTRY,
            150 => Self::SEGMENT_POINT,
            151 => Self::SEGMENT_FILE,
            158 => Self::WORKOUT_SESSION,
            159 => Self::WATCHFACE_SETTINGS,
            160 => Self::GPS_METADATA,
            161 => Self::CAMERA_EVENT,
            162 => Self::TIMESTAMP_CORRELATION,
            164 => Self::GYROSCOPE_DATA,
            165 => Self::ACCELEROMETER_DATA,
            167 => Self::THREE_D_SENSOR_CALIBRATION,
            169 => Self::VIDEO_FRAME,
            174 => Self::OBD_II_DATA,
            177 => Self::NMEA_SENTENCE,
            178 => Self::AVIATION_ATTITUDE,
            184 => Self::VIDEO,
            185 => Self::VIDEO_TITLE,
            186 => Self::VIDEO_DESCRIPTION,
            187 => Self::VIDEO_CLIP,
            188 => Self::OHR_SETTINGS,
            200 => Self::EXD_SCREEN_CONFIGURATION,
            201 => Self::EXD_DATA_FIELD_CONFIGURATION,
            202 => Self::EXD_DATA_CONCEPT_CONFIGURATION,
            206 => Self::FIELD_DESCRIPTION,
            207 => Self::DEVELOPER_DATA_ID,
            208 => Self::MAGNETOMETER_DATA,
            209 => Self::BAROMETER_DATA,
            210 => Self::ONE_D_SENSOR_CALIBRATION,
            216 => Self::TIME_IN_ZONE,
            225 => Self::SET,
            227 => Self::STRESS_LEVEL,
            258 => Self::DIVE_SETTINGS,
            259 => Self::DIVE_GAS,
            262 => Self::DIVE_ALARM,
            264 => Self::EXERCISE_TITLE,
            268 => Self::DIVE_SUMMARY,
            285 => Self::JUMP,
            312 => Self::SPLIT,
            313 => Self::SPLIT_SUMMARY,
            317 => Self::CLIMB_PRO,
            375 => Self::DEVICE_AUX_BATTERY_INFO,
            0xFF00 => Self::MFG_RANGE_MIN,
            0xFFFE => Self::MFG_RANGE_MAX,
            147 => Self::UNDOCUMENTED_AUDIO,
            1024 | _ => Self::UNKNOWN,
        }
    }
}
