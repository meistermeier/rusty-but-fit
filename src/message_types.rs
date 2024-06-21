use std::collections::HashMap;

use serde_with::serde_derive::Serialize;

use crate::data_types::BaseType;
use crate::fields::Field;
use crate::fit_file::FitFileConfig;
use crate::message::MessageMap;
use crate::Message;

pub struct MessageDefinition {
    pub message_type: MessageType,
    pub fields: Vec<FieldDefinition>,
}

impl MessageDefinition {
    pub fn read(
        &self,
        current_position: &usize,
        buffer: &Vec<u8>,
        config: &FitFileConfig,
    ) -> (Message, usize) {
        let print_unknown = config.include_unknown_fields;
        let print_invalid = config.include_invalid_values;
        let mut position = current_position.clone();
        let mut data_map = HashMap::new();
        for field_definition in self.fields.iter().clone() {
            let end = position + (field_definition.size as usize);
            let data = &buffer[position..end];
            let value = ((field_definition.base_type).read)(&field_definition.base_type, data);
            let data_field = &field_definition.field;
            position += field_definition.size as usize;
            if (!data_field.is_unknown() || print_unknown) && (!value.is_invalid() || print_invalid)
            {
                data_map.insert(data_field.clone(), value.clone());
            }
        }
        (
            Message::from(self.message_type.clone(), MessageMap { data: data_map }),
            position,
        )
    }
}

pub struct FieldDefinition {
    pub field: Field,
    #[allow(dead_code)]
    pub number: u8, // still here for debugging purposes
    pub size: u8,
    pub base_type: BaseType,
}

#[derive(Serialize)]
pub struct MessageType {
    pub number: u16,
    pub name: &'static str,
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
        }
    }
}

