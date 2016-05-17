use libc;
// use std::mem;
// use std::ptr;
use std::ffi::CString;
use super::*;


#[test]
fn test_read_objid() {
    init(b"test");
    unsafe {
        let input = CString::new("1.3.6").unwrap();
        let mut objid: [oid; 256] = [0; 256];
        let mut objid_size: libc::size_t = objid.len() as libc::size_t;

        let res = read_objid(input.as_ptr(), &mut objid[0], &mut objid_size);

        let expected: &[oid] = &[1,3,6];
        assert_eq!(res, 1);
        assert_eq!(expected, &objid[..objid_size]);
    }
}

// #[test]
// fn test_snmp_session() {
//     init(b"test");
//     let peer_name = CString::new("edgy.asdf.dk").unwrap().into_raw();
//     let peer_community = b"st0vsuger";
//     let peer_obj = CString::new("SNMPv2-MIB::sysDescr.0").unwrap().into_raw();

//     unsafe {
//         let mut sess: Struct_snmp_session = mem::uninitialized();
//         let pdu: *mut Struct_snmp_pdu;
//         let mut response_ptr: *mut Struct_snmp_pdu = mem::uninitialized();

//         snmp_sess_init(&mut sess);
//         sess.peername = peer_name;
//         sess.community = &mut peer_community.clone()[0];
//         sess.community_len = peer_community.len();

//         sess.version = SNMP_VERSION_2c;

//         let ss = snmp_open(&mut sess);

//         pdu = snmp_pdu_create(SNMP_MSG_GET);

//         let mut anOID: [oid; MAX_OID_LEN] = [0; MAX_OID_LEN];
//         let mut anOID_len = MAX_OID_LEN;
//         if snmp_parse_oid(peer_obj, &mut anOID[0], &mut anOID_len) == ptr::null_mut() {
//             panic!();
//         }

//         snmp_add_null_var(pdu, &mut anOID[0], anOID_len);

//         let status = snmp_synch_response(ss, pdu, &mut response_ptr);
//         println!("snmp_synch_response() -> {}", status);

//         if status == STAT_SUCCESS {
//             let response = *response_ptr;
//             if response.errstat == SNMP_ERR_NOERROR {
//                 let mut vars = response.variables;
//                 while vars != ptr::null_mut() {
//                     print_variable((*vars).name, (*vars).name_length, vars);
//                     vars = (*vars).next_variable;
//                 }
//             }
//         } else {
//             panic!("status={}", status);
//         }
//     }

// }
