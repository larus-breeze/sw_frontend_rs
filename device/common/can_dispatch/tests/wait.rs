mod queues;
use queues::*;

use can_dispatch::*;

const TEST_DATA: [&str; 8] = [
    "result Some(585000), frame Some(CanFrame { id: 15, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(635000), frame Some(CanFrame { id: 14, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(685000), frame Some(CanFrame { id: 13, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(735000), frame Some(CanFrame { id: 12, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(785000), frame Some(CanFrame { id: 11, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(835000), frame Some(CanFrame { id: 10, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(885000), frame Some(CanFrame { id: 9, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
    "result Some(935000), frame Some(CanFrame { id: 9, rtr: true, len: 0, data: [0, 0, 0, 0, 0, 0, 0, 0] })",
];

fn rng() -> u32 {
    0x1abc_dd28
}

#[test]
fn wait() {
    let mut ticks: u64 = 0;
    #[allow(unused)]
    let (
        mut p_tx_frames,
        mut c_tx_frames,
        mut p_view_tx_frames,
        mut c_view_tx_frames,
        mut p_view_rx_frames,
        mut c_view_rx_frames,
    ) = get_the_queues();

    let mut dis =
        CanDispatch::<32, 8, 10, 30>::new(rng, p_tx_frames, p_view_rx_frames, c_view_tx_frames);

    // Startup and negotiating the basic_id
    for expected in TEST_DATA {
        // An other guy is blocking the process, so we can only reach level 9
        let other_guys_frame = CanFrame::remote_trans_rq(7, 0);
        dis.rx_data(other_guys_frame);

        let nt = dis.tick(ticks);
        let result = format!("result {:?}, frame {:?}", nt, c_tx_frames.dequeue());
        assert_eq!(&result, expected);
        if nt.is_none() {
            break;
        }
        ticks = nt.unwrap();
    }

    //println!("    \"result {:?}, frame {:?}\",", nt, c_tx_f.dequeue());
}
