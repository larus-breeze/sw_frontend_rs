mod queues;
use queues::*;

use can_dispatch::*;

const TEST_DATA: [&str; 8] = [
    "result Some(550000), frame Some(CanFrame { id: 15, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(600500), frame Some(CanFrame { id: 14, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(651000), frame Some(CanFrame { id: 13, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(701500), frame Some(CanFrame { id: 12, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(752000), frame Some(CanFrame { id: 11, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(802500), frame Some(CanFrame { id: 10, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(853000), frame Some(CanFrame { id: 9, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(903500), frame Some(CanFrame { id: 9, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
];

#[test]
fn wait() {
    let mut ticks: u64 = 0;
    #[allow(unused)]
    let (mut p_tx_frames, mut c_tx_frames, mut p_rx_frames, mut c_rx_frames) =
        get_the_queues();

    let mut dis = CanDispatch::<32, 8, 10, 30, Rng>::new(Rng{}, p_rx_frames, c_tx_frames);

    // Startup and negotiating the basic_id
    for expected in TEST_DATA {
        // An other guy is blocking the process, so we can only reach level 9
        let other_guys_frame = CanFrame::remote_trans_rq(7, 0);
        dis.rx_data(other_guys_frame);

        let nt = dis.tick(ticks);
        let result = format!("result {:?}, frame {:?}", nt, dis.tx_data());
        //println!("    \"{}\",", result);
        assert_eq!(&result, expected);
        if nt.is_none() {
            break;
        }
        ticks = nt.unwrap();
    }

    //println!("    \"result {:?}, frame {:?}\",", nt, c_tx_f.dequeue());
}
