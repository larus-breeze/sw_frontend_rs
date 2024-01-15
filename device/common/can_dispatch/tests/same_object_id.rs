mod queues;
use queues::*;

use can_dispatch::*;

#[test]
fn same_object_id() {
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

    // Let's set the filter for special data,
    dis.set_object_id_filter(17).unwrap();

    // We now emulate two devices with the same object_id
    let data = [17, 00, 00, 00, 01, 02, 03, 04]; // object_id: 1
    let heartbeat = CanFrame::from_slice(0x640, &data[0..8]);
    dis.rx_data(heartbeat);
    let heartbeat2 = CanFrame::from_slice(0x650, &data[0..8]);
    dis.rx_data(heartbeat2);

    // Let's activate the 1 Hz routine, which cleans up the can_device list
    ticks += 1_000_000;
    dis.tick(ticks);

    // The first device is passed through the filter
    let other_frame = CanFrame::empty_from_id(0x242); // Some special data
    dis.rx_data(other_frame);
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // Now, special data is passed through
    assert_eq!(result, "result None, frame Some(Specific(SpecificFrame { can_frame: CanFrame { id: 578, rtr: false, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] }, specific_id: 2, object_id: 17 }))");

    // The second device is not passed through, even though it has the same object_id. This is
    // desirable as identical information should only be passed through to the application once.
    let other_frame = CanFrame::empty_from_id(0x252); // Some special data
    dis.rx_data(other_frame);
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, c_rx_frames.dequeue());
    // No data is passed through
    assert_eq!(result, "result None, frame None");

    //println!("'{}'", result);
}
