use crate::{
    model::{GpsState, SystemState, TcrMode, VarioModeControl},
    CoreController, CoreModel, FloatToSpeed, FlyMode, IdleEvent, VarioMode,
};

pub fn recalc_polar(cm: &mut CoreModel, cc: &mut CoreController) {
    cc.polar.recalc(&cm.glider_data, cm.sensor.density);

    let _ = cc.scheduler.chain(speed_to_fly);
}

fn speed_to_fly(cm: &mut CoreModel, cc: &mut CoreController) {
    let stf = cc.polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
    cm.control.speed_to_fly_limit = stf.ias() * cm.control.vario_mode_switch_ratio;

    // In auto mode switch between Vario and SpeedToFly
    cm.control.vario_mode = match cm.control.vario_mode_control {
        VarioModeControl::Auto => {
            if cm.sensor.airspeed.ias() > cm.control.speed_to_fly_limit {
                VarioMode::SpeedToFly
            } else {
                VarioMode::Vario
            }
        }
        VarioModeControl::SpeedToFly => VarioMode::SpeedToFly,
        VarioModeControl::Vario => VarioMode::Vario,
    };

    // Set 1-second-speed-to-fly value
    cm.calculated.speed_to_fly_1s = cm.calculated.av_speed_to_fly;

    if cc.last_vario_mode != cm.control.vario_mode {
        cc.last_vario_mode = cm.control.vario_mode;
        if cm.control.vario_mode == VarioMode::Vario {
            // Set average climbrate to current climbrate
            cc.av2_climb_rate.set_value(cm.sensor.climb_rate);
        }
    }
    match cm.control.fly_mode {
        FlyMode::Circling => {
            // Start measuring thermal climb rate
            match cm.control.tcr_mode {
                TcrMode::StraightFlight => {
                    cm.control.tcr_start = cm.sensor.gps_altitude;
                    cm.control.tcr_1s_climb_ticks = 1;
                }
                TcrMode::Transition => {
                    cm.control.tcr_1s_transient_ticks = 0;
                    cm.control.tcr_1s_climb_ticks += 1;
                }
                TcrMode::Climbing => {
                    cm.control.tcr_1s_climb_ticks += 1;
                }
            }
            cm.control.tcr_mode = TcrMode::Climbing;
            // Calculate thermal climb rate if not in slave mode
            if cm.control.avg_climb_slave_ticks > 0 {
                cm.control.avg_climb_slave_ticks -= 1;
            } else {
                let tcr = {
                    let diff_h = (cm.sensor.gps_altitude - cm.control.tcr_start).to_m();
                    (diff_h / cm.control.tcr_1s_climb_ticks as f32).m_s()
                };
                cm.calculated.thermal_climb_rate = tcr;
            }
        }
        FlyMode::StraightFlight => match cm.control.tcr_mode {
            TcrMode::Climbing => {
                cm.control.tcr_mode = TcrMode::Transition;
                cm.control.tcr_1s_transient_ticks = 0;
            }
            TcrMode::Transition => {
                cm.control.tcr_1s_transient_ticks += 1;
                if cm.control.tcr_1s_transient_ticks > 30 {
                    cm.control.tcr_mode = TcrMode::StraightFlight;
                    cm.calculated.thermal_climb_rate = 0.0.m_s();
                }
            }
            TcrMode::StraightFlight => cm.control.tcr_start = cm.sensor.gps_altitude,
        },
    }

    let _ = cc.scheduler.chain(can_heartbeat);
}

fn can_heartbeat(cm: &mut CoreModel, cc: &mut CoreController) {
    // create CAN heartbeat frame and add to queue
    let can_frame = cm.can_frame_heartbeat();
    let _ = cc.p_tx_frames.enqueue(can_frame);

    // check, if other can devices are visible
    cm.control.system_state = if cm.control.can_devices != 0 {
        match cm.sensor.gps_state {
            GpsState::PosAvail | GpsState::HeadingAvail => SystemState::CanAndGpsOk,
            _ => SystemState::CanOk,
        }
    } else {
        cm.sensor.gps_state = GpsState::NoGps;
        SystemState::NoCom
    };
    cm.control.can_devices = 0;

    let _ = cc.scheduler.chain(set_date_time);
}

fn set_date_time(cm: &mut CoreModel, cc: &mut CoreController) {
    // Sets time and date for the log of a crash
    let event = IdleEvent::DateTime(cm.sensor.gps_date_time);
    cc.send_idle_event(event);

    let _ = cc.scheduler.chain(send_can_nmea);
}

fn send_can_nmea(cm: &mut CoreModel, cc: &mut CoreController) {
    // send some datagrams every second
    let can_frame = cm.can_frame_volt_temp();
    let _ = cc.p_tx_frames.enqueue(can_frame);

    cc.nmea_cyclic_1s();
}
