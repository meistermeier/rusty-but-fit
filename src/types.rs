use std::fmt;
use std::fmt::{Display, Formatter};

crate::key_value_enum! {
    pub enum FileType {
           Device = 1 ,
           Settings = 2 ,
           Sport = 3 ,
           Activity = 4 ,
           Workout = 5 ,
           Course = 6 ,
           Schedules = 7 ,
           Weight = 9 ,
           Totals = 10 ,
           Goals = 11 ,
           BloodPressure = 14 ,
           MonitoringA = 15 ,
           ActivitySummary = 20 ,
           MonitoringDaily = 28 ,
           MonitoringB = 32 ,
           Segment = 34 ,
           SegmentList = 35 ,
           ExdConfiguration = 40 ,
           MfgRangeMin = 0xF7 ,
           MfgRangeMax = 0xFE ,

    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::key_value_enum! {
    pub enum TimeMode {
        Hour12=0,
        Hour24 = 1,
        Military = 2,
        Hour12WithSeconds = 3,
        Hour24WithSeconds = 4,
        Utc = 5,
    }
}

impl Display for TimeMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::key_value_enum! {
    pub enum BacklightMode {
        Off = 0,
        Manual = 1,
        KeyAndMessages = 2,
        AutoBrightness = 3,
        SmartNotifications = 4,
        KeyAndMessagesNight = 5,
        KeyAndMessagesAndSmartNotifications = 6,
    }
}

impl Display for BacklightMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::key_value_enum! {
    pub enum EventType {
        start	= 0,
        stop	= 1,
        consecutive_depreciated	= 2,
        marker	= 3,
        stop_all	= 4,
        begin_depreciated	= 5,
        end_depreciated	= 6,
        end_all_depreciated	= 7,
        stop_disable	= 8,
        stop_disable_all	= 9,
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::key_value_enum! {
    pub enum Event {
        timer	=0,
        workout	=3,
        workout_step	=4,
        power_down	=5,
        power_up	=6,
        off_course	=7,
        session	=8,
        lap	=9,
        course_point	=10,
        battery	=11,
        virtual_partner_pace	=12,
        hr_high_alert	=13,
        hr_low_alert	=14,
        speed_high_alert	=15,
        speed_low_alert	=16,
        cad_high_alert	=17,
        cad_low_alert	=18,
        power_high_alert	=19,
        power_low_alert	=20,
        recovery_hr	=21,
        battery_low	=22,
        time_duration_alert	=23,
        distance_duration_alert	=24,
        calorie_duration_alert	=25,
        activity	=26,
        fitness_equipment	=27,
        length	=28,
        user_marker	=32,
        sport_point	=33,
        calibration	=36,
        front_gear_change	=42,
        rear_gear_change	=43,
        rider_position_change	=44,
        elev_high_alert	=45,
        elev_low_alert	=46,
        comm_timeout	=47,
        auto_activity_detect	=54,
        dive_alert	=56,
        dive_gas_switched	=57,
        tank_pressure_reserve	=71,
        tank_pressure_critical	=72,
        tank_lost	=73,
        radar_threat_alert	=75,
        tank_battery_low	=76,
        tank_pod_connected	=81,
        tank_pod_disconnected	=82,

    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

crate::key_value_enum! {
    pub enum Sport {
            Generic = 0,
            Running = 1,
            Cycling = 2,
            Transistion = 3,
            FitnessEquipment = 4,
            Swimming = 5,
            Basketball = 6,
            Soccer = 7,
            Tennis = 8,
            AmericanFootball = 9,
            Training = 10,
            Walking = 11,
            CrossCountrySkiing = 12,
            AlpineSkiing = 13,
            Snowboarding = 14,
            Rowing = 15,
            Mountaineering = 16,
            Hiking = 17,
            Multisport = 18,
            Paddling = 19,
            Flying = 20,
            EBiking = 21,
            Motorcycling = 22,
            Boating = 23,
            Driving = 24,
            Golf = 25,
            HangGliding = 26,
            HorsebackRiding = 27,
            Hunting = 28,
            Fishing = 29,
            InlineSkating = 30,
            RockClimbing = 31,
            Sailing = 32,
            IceSkating = 33,
            SkyDiving = 34,
            Snowshoeing = 35,
            Snowmobiling = 36,
            StandUpPaddleboarding = 37,
            Surfing = 38,
            Wakeboarding = 39,
            WaterSkiing = 40,
            Kayaking = 41,
            Rafting = 42,
            Windsurfing = 43,
            Kitesurfing = 44,
            Tactical = 45,
            Jumpmaster = 46,
            Boxing = 47,
            FloorClimbing = 48,
            Baseball = 49,
            Diving = 53,
            Hiit = 62,
            Racket = 64,
            WheelchairPushWalk = 65,
            WheelchairPushRun = 66,
            Meditation = 67,
            DiscGolf = 69,
            Cricket = 71,
            Rugby = 72,
            Hockey = 73,
            Lacrosse = 74,
            Volleyball = 75,
            WaterTubing = 76,
            Wakesurfing = 77,
            MixedMartialArts = 80,
            Snorkeling = 82,
            Dance = 83,
            JumpRope = 84,
            All = 254,
    }
}

impl Display for Sport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
