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
            /*
            match value {
                Value::NumberValueVecU8(my_value) => {
                    //if !data_field.name.eq("Unknown") {
                    if field_definition.base_type.name == "enum" && !my_value.is_empty() {
                        let enum_value = Value::StringValue((data_field.translate_enum)(&data_field, my_value.get(0).unwrap()));
                        println!("\t{} (type: {} / number: {}) with value {:?}", data_field.name, field_definition.base_type.name, field_definition.number, enum_value);
                    } else {
                        println!("\t{} (type: {} / number: {}) with value {:?}", data_field.name, field_definition.base_type.name, field_definition.number, my_value);
                    }
                    //}
                }
                Value::Invalid => {
                    if !data_field.name.eq("Unknown") {
                        //println!("\tIgnoring invalid value for field {} / {}", data_field.name, field_definition.number);
                    }
                }
                // _ => if !data_field.name.eq("Unknown") {println!("\t{} (type: {} / number: {}) with value {:?}", data_field.name, field_definition.base_type.name, field_definition.number, value)}
                _ => println!("\t{} (type: {} / number: {}) with value {:?}", data_field.name, field_definition.base_type.name, field_definition.number, value)
            }
             */
        }
        (Message { message_type: self.message_type.name.to_string(), data: data_map }, position)
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
}

