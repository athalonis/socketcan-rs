use CanSocket;

#[test]
fn test_nonexistant_device() {
    assert!(CanSocket::open("invalid").is_err());
}
#[cfg(feature = "vcan_tp_tests")]
mod vcan_tp_tests {
    extern crate futures;
    extern crate tokio_core;
    extern crate tokio_io;

    use futures::stream::Stream;
    use self::tokio_core::reactor::Core;
    use {TpListener, CanInterface, CanTpSocket, ERR_MASK_ALL,
         ERR_MASK_NONE};
    use std::time;
    use ShouldRetry;

    #[test]
    fn vcan0_cantp_loopback(){

        let len : usize = 1978;

        let tp = CanTpSocket::open_nb("vcan0", 0x4de, 0x4c8).unwrap();

        let tp2 = CanTpSocket::open_nb("vcan0", 0x4c8, 0x4de).unwrap();
        let mut test_msg : Vec<u8> = Vec::new();
        for i in 0..len { test_msg.push((0xff & i) as u8) }

        tp2.write_frame(test_msg.clone());
        let msg = tp.read_msg().unwrap();

        let err = 0;
        for i in 0..len {
            print!("{:X} ", msg[i as usize]);
            assert!(msg[i as usize] == test_msg[i as usize])
        }
        println!("");

        println!("size {:?}", msg.len());
        assert!(len as usize == msg.len());
    }

    #[test]
    fn vcan0_cantp_socket_nonblocking() {
        let cs = CanTpSocket::open_nb("vcan0", 0x4de, 0x4c8).unwrap();
        cs.set_nonblocking(true);

        // no timeout set, but should return immediately
        assert!(cs.read_msg().should_retry());
    }

    //#[test]
    //fn vcan0_cantp_non_blocking() {
    //    let mut core = Core::new().unwrap();
    //    let cbs = CanTpSocket::open_nb("vcan0", 0x4de, 0x4c8).unwrap();
    //    let ival = time::Duration::from_millis(1);


    //    let cl = TpListener::from(cbs, &core.handle()).unwrap();
    //    println!("Listener created");
    //    let received = 0;
    //    let msg_stream = cl.for_each(|msg| {
    //        print!("MSG HEAD {:X} len: {:?}", msg[0], msg.len());
    //        Ok(())
    //    });

    //    let tp2 = CanTpSocket::open_nb("vcan0", 0x4c8, 0x4de).unwrap();
    //    let mut test_msg : Vec<u8> = Vec::new();
    //    for i in 0..4095 { test_msg.push((0xff & i) as u8) }
    //    tp2.write_frame(test_msg.clone());

    //    core.run(msg_stream).unwrap();

    //    println!("Done Done");
    //}


}



//#[cfg(feature = "vcan_tests")]
//mod vcan_tests {
//    extern crate futures;
//    extern crate tokio_core;
//    extern crate tokio_io;
//
//    use futures::stream::Stream;
//    use self::tokio_core::reactor::Core;
//    use {BcmListener, CanFrame, CanInterface, CanSocket, CanBCMSocket, ERR_MASK_ALL,
//         ERR_MASK_NONE};
//    use std::time;
//    use ShouldRetry;
//
//    #[test]
//    fn vcan0_timeout() {
//        let cs = CanSocket::open("vcan0").unwrap();
//        cs.set_read_timeout(time::Duration::from_millis(100))
//            .unwrap();
//        assert!(cs.read_frame().should_retry());
//    }
//
//    //#[test]
//    //fn vcan0_set_error_mask() {
//    //    let cs = CanSocket::open("vcan0").unwrap();
//    //    cs.set_error_mask(ERR_MASK_ALL).unwrap();
//    //    cs.set_error_mask(ERR_MASK_NONE).unwrap();
//    //}
//
//    #[test]
//    fn vcan0_enable_own_loopback() {
//        let cs = CanSocket::open("vcan0").unwrap();
//        cs.set_loopback(true).unwrap();
//        cs.set_recv_own_msgs(true).unwrap();
//
//        let frame = CanFrame::new(0x123, &[], true, false).unwrap();
//
//        cs.write_frame(&frame).unwrap();
//
//        cs.read_frame().unwrap();
//    }
//
//    #[test]
//    fn vcan0_set_down() {
//        let can_if = CanInterface::open("vcan0").unwrap();
//        can_if.bring_down().unwrap();
//    }
//
//    #[test]
//    fn vcan0_test_nonblocking() {
//        let cs = CanSocket::open("vcan0").unwrap();
//        cs.set_nonblocking(true);
//
//        // no timeout set, but should return immediately
//        assert!(cs.read_frame().should_retry());
//    }
//
//    #[test]
//    fn vcan0_bcm_filter() {
//        let cbs = CanBCMSocket::open_nb("vcan0").unwrap();
//        let ival = time::Duration::from_millis(1);
//        cbs.filter_id(0x123, ival, ival).unwrap();
//
//        let cs = CanSocket::open("vcan0").unwrap();
//        let frame = CanFrame::new(0x123, &[], true, false).unwrap();
//        cs.write_frame(&frame).unwrap();
//
//        // TODO this currently blocks the tests and requires a manual
//        // cansend vcan0 123#1122334455667788
//        let msghead = cbs.read_msg().unwrap();
//        assert!(msghead.frames()[0].id() == 0x123);
//    }
//
//    #[test]
//    fn vcan0_bcm_filter_delete() {
//        let cbs = CanBCMSocket::open_nb("vcan0").unwrap();
//        let ival = time::Duration::from_millis(1);
//        cbs.filter_id(0x123, ival, ival).unwrap();
//
//        cbs.filter_delete(0x123).unwrap();
//    }
//
//    #[test]
//    fn vcan0_bcm_filter_delete_err() {
//        let cbs = CanBCMSocket::open_nb("vcan0").unwrap();
//        assert!(cbs.filter_delete(0x124).is_err())
//    }
//
//    #[test]
//    fn vcan0_bcm_non_blocking() {
//        let mut core = Core::new().unwrap();
//        let cbs = CanBCMSocket::open_nb("vcan0").unwrap();
//        let ival = time::Duration::from_millis(1);
//        cbs.filter_id(0x123, ival, ival).unwrap();
//
//        let cl = BcmListener::from(cbs, &core.handle()).unwrap();
//        let msg_stream = cl.for_each(|msg_head| {
//            print!("MSG HEAD {:?}", msg_head._can_id);
//            Ok(())
//        });
//        core.run(msg_stream).unwrap();
//        println!("Done Done");
//    }
//}
