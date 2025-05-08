use crate::{
    model::{GpsState, SystemState, TcrMode, VarioModeControl},
    controller::persist,
    utils::Variant,
    CoreController, CoreModel, Echo, FloatToSpeed, FlyMode, IdleEvent, PersistenceId, VarioMode,
};

pub fn recalc_polar(cm: &mut CoreModel, cc: &mut CoreController) {
    cc.polar.recalc(&cm.glider_data, cm.sensor.density);

    let _ = cc.scheduler.chain(speed_to_fly);
}

fn speed_to_fly(cm: &mut CoreModel, cc: &mut CoreController) {
    let stf = cc.polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
    cm.control.speed_to_fly_limit = stf.ias() * cm.control.vario_mode_switch_ratio;

    // save vario mode to record changes
    let vario_mode = cm.control.vario_mode;

    // In auto mode switch between Vario and SpeedToFly
    if cm.sensor.airspeed.ias() > cm.control.speed_to_fly_limit {
        cm.set_vario_mode(VarioMode::SpeedToFly, VarioModeControl::Auto);
    } else {
        cm.set_vario_mode(VarioMode::Vario, VarioModeControl::Auto);
    }

    // in pin mode set according to pin state
    cm.set_vario_mode(cc.speed_to_fly_control.vario_mode(), VarioModeControl::InputPin);

    // changes? then push to nmea output
    if vario_mode != cm.control.vario_mode {
        let _ = cc
            .nmea_buffer
            .pers_id
            .push_front(PersistenceId::VarioModeControl);
    }

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
    let _ = cc.scheduler.chain(process_hardware_pins);
}

fn process_hardware_pins(cm: &mut CoreModel, cc: &mut CoreController) {
    // check water ballast system
    cc.drain_control.tick_1s(cm);
    if cc.drain_control.is_flowing() {
        persist::persist_set(
            cc,
            cm,
            Variant::Mass(cm.glider_data.water_ballast),
            PersistenceId::WaterBallast,
            Echo::NmeaAndCan,
        )
    }

    if let Some(state) = cc.flash_control.tick_1s(cm) {
        let _ = cc.p_idle_events.enqueue(IdleEvent::Output1(state));
    }
}