impl MessageType {
    pub fn resolve(i: u16) -> MessageType {
        match i {
            0 => MessageType {
                number: 0,
                name: "File Id",
            },
            1 => MessageType {
                number: 1,
                name: "Capabilities",
            },
            2 => MessageType {
                number: 2,
                name: "Device settings",
            },
            3 => MessageType {
                number: 3,
                name: "User profile",
            },
            4 => MessageType {
                number: 4,
                name: "HRM profile",
            },
            5 => MessageType {
                number: 5,
                name: "SDM profile",
            },
            6 => MessageType {
                number: 6,
                name: "Bike profile",
            },
            7 => MessageType {
                number: 7,
                name: "Zones target",
            },
            8 => MessageType {
                number: 8,
                name: "HR zone",
            },
            9 => MessageType {
                number: 9,
                name: "Power zone",
            },
            10 => MessageType {
                number: 10,
                name: "MET zone",
            },
            12 => MessageType {
                number: 12,
                name: "Sport",
            },
            15 => MessageType {
                number: 15,
                name: "Goal",
            },
            18 => MessageType {
                number: 18,
                name: "Session",
            },
            19 => MessageType {
                number: 19,
                name: "Lap",
            },
            20 => MessageType {
                number: 20,
                name: "Record",
            },
            21 => MessageType {
                number: 21,
                name: "Event",
            },
            23 => MessageType {
                number: 23,
                name: "Device info",
            },
            26 => MessageType {
                number: 26,
                name: "Workout",
            },
            27 => MessageType {
                number: 27,
                name: "Workout step",
            },
            28 => MessageType {
                number: 28,
                name: "Schedule",
            },
            30 => MessageType {
                number: 30,
                name: "Weight scale",
            },
            31 => MessageType {
                number: 31,
                name: "Course",
            },
            32 => MessageType {
                number: 32,
                name: "Course point",
            },
            33 => MessageType {
                number: 33,
                name: "Totals",
            },
            34 => MessageType {
                number: 34,
                name: "Activity",
            },
            35 => MessageType {
                number: 35,
                name: "Software",
            },
            37 => MessageType {
                number: 37,
                name: "File capabilities",
            },
            38 => MessageType {
                number: 38,
                name: "Message capabilities",
            },
            39 => MessageType {
                number: 39,
                name: "Field capabilities",
            },
            49 => MessageType {
                number: 49,
                name: "File creator",
            },
            51 => MessageType {
                number: 51,
                name: "Blood pressure",
            },
            53 => MessageType {
                number: 53,
                name: "Speed zone",
            },
            55 => MessageType {
                number: 55,
                name: "Monitoring",
            },
            72 => MessageType {
                number: 72,
                name: "Training file",
            },
            78 => MessageType {
                number: 78,
                name: "HRV",
            },
            80 => MessageType {
                number: 80,
                name: "ANT rx",
            },
            81 => MessageType {
                number: 81,
                name: "ANT tx",
            },
            82 => MessageType {
                number: 82,
                name: "ANT channel id",
            },
            101 => MessageType {
                number: 101,
                name: "Length",
            },
            103 => MessageType {
                number: 103,
                name: "Monitoring info",
            },
            105 => MessageType {
                number: 105,
                name: "Pad",
            },
            106 => MessageType {
                number: 106,
                name: "Slave device",
            },
            127 => MessageType {
                number: 127,
                name: "Connectivity",
            },
            128 => MessageType {
                number: 128,
                name: "Weather conditions",
            },
            129 => MessageType {
                number: 129,
                name: "Weather alert",
            },
            131 => MessageType {
                number: 131,
                name: "Cadence zone",
            },
            132 => MessageType {
                number: 132,
                name: "HR",
            },
            142 => MessageType {
                number: 142,
                name: "Segment lap",
            },
            145 => MessageType {
                number: 145,
                name: "Memo glob",
            },
            148 => MessageType {
                number: 148,
                name: "Segment id",
            },
            149 => MessageType {
                number: 149,
                name: "Segment leaderboard entry",
            },
            150 => MessageType {
                number: 150,
                name: "Segment point",
            },
            151 => MessageType {
                number: 151,
                name: "Segment file",
            },
            158 => MessageType {
                number: 158,
                name: "Workout session",
            },
            159 => MessageType {
                number: 159,
                name: "Watchface settings",
            },
            160 => MessageType {
                number: 160,
                name: "GPS Metadata",
            },
            161 => MessageType {
                number: 161,
                name: "Camera event",
            },
            162 => MessageType {
                number: 162,
                name: "Timestamp correlation",
            },
            164 => MessageType {
                number: 164,
                name: "Gyroscope data",
            },
            165 => MessageType {
                number: 165,
                name: "Accelerometer data",
            },
            167 => MessageType {
                number: 167,
                name: "3D sensor calibration",
            },
            169 => MessageType {
                number: 169,
                name: "Video frame",
            },
            174 => MessageType {
                number: 174,
                name: "OBD II data",
            },
            177 => MessageType {
                number: 177,
                name: "NMEA sentence",
            },
            178 => MessageType {
                number: 178,
                name: "Aviation attitude",
            },
            184 => MessageType {
                number: 184,
                name: "Video",
            },
            185 => MessageType {
                number: 185,
                name: "Video title",
            },
            186 => MessageType {
                number: 186,
                name: "Video description",
            },
            187 => MessageType {
                number: 187,
                name: "Video clip",
            },
            188 => MessageType {
                number: 188,
                name: "OHR settings",
            },
            200 => MessageType {
                number: 200,
                name: "EXD screen configuration",
            },
            201 => MessageType {
                number: 201,
                name: "EXD data field configuration",
            },
            202 => MessageType {
                number: 202,
                name: "EXD data concept configuration",
            },
            206 => MessageType {
                number: 206,
                name: "Field description",
            },
            207 => MessageType {
                number: 207,
                name: "Developer data id",
            },
            208 => MessageType {
                number: 208,
                name: "Magnetometer data",
            },
            209 => MessageType {
                number: 209,
                name: "Barometer data",
            },
            210 => MessageType {
                number: 210,
                name: "1D sensor calibration",
            },
            216 => MessageType {
                number: 216,
                name: "Time in zone",
            },
            225 => MessageType {
                number: 225,
                name: "Set",
            },
            227 => MessageType {
                number: 227,
                name: "Stress level",
            },
            258 => MessageType {
                number: 258,
                name: "Dive settings",
            },
            259 => MessageType {
                number: 259,
                name: "Dive gas",
            },
            262 => MessageType {
                number: 262,
                name: "Dive alarm",
            },
            264 => MessageType {
                number: 264,
                name: "Exercise title",
            },
            268 => MessageType {
                number: 268,
                name: "Dive summary",
            },
            285 => MessageType {
                number: 285,
                name: "Jump",
            },
            312 => MessageType {
                number: 312,
                name: "Split",
            },
            313 => MessageType {
                number: 313,
                name: "Split summary",
            },
            317 => MessageType {
                number: 317,
                name: "Climb pro",
            },
            375 => MessageType {
                number: 375,
                name: "Device AUX battery info",
            },
            0xFF00 => MessageType {
                number: 0xFF00,
                name: "MFG range min",
            },
            0xFFFE => MessageType {
                number: 0xFFFE,
                name: "MFG range max",
            },

            147 => MessageType {
                number: 147,
                name: "Audio (undocumented)",
            },
            1024 | _ => MessageType {
                number: 1024,
                name: "Unknown",
            },
        }
    }
}
