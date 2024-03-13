#[derive(Debug, PartialEq)]
pub struct MessageType {
    pub number: u16,
    pub name: &'static str,
}

impl MessageType {
    // pub : MessageType = MessageTypeconst INVALID                                                    FIT_UINT16_INVALID;
    pub const UNKNOWN: MessageType = MessageType { number: 1024, name: "Unknown" };
    pub const FILE_ID: MessageType = MessageType { number: 0, name: "File Id" };
    pub const CAPABILITIES: MessageType = MessageType { number: 1, name: "Capabilities" };
    pub const DEVICE_SETTINGS: MessageType = MessageType { number: 2, name: "Device settings" };
    pub const USER_PROFILE: MessageType = MessageType { number: 3, name: "User profile" };
    pub const HRM_PROFILE: MessageType = MessageType { number: 4, name: "HRM profile" };
    pub const SDM_PROFILE: MessageType = MessageType { number: 5, name: "SDM profile" };
    pub const BIKE_PROFILE: MessageType = MessageType { number: 6, name: "Bike profile" };
    pub const ZONES_TARGET: MessageType = MessageType { number: 7, name: "Zones target" };
    pub const HR_ZONE: MessageType = MessageType { number: 8, name: "HR zone" };
    pub const POWER_ZONE: MessageType = MessageType { number: 9, name: "Power zone" };
    pub const MET_ZONE: MessageType = MessageType { number: 10, name: "MET zone" };
    pub const SPORT: MessageType = MessageType { number: 12, name: "Sport" };
    pub const GOAL: MessageType = MessageType { number: 15, name: "Goal" };
    pub const SESSION: MessageType = MessageType { number: 18, name: "Session" };
    pub const LAP: MessageType = MessageType { number: 19, name: "Lap" };
    pub const RECORD: MessageType = MessageType { number: 20, name: "Record" };
    pub const EVENT: MessageType = MessageType { number: 21, name: "Event" };
    pub const DEVICE_INFO: MessageType = MessageType { number: 23, name: "Device info" };
    pub const WORKOUT: MessageType = MessageType { number: 26, name: "Workout" };
    pub const WORKOUT_STEP: MessageType = MessageType { number: 27, name: "Workout step" };
    pub const SCHEDULE: MessageType = MessageType { number: 28, name: "Schedule" };
    pub const WEIGHT_SCALE: MessageType = MessageType { number: 30, name: "Weight scale" };
    pub const COURSE: MessageType = MessageType { number: 31, name: "Course" };
    pub const COURSE_POINT: MessageType = MessageType { number: 32, name: "Course point" };
    pub const TOTALS: MessageType = MessageType { number: 33, name: "Totals" };
    pub const ACTIVITY: MessageType = MessageType { number: 34, name: "Activity" };
    pub const SOFTWARE: MessageType = MessageType { number: 35, name: "Software" };
    pub const FILE_CAPABILITIES: MessageType = MessageType { number: 37, name: "File capabilities" };
    pub const MESG_CAPABILITIES: MessageType = MessageType { number: 38, name: "Message capabilities" };
    pub const FIELD_CAPABILITIES: MessageType = MessageType { number: 39, name: "Field capabilities" };
    pub const FILE_CREATOR: MessageType = MessageType { number: 49, name: "File creator" };
    pub const BLOOD_PRESSURE: MessageType = MessageType { number: 51, name: "Blood pressure" };
    pub const SPEED_ZONE: MessageType = MessageType { number: 53, name: "Speed zone" };
    pub const MONITORING: MessageType = MessageType { number: 55, name: "Monitoring" };
    pub const TRAINING_FILE: MessageType = MessageType { number: 72, name: "Training file" };
    pub const HRV: MessageType = MessageType { number: 78, name: "HRV" };
    pub const ANT_RX: MessageType = MessageType { number: 80, name: "ANT rx" };
    pub const ANT_TX: MessageType = MessageType { number: 81, name: "ANT tx" };
    pub const ANT_CHANNEL_ID: MessageType = MessageType { number: 82, name: "ANT channel id" };
    pub const LENGTH: MessageType = MessageType { number: 101, name: "Length" };
    pub const MONITORING_INFO: MessageType = MessageType { number: 103, name: "Monitoring info" };
    pub const PAD: MessageType = MessageType { number: 105, name: "Pad" };
    pub const SLAVE_DEVICE: MessageType = MessageType { number: 106, name: "Slave device" };
    pub const CONNECTIVITY: MessageType = MessageType { number: 127, name: "Connectivity" };
    pub const WEATHER_CONDITIONS: MessageType = MessageType { number: 128, name: "Weather conditions" };
    pub const WEATHER_ALERT: MessageType = MessageType { number: 129, name: "Weather alert" };
    pub const CADENCE_ZONE: MessageType = MessageType { number: 131, name: "Cadence zone" };
    pub const HR: MessageType = MessageType { number: 132, name: "HR" };
    pub const SEGMENT_LAP: MessageType = MessageType { number: 142, name: "Segment lap" };
    pub const MEMO_GLOB: MessageType = MessageType { number: 145, name: "Memo glob" };
    pub const SEGMENT_ID: MessageType = MessageType { number: 148, name: "Segment id" };
    pub const SEGMENT_LEADERBOARD_ENTRY: MessageType = MessageType { number: 149, name: "Segment leaderboard entry" };
    pub const SEGMENT_POINT: MessageType = MessageType { number: 150, name: "Segment point" };
    pub const SEGMENT_FILE: MessageType = MessageType { number: 151, name: "Segment file" };
    pub const WORKOUT_SESSION: MessageType = MessageType { number: 158, name: "Workout session" };
    pub const WATCHFACE_SETTINGS: MessageType = MessageType { number: 159, name: "Watchface settings" };
    pub const GPS_METADATA: MessageType = MessageType { number: 160, name: "GPS Metadata" };
    pub const CAMERA_EVENT: MessageType = MessageType { number: 161, name: "Camera event" };
    pub const TIMESTAMP_CORRELATION: MessageType = MessageType { number: 162, name: "Timestamp correlation" };
    pub const GYROSCOPE_DATA: MessageType = MessageType { number: 164, name: "Gyroscope data" };
    pub const ACCELEROMETER_DATA: MessageType = MessageType { number: 165, name: "Accelerometer data" };
    pub const THREE_D_SENSOR_CALIBRATION: MessageType = MessageType { number: 167, name: "3D sensor calibration" };
    pub const VIDEO_FRAME: MessageType = MessageType { number: 169, name: "Video frame" };
    pub const OBDII_DATA: MessageType = MessageType { number: 174, name: "OBD II data" };
    pub const NMEA_SENTENCE: MessageType = MessageType { number: 177, name: "NMEA sentence" };
    pub const AVIATION_ATTITUDE: MessageType = MessageType { number: 178, name: "Aviation attitude" };
    pub const VIDEO: MessageType = MessageType { number: 184, name: "Video" };
    pub const VIDEO_TITLE: MessageType = MessageType { number: 185, name: "Video title" };
    pub const VIDEO_DESCRIPTION: MessageType = MessageType { number: 186, name: "Video description" };
    pub const VIDEO_CLIP: MessageType = MessageType { number: 187, name: "Video clip" };
    pub const OHR_SETTINGS: MessageType = MessageType { number: 188, name: "OHR settings" };
    pub const EXD_SCREEN_CONFIGURATION: MessageType = MessageType { number: 200, name: "EXD screen configuration" };
    pub const EXD_DATA_FIELD_CONFIGURATION: MessageType = MessageType { number: 201, name: "EXD data field configuration" };
    pub const EXD_DATA_CONCEPT_CONFIGURATION: MessageType = MessageType { number: 202, name: "EXD data concept configuration" };
    pub const FIELD_DESCRIPTION: MessageType = MessageType { number: 206, name: "Field description" };
    pub const DEVELOPER_DATA_ID: MessageType = MessageType { number: 207, name: "Developer data id" };
    pub const MAGNETOMETER_DATA: MessageType = MessageType { number: 208, name: "Magnetometer data" };
    pub const BAROMETER_DATA: MessageType = MessageType { number: 209, name: "Barometer data" };
    pub const ONE_D_SENSOR_CALIBRATION: MessageType = MessageType { number: 210, name: "1D sensor calibration" };
    pub const SET: MessageType = MessageType { number: 225, name: "Set" };
    pub const STRESS_LEVEL: MessageType = MessageType { number: 227, name: "Stress level" };
    pub const DIVE_SETTINGS: MessageType = MessageType { number: 258, name: "Dive settings" };
    pub const DIVE_GAS: MessageType = MessageType { number: 259, name: "Dive gas" };
    pub const DIVE_ALARM: MessageType = MessageType { number: 262, name: "Dive alarm" };
    pub const EXERCISE_TITLE: MessageType = MessageType { number: 264, name: "Exercise title" };
    pub const DIVE_SUMMARY: MessageType = MessageType { number: 268, name: "Dive summary" };
    pub const JUMP: MessageType = MessageType { number: 285, name: "Jump" };
    pub const CLIMB_PRO: MessageType = MessageType { number: 317, name: "Climb pro" };
    pub const DEVICE_AUX_BATTERY_INFO: MessageType = MessageType { number: 375, name: "Device AUX battery info" };
    pub const MFG_RANGE_MIN: MessageType = MessageType { number: 0xFF00, name: "MFG range min" };
    pub const MFG_RANGE_MAX: MessageType = MessageType { number: 0xFFFE, name: "MFG range max" };