impl MessageType {
    pub const FILE_ID: MessageType = MessageType {
        number: 0,
        name: "File Id",
    };
    pub const CAPABILITIES: MessageType = MessageType {
        number: 1,
        name: "Capabilities",
    };
    pub const DEVICE_SETTINGS: MessageType = MessageType {
        number: 2,
        name: "Device settings",
    };
    pub const USER_PROFILE: MessageType = MessageType {
        number: 3,
        name: "User profile",
    };
    pub const HRM_PROFILE: MessageType = MessageType {
        number: 4,
        name: "HRM profile",
    };
    pub const SDM_PROFILE: MessageType = MessageType {
        number: 5,
        name: "SDM profile",
    };
    pub const BIKE_PROFILE: MessageType = MessageType {
        number: 6,
        name: "Bike profile",
    };
    pub const ZONES_TARGET: MessageType = MessageType {
        number: 7,
        name: "Zones target",
    };
    pub const HR_ZONE: MessageType = MessageType {
        number: 8,
        name: "HR zone",
    };
    pub const POWER_ZONE: MessageType = MessageType {
        number: 9,
        name: "Power zone",
    };
    pub const MET_ZONE: MessageType = MessageType {
        number: 10,
        name: "MET zone",
    };
    pub const SPORT: MessageType = MessageType {
        number: 12,
        name: "Sport",
    };
    pub const GOAL: MessageType = MessageType {
        number: 15,
        name: "Goal",
    };
    pub const SESSION: MessageType = MessageType {
        number: 18,
        name: "Session",
    };
    pub const LAP: MessageType = MessageType {
        number: 19,
        name: "Lap",
    };
    pub const RECORD: MessageType = MessageType {
        number: 20,
        name: "Record",
    };
    pub const EVENT: MessageType = MessageType {
        number: 21,
        name: "Event",
    };
    pub const DEVICE_INFO: MessageType = MessageType {
        number: 23,
        name: "Device info",
    };
    pub const WORKOUT: MessageType = MessageType {
        number: 26,
        name: "Workout",
    };
    pub const WORKOUT_STEP: MessageType = MessageType {
        number: 27,
        name: "Workout step",
    };
    pub const SCHEDULE: MessageType = MessageType {
        number: 28,
        name: "Schedule",
    };
    pub const WEIGHT_SCALE: MessageType = MessageType {
        number: 30,
        name: "Weight scale",
    };
    pub const COURSE: MessageType = MessageType {
        number: 31,
        name: "Course",
    };
    pub const COURSE_POINT: MessageType = MessageType {
        number: 32,
        name: "Course point",
    };
    pub const TOTALS: MessageType = MessageType {
        number: 33,
        name: "Totals",
    };
    pub const ACTIVITY: MessageType = MessageType {
        number: 34,
        name: "Activity",
    };
    pub const SOFTWARE: MessageType = MessageType {
        number: 35,
        name: "Software",
    };
    pub const FILE_CAPABILITIES: MessageType = MessageType {
        number: 37,
        name: "File capabilities",
    };
    pub const MESSAGE_CAPABILITIES: MessageType = MessageType {
        number: 38,
        name: "Message capabilities",
    };
    pub const FIELD_CAPABILITIES: MessageType = MessageType {
        number: 39,
        name: "Field capabilities",
    };
    pub const FILE_CREATOR: MessageType = MessageType {
        number: 49,
        name: "File creator",
    };
    pub const BLOOD_PRESSURE: MessageType = MessageType {
        number: 51,
        name: "Blood pressure",
    };
    pub const SPEED_ZONE: MessageType = MessageType {
        number: 53,
        name: "Speed zone",
    };
    pub const MONITORING: MessageType = MessageType {
        number: 55,
        name: "Monitoring",
    };
    pub const TRAINING_FILE: MessageType = MessageType {
        number: 72,
        name: "Training file",
    };
    pub const HRV: MessageType = MessageType {
        number: 78,
        name: "HRV",
    };
    pub const ANT_RX: MessageType = MessageType {
        number: 80,
        name: "ANT rx",
    };
    pub const ANT_TX: MessageType = MessageType {
        number: 81,
        name: "ANT tx",
    };
    pub const ANT_CHANNEL_ID: MessageType = MessageType {
        number: 82,
        name: "ANT channel id",
    };
    pub const LENGTH: MessageType = MessageType {
        number: 101,
        name: "Length",
    };
    pub const MONITORING_INFO: MessageType = MessageType {
        number: 103,
        name: "Monitoring info",
    };
    pub const PAD: MessageType = MessageType {
        number: 105,
        name: "Pad",
    };
    pub const SLAVE_DEVICE: MessageType = MessageType {
        number: 106,
        name: "Slave device",
    };
    pub const CONNECTIVITY: MessageType = MessageType {
        number: 127,
        name: "Connectivity",
    };
    pub const WEATHER_CONDITIONS: MessageType = MessageType {
        number: 128,
        name: "Weather conditions",
    };
    pub const WEATHER_ALERT: MessageType = MessageType {
        number: 129,
        name: "Weather alert",
    };
    pub const CADENCE_ZONE: MessageType = MessageType {
        number: 131,
        name: "Cadence zone",
    };
    pub const HR: MessageType = MessageType {
        number: 132,
        name: "HR",
    };
    pub const SEGMENT_LAP: MessageType = MessageType {
        number: 142,
        name: "Segment lap",
    };
    pub const MEMO_GLOB: MessageType = MessageType {
        number: 145,
        name: "Memo glob",
    };
    pub const SEGMENT_ID: MessageType = MessageType {
        number: 148,
        name: "Segment id",
    };
    pub const SEGMENT_LEADERBOARD_ENTRY: MessageType = MessageType {
        number: 149,
        name: "Segment leaderboard entry",
    };
    pub const SEGMENT_POINT: MessageType = MessageType {
        number: 150,
        name: "Segment point",
    };
    pub const SEGMENT_FILE: MessageType = MessageType {
        number: 151,
        name: "Segment file",
    };
    pub const WORKOUT_SESSION: MessageType = MessageType {
        number: 158,
        name: "Workout session",
    };
    pub const WATCHFACE_SETTINGS: MessageType = MessageType {
        number: 159,
        name: "Watchface settings",
    };
    pub const GPS_METADATA: MessageType = MessageType {
        number: 160,
        name: "GPS Metadata",
    };
    pub const CAMERA_EVENT: MessageType = MessageType {
        number: 161,
        name: "Camera event",
    };
    pub const TIMESTAMP_CORRELATION: MessageType = MessageType {
        number: 162,
        name: "Timestamp correlation",
    };
    pub const GYROSCOPE_DATA: MessageType = MessageType {
        number: 164,
        name: "Gyroscope data",
    };
    pub const ACCELEROMETER_DATA: MessageType = MessageType {
        number: 165,
        name: "Accelerometer data",
    };
    pub const THREE_D_SENSOR_CALIBRATION: MessageType = MessageType {
        number: 167,
        name: "3D sensor calibration",
    };
    pub const VIDEO_FRAME: MessageType = MessageType {
        number: 169,
        name: "Video frame",
    };
    pub const OBD_II_DATA: MessageType = MessageType {
        number: 174,
        name: "OBD II data",
    };
    pub const NMEA_SENTENCE: MessageType = MessageType {
        number: 177,
        name: "NMEA sentence",
    };
    pub const AVIATION_ATTITUDE: MessageType = MessageType {
        number: 178,
        name: "Aviation attitude",
    };
    pub const VIDEO: MessageType = MessageType {
        number: 184,
        name: "Video",
    };
    pub const VIDEO_TITLE: MessageType = MessageType {
        number: 185,
        name: "Video title",
    };
    pub const VIDEO_DESCRIPTION: MessageType = MessageType {
        number: 186,
        name: "Video description",
    };
    pub const VIDEO_CLIP: MessageType = MessageType {
        number: 187,
        name: "Video clip",
    };
    pub const OHR_SETTINGS: MessageType = MessageType {
        number: 188,
        name: "OHR settings",
    };
    pub const EXD_SCREEN_CONFIGURATION: MessageType = MessageType {
        number: 200,
        name: "EXD screen configuration",
    };
    pub const EXD_DATA_FIELD_CONFIGURATION: MessageType = MessageType {
        number: 201,
        name: "EXD data field configuration",
    };
    pub const EXD_DATA_CONCEPT_CONFIGURATION: MessageType = MessageType {
        number: 202,
        name: "EXD data concept configuration",
    };
    pub const FIELD_DESCRIPTION: MessageType = MessageType {
        number: 206,
        name: "Field description",
    };
    pub const DEVELOPER_DATA_ID: MessageType = MessageType {
        number: 207,
        name: "Developer data id",
    };
    pub const MAGNETOMETER_DATA: MessageType = MessageType {
        number: 208,
        name: "Magnetometer data",
    };
    pub const BAROMETER_DATA: MessageType = MessageType {
        number: 209,
        name: "Barometer data",
    };
    pub const ONE_D_SENSOR_CALIBRATION: MessageType = MessageType {
        number: 210,
        name: "1D sensor calibration",
    };
    pub const TIME_IN_ZONE: MessageType = MessageType {
        number: 216,
        name: "Time in zone",
    };
    pub const SET: MessageType = MessageType {
        number: 225,
        name: "Set",
    };
    pub const STRESS_LEVEL: MessageType = MessageType {
        number: 227,
        name: "Stress level",
    };
    pub const DIVE_SETTINGS: MessageType = MessageType {
        number: 258,
        name: "Dive settings",
    };
    pub const DIVE_GAS: MessageType = MessageType {
        number: 259,
        name: "Dive gas",
    };
    pub const DIVE_ALARM: MessageType = MessageType {
        number: 262,
        name: "Dive alarm",
    };
    pub const EXERCISE_TITLE: MessageType = MessageType {
        number: 264,
        name: "Exercise title",
    };
    pub const DIVE_SUMMARY: MessageType = MessageType {
        number: 268,
        name: "Dive summary",
    };
    pub const JUMP: MessageType = MessageType {
        number: 285,
        name: "Jump",
    };
    pub const SPLIT: MessageType = MessageType {
        number: 312,
        name: "Split",
    };
    pub const SPLIT_SUMMARY: MessageType = MessageType {
        number: 313,
        name: "Split summary",
    };
    pub const CLIMB_PRO: MessageType = MessageType {
        number: 317,
        name: "Climb pro",
    };
    pub const DEVICE_AUX_BATTERY_INFO: MessageType = MessageType {
        number: 375,
        name: "Device AUX battery info",
    };
    pub const MFG_RANGE_MIN: MessageType = MessageType {
        number: 0xFF00,
        name: "MFG range min",
    };
    pub const MFG_RANGE_MAX: MessageType = MessageType {
        number: 0xFFFE,
        name: "MFG range max",
    };
    pub const UNDOCUMENTED_CONNECTED_DEVICES: MessageType = MessageType {
        number: 147,
        name: "Connected devices (undocumented)",
    };
    pub const UNKNOWN: MessageType = MessageType {
        number: 1024,
        name: "Unknown",
    };

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
            147 => Self::UNDOCUMENTED_CONNECTED_DEVICES,
            1024 | _ => Self::UNKNOWN,
        }
    }
}
