mod queues;
use queues::*;

use corelib::*;

const TEST_DATA: [&str; 14] = [
    "result Some(550000), frame Some(can_id 0xf data 0x00000000_00000000])",
    "result Some(600500), frame Some(can_id 0xe data 0x00000000_00000000])",
    "result Some(651000), frame Some(can_id 0xd data 0x00000000_00000000])",
    "result Some(701500), frame Some(can_id 0xc data 0x00000000_00000000])",
    "result Some(752000), frame Some(can_id 0xb data 0x00000000_00000000])",
    "result Some(802500), frame Some(can_id 0xa data 0x00000000_00000000])",
    "result Some(853000), frame Some(can_id 0x9 data 0x00000000_00000000])",
    "result Some(903500), frame Some(can_id 0x8 data 0x00000000_00000000])",
    "result Some(954000), frame Some(can_id 0x7 data 0x00000000_00000000])",
    "result Some(1004500), frame Some(can_id 0x6 data 0x00000000_00000000])",
    "result Some(1055000), frame Some(can_id 0x5 data 0x00000000_00000000])",
    "result Some(1105500), frame Some(can_id 0x4 data 0x00000000_00000000])",
    "result Some(1156000), frame Some(can_id 0x3 data 0x00000000_00000000])",
    "result Some(1206500), frame Some(can_id 0x2 data 0x00000000_00000000])",
];

#[test]
fn vda_blocked() {
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
    for expected in TEST_DATA {
        let nt = dis.tick(ticks);
        let result = format!("result {:?}, frame {:?}", nt, c_tx_irq_frames.dequeue());
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
    let result = format!("result {:?}, frame {:?}", nt, c_tx_irq_frames.dequeue());

    // We use the vda 0x33 and send rtr, as 0x32 is occupied
    assert_eq!(&result, "result None, frame Some(can_id 0x610 data 0x00000000_00000000])");

    // Now create first heartbeat in normal mode, emulate application
    let frame = GenericFrame {
        generic_id: 0,
        can_frame: CanFrame::empty_from_id(0),
    };
    p_tx_frames.enqueue(Frame::Generic(frame)).unwrap();

    // Dispatch the frame
    let nt = dis.tick(ticks);
    let result = format!("result {:?}, frame {:?}", nt, c_tx_irq_frames.dequeue());

    // This is the first real heartbeat
    assert_eq!(&result, "result None, frame Some(can_id 0x610 data 0x00000000_00000000])");

    //println!("    \"result {:?}, frame {:?}\",", nt, c_tx_f.dequeue());
}
