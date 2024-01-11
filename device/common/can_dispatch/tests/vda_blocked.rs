mod queues;
use queues::*;

use can_dispatch::*;

const TEST_DATA: [&str; 14] = [
    "result Some(550000), frame Some(CanFrame { id: 15, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(600500), frame Some(CanFrame { id: 14, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(651000), frame Some(CanFrame { id: 13, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(701500), frame Some(CanFrame { id: 12, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(752000), frame Some(CanFrame { id: 11, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(802500), frame Some(CanFrame { id: 10, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(853000), frame Some(CanFrame { id: 9, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(903500), frame Some(CanFrame { id: 8, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(954000), frame Some(CanFrame { id: 7, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(1004500), frame Some(CanFrame { id: 6, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(1055000), frame Some(CanFrame { id: 5, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(1105500), frame Some(CanFrame { id: 4, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(1156000), frame Some(CanFrame { id: 3, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(1206500), frame Some(CanFrame { id: 2, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
//    "result None, frame Some(CanFrame { id: 1536, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",        // hex(1536) -> RTR on 0x600, heartbeat(vda=32)
];

#[test]
fn vda_blocked() {
    let mut ticks: u64 = 0;
    #[allow(unused)]
    let (mut p_tx_frames, mut c_tx_frames, mut p_rx_frames, mut c_rx_frames) =
        get_the_queues();

    let mut dis = CanDispatch::<32, 8, 10, 30, Rng>::new(Rng{}, p_rx_frames, c_tx_frames);

    // Startup and negotiating the basic_id
    for expected in TEST_DATA {
        let nt = dis.tick(ticks);
        let result = format!("result {:?}, frame {:?}", nt, dis.tx_data());
        //println!("    \"{}\"", result);
        assert_eq!(&result, expected);
        if nt.is_none() {
            break;
        }
        ticks = nt.unwrap();
    }

    // An other guy sends a heartbeat on our vda
    let other_guys_frame = CanFrame::empty_from_id(0x600);
    dis.rx_data(other_guys_frame);
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, dis.tx_data());

    // We use the vda 0x33 and send rtr, as 0x32 is occupied
    assert_eq!(&result, "result None, frame Some(CanFrame { id: 1552, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })");

    // Now create first heartbeat in normal mode, emulate application
    let frame = GenericFrame {
        generic_id: 0,
        can_frame: CanFrame::empty_from_id(0),
    };
    p_tx_frames.enqueue(Frame::Generic(frame)).unwrap();

    // Dispatch the frame
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, dis.tx_data());

    // This is the first real heartbeat
    assert_eq!(&result, "result None, frame Some(CanFrame { id: 1552, rtr: false, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })");

    //println!("    \"result {:?}, frame {:?}\",", nt, c_tx_f.dequeue());
}
