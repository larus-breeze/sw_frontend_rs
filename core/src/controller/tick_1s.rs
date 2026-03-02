use crate::{
    controller::persist::{persist_set, set_vario_mode},
    model::{GpsState, SystemState, TcrMode, VarioModeControl},
    utils::Variant,
    CoreController, CoreModel, Echo, FloatToLength, FloatToSpeed, FlyMode, IdleEvent,
    PersistenceId, VarioMode,
};
use embedded_graphics::geometry::AngleUnit;
use num::clamp;

pub fn recalc_polar(cm: &mut CoreModel, cc: &mut CoreController) {
    cc.polar.recalc(&cm.glider_data, cm.sensor.density);

    let _ = cc.scheduler.chain(speed_to_fly);
}

fn speed_to_fly(cm: &mut CoreModel, cc: &mut CoreController) {
    let stf = cc.polar.speed_to_fly(0.0.m_s(), 0.0.m_s());
    cm.control.speed_to_fly_limit = stf.ias() * cm.control.vario_mode_switch_ratio;

    // In auto mode switch between Vario and SpeedToFly
    if cm.sensor.airspeed.ias() > cm.control.speed_to_fly_limit
        && cm.control.fly_mode == FlyMode::StraightFlight
    {
        set_vario_mode(cm, cc, VarioMode::SpeedToFly, VarioModeControl::Auto);
    } else {
        set_vario_mode(cm, cc, VarioMode::Vario, VarioModeControl::Auto);
    }

    // in pin mode set according to pin state
    set_vario_mode(
        cm,
        cc,
        cc.speed_to_fly_control.vario_mode(),
        VarioModeControl::InputPin,
    );

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

    // decide circling or straight flight, 5°/sec --> circling
    let mut hyst = if cm.sensor.turn_rate.to_rad_s().abs() > 0.1 {
        cm.calculated.circle_hysteresis + 1
    } else {
        cm.calculated.circle_hysteresis - 1
    };
    hyst = clamp(hyst, 0, cm.config.circle_hysteresis_tc);

    if hyst == cm.config.circle_hysteresis_tc {
        cm.control.fly_mode = FlyMode::Circling;
    }
    if hyst == 0 {
        cm.control.fly_mode = FlyMode::StraightFlight;
    }
    cm.calculated.circle_hysteresis = hyst;

    let is_circling = cm.control.fly_mode == FlyMode::Circling;

    // Circle diameter: D = 2 * v / omega (only meaningful in circling)
    let omega = cm.sensor.turn_rate.to_rad_s().abs();
    let tas_mps = cm.sensor.airspeed.tas().to_m_s();
    if is_circling && omega > 0.02 && tas_mps > 5.0 {
        let d = (2.0 * tas_mps / omega).clamp(5.0, 2000.0);
        cm.calculated.circle_diameter = d.m();
        cm.calculated.circle_diameter_valid = true;
    } else {
        cm.calculated.circle_diameter_valid = false;
    }

    // Circle max-min over 24 heading bins. Show "--" until first full circle is complete.
    let climb_delta = (cm.sensor.climb_rate - cm.calculated.av2_climb_rate).to_m_s();
    let yaw = cm.sensor.euler_yaw.to_radians();
    if let Some(delta) = cc.circle_stats.update(yaw, climb_delta, is_circling) {
        cm.calculated.circle_max_min_last = delta.m_s();
        cm.calculated.circle_max_min_valid = true;
    } else {
        cm.calculated.circle_max_min_valid = false;
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
        persist_set(
            cc,
            cm,
            Variant::Mass(cm.glider_data.water_ballast),
            PersistenceId::WaterBallast,
            Echo::NmeaAndCan,
        )
    }
    let _ = cc.gear_alarm_control.alarm_is_active(cm);

    if let Some(state) = cc.flash_control.tick_1s(cm) {
        let _ = cc.queue_to_idle_task.enqueue(IdleEvent::Output1(state));
    }
    let _ = cc.scheduler.chain(sync_config_items);
}

fn sync_config_items(cm: &mut CoreModel, cc: &mut CoreController) {
    // Only if frontend is can bus master
    if cm.control.is_can_master {
        // every 3 seconds please
        if (cm.control.alive_ticks / 10).is_multiple_of(3) {
            let bugs = Variant::F32(cm.glider_data.bugs());
            persist_set(cc, cm, bugs, PersistenceId::Bugs, Echo::NmeaAndCan);
            let mc = Variant::F32(cm.config.mc_cready.to_m_s());
            persist_set(cc, cm, mc, PersistenceId::McCready, Echo::NmeaAndCan);
            let ballast = Variant::F32(cm.glider_data.water_ballast.to_kg());
            persist_set(
                cc,
                cm,
                ballast,
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            );
            let ballast = Variant::F32(cm.glider_data.water_ballast.to_kg());
            persist_set(
                cc,
                cm,
                ballast,
                PersistenceId::WaterBallast,
                Echo::NmeaAndCan,
            );
            let vario_mode = Variant::U8(cm.control.vario_mode as u8);
            persist_set(
                cc,
                cm,
                vario_mode,
                PersistenceId::VarioMode,
                Echo::NmeaAndCan,
            );
        }
    }
}
