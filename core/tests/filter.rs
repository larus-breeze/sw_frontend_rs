mod queues;
use queues::*;

use corelib::*;

#[test]
fn filter() {
    let mut ticks: u64 = 0;
    #[allow(unused)]
    let (
        mut p_tx_irq_frames,
        mut c_tx_irq_frames,
        mut p_tx_frames,
        mut c_tx_frames,
        mut p_rx_frames,
        mut c_rx_frames,
    ) = get_the_queues();

    let mut dis =
        CanDispatch::<32, 8, 10, 30, Rng>::new(Rng {}, p_tx_irq_frames, p_rx_frames, c_tx_frames);

    // Startup and negotiating the basic_id
    for _ in 1..15 {
        let nt = dis.tick(ticks);
        let _ = c_tx_irq_frames.dequeue(); // clear the queue
        if nt.is_none() {
            break;
        }
        ticks = nt.unwrap();
    }

    // ***** ==> We do not pay attention to the time stamps because they are irrelevant for the filter functions

    // We are now in normal mode and try to receive a legacy datagram
    // other guy
    let other_frame = CanFrame::empty_from_id(0x102); // Airspeed from sensorbox
    dis.rx_data(other_frame);
    let nt = dis.tick(ticks);

    // ther is nothing to decode, because we did not set filters
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    assert_eq!(result, "result None, frame None");

    // Now we set the legacy filter
    dis.set_legacy_filter(0x100, 0x120).unwrap();
    dis.rx_data(other_frame);
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // aha, now we get the data
    assert_eq!(result, "result None, frame Some(Legacy(CanFrame { id: 258, rtr: false, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] }))");

    // Let's try some generic data
    let other_frame = CanFrame::empty_from_id(0x641); // Some generic data
    dis.rx_data(other_frame);
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // Generic messages are is always passed through. It does not matter, where they come from
    assert_eq!(result, "result None, frame Some(Generic(GenericFrame { can_frame: CanFrame { id: 1601, rtr: false, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] }, generic_id: 1 }))");

    // Let's try some special data
    let other_frame = CanFrame::empty_from_id(0x242); // Some special data
    dis.rx_data(other_frame);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // Special data must be booked via the object id
    assert_eq!(result, "result None, frame None");

    // Let's set the filter for special data,
    dis.set_object_id_filter(17).unwrap();
    // We need a heartbeat for this device, to get data passed through
    let data = [17, 00, 00, 00, 01, 02, 03, 04]; // object_id: 1
    let heartbeat = CanFrame::from_slice(0x640, &data[0..8]);
    dis.rx_data(heartbeat);
    ticks += 1_000_000; // Clean up can_device list with sec tick
    let nt = dis.tick(ticks);

    // Again: Let's try some special data
    let other_frame = CanFrame::empty_from_id(0x242); // Some special data
    dis.rx_data(other_frame);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // Now, special data is passed through
    assert_eq!(result, "result None, frame Some(Specific(SpecificFrame { can_frame: CanFrame { id: 578, rtr: false, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] }, specific_id: 2, object_id: 17 }))");

    //println!("'{}'", result);
}