    const ALL_TYPES: [MessageType; 91] = [
        MessageType::FILE_ID,
        MessageType::CAPABILITIES,
        MessageType::DEVICE_SETTINGS,
        MessageType::USER_PROFILE,
        MessageType::HRM_PROFILE,
        MessageType::SDM_PROFILE,
        MessageType::BIKE_PROFILE,
        MessageType::ZONES_TARGET,
        MessageType::HR_ZONE,
        MessageType::POWER_ZONE,
        MessageType::MET_ZONE,
        MessageType::SPORT,
        MessageType::GOAL,
        MessageType::SESSION,
        MessageType::LAP,
        MessageType::RECORD,
        MessageType::EVENT,
        MessageType::DEVICE_INFO,
        MessageType::WORKOUT,
        MessageType::WORKOUT_STEP,
        MessageType::SCHEDULE,
        MessageType::WEIGHT_SCALE,
        MessageType::COURSE,
        MessageType::COURSE_POINT,
        MessageType::TOTALS,
        MessageType::ACTIVITY,
        MessageType::SOFTWARE,
        MessageType::FILE_CAPABILITIES,
        MessageType::MESG_CAPABILITIES,
        MessageType::FIELD_CAPABILITIES,
        MessageType::FILE_CREATOR,
        MessageType::BLOOD_PRESSURE,
        MessageType::SPEED_ZONE,
        MessageType::MONITORING,
        MessageType::TRAINING_FILE,
        MessageType::HRV,
        MessageType::ANT_RX,
        MessageType::ANT_TX,
        MessageType::ANT_CHANNEL_ID,
        MessageType::LENGTH,
        MessageType::MONITORING_INFO,
        MessageType::PAD,
        MessageType::SLAVE_DEVICE,
        MessageType::CONNECTIVITY,
        MessageType::WEATHER_CONDITIONS,
        MessageType::WEATHER_ALERT,
        MessageType::CADENCE_ZONE,
        MessageType::HR,
        MessageType::SEGMENT_LAP,
        MessageType::MEMO_GLOB,
        MessageType::SEGMENT_ID,
        MessageType::SEGMENT_LEADERBOARD_ENTRY,
        MessageType::SEGMENT_POINT,
        MessageType::SEGMENT_FILE,
        MessageType::WORKOUT_SESSION,
        MessageType::WATCHFACE_SETTINGS,
        MessageType::GPS_METADATA,
        MessageType::CAMERA_EVENT,
        MessageType::TIMESTAMP_CORRELATION,
        MessageType::GYROSCOPE_DATA,
        MessageType::ACCELEROMETER_DATA,
        MessageType::THREE_D_SENSOR_CALIBRATION,
        MessageType::VIDEO_FRAME,
        MessageType::OBDII_DATA,
        MessageType::NMEA_SENTENCE,
        MessageType::AVIATION_ATTITUDE,
        MessageType::VIDEO,
        MessageType::VIDEO_TITLE,
        MessageType::VIDEO_DESCRIPTION,
        MessageType::VIDEO_CLIP,
        MessageType::OHR_SETTINGS,
        MessageType::EXD_SCREEN_CONFIGURATION,
        MessageType::EXD_DATA_FIELD_CONFIGURATION,
        MessageType::EXD_DATA_CONCEPT_CONFIGURATION,
        MessageType::FIELD_DESCRIPTION,
        MessageType::DEVELOPER_DATA_ID,
        MessageType::MAGNETOMETER_DATA,
        MessageType::BAROMETER_DATA,
        MessageType::ONE_D_SENSOR_CALIBRATION,
        MessageType::SET,
        MessageType::STRESS_LEVEL,
        MessageType::DIVE_SETTINGS,
        MessageType::DIVE_GAS,
        MessageType::DIVE_ALARM,
        MessageType::EXERCISE_TITLE,
        MessageType::DIVE_SUMMARY,
        MessageType::JUMP,
        MessageType::CLIMB_PRO,
        MessageType::DEVICE_AUX_BATTERY_INFO,
        MessageType::MFG_RANGE_MIN,
        MessageType::MFG_RANGE_MAX,
    ];
    pub(crate) fn parse(i: u16) -> MessageType {
        for message_type in Self::ALL_TYPES {
            if message_type.number == i {
                return message_type;
            }
        }

        MessageType::UNKNOWN
    }
}
