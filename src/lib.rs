#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals)]
extern crate libc;

// pub use libc::c_void;
// pub use libc::c_long;
// pub use libc::c_int;
// pub use libc::size_t;

use std::sync::{Once, ONCE_INIT};

pub const MAX_OID_LEN:       libc::size_t = 128;

pub const STAT_SUCCESS:      libc::c_int = 0;
pub const STAT_ERROR:        libc::c_int = 1;
pub const STAT_TIMEOUT:      libc::c_int = 2;

pub const SNMP_VERSION_1:    libc::c_long = 0;
pub const SNMP_VERSION_2c:   libc::c_long = 1;
pub const SNMP_VERSION_3:    libc::c_long = 3;

pub const SNMP_MSG_GET:      libc::c_int = 160;
pub const SNMP_MSG_GETNEXT:  libc::c_int = 161;
pub const SNMP_MSG_RESPONSE: libc::c_int = 162;
pub const SNMP_MSG_SET:      libc::c_int = 163;
pub const SNMP_MSG_TRAP:     libc::c_int = 164;
pub const SNMP_MSG_GETBULK:  libc::c_int = 165;
pub const SNMP_MSG_INFORM:   libc::c_int = 166;
pub const SNMP_MSG_TRAP2:    libc::c_int = 167;
pub const SNMP_MSG_REPORT:   libc::c_int = 168;

pub const SNMP_NOSUCHOBJECT:     libc::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x0);
pub const SNMP_NOSUCHINSTANCE:   libc::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x1);
pub const SNMP_ENDOFMIBVIEW:     libc::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x2);

pub const SNMP_ERR_NOERROR:             libc::c_long =  0;
pub const SNMP_ERR_TOOBIG:	            libc::c_long =  1;
pub const SNMP_ERR_NOSUCHNAME:          libc::c_long =  2;
pub const SNMP_ERR_BADVALUE:            libc::c_long =  3;
pub const SNMP_ERR_READONLY:            libc::c_long =  4;
pub const SNMP_ERR_GENERR:	            libc::c_long =  5;
pub const SNMP_ERR_NOACCESS:	          libc::c_long =  6;
pub const SNMP_ERR_WRONGTYPE:	          libc::c_long =  7;
pub const SNMP_ERR_WRONGLENGTH:		      libc::c_long =  8;
pub const SNMP_ERR_WRONGENCODING:	      libc::c_long =  9;
pub const SNMP_ERR_WRONGVALUE:		      libc::c_long = 10;
pub const SNMP_ERR_NOCREATION:		      libc::c_long = 11;
pub const SNMP_ERR_INCONSISTENTVALUE:	  libc::c_long = 12;
pub const SNMP_ERR_RESOURCEUNAVAILABLE: libc::c_long = 13;
pub const SNMP_ERR_COMMITFAILED:        libc::c_long = 14;
pub const SNMP_ERR_UNDOFAILED:	        libc::c_long = 15;
pub const SNMP_ERR_AUTHORIZATIONERROR:  libc::c_long = 16;
pub const SNMP_ERR_NOTWRITABLE:         libc::c_long = 17;
pub const SNMP_ERR_INCONSISTENTNAME:    libc::c_long = 18;

pub const SNMPERR_SUCCESS:                      libc::c_int =   0;     /* XXX  Non-PDU "success" code. */
pub const SNMPERR_GENERR:                       libc::c_int =  -1;
pub const SNMPERR_BAD_LOCPORT:                  libc::c_int =  -2;
pub const SNMPERR_BAD_ADDRESS:                  libc::c_int =  -3;
pub const SNMPERR_BAD_SESSION:                  libc::c_int =  -4;
pub const SNMPERR_TOO_LONG:                     libc::c_int =  -5;
pub const SNMPERR_NO_SOCKET:                    libc::c_int =  -6;
pub const SNMPERR_V2_IN_V1:                     libc::c_int =  -7;
pub const SNMPERR_V1_IN_V2:                     libc::c_int =  -8;
pub const SNMPERR_BAD_REPEATERS:                libc::c_int =  -9;
pub const SNMPERR_BAD_REPETITIONS:              libc::c_int = -10;
pub const SNMPERR_BAD_ASN1_BUILD:               libc::c_int = -11;
pub const SNMPERR_BAD_SENDTO:                   libc::c_int = -12;
pub const SNMPERR_BAD_PARSE:                    libc::c_int = -13;
pub const SNMPERR_BAD_VERSION:                  libc::c_int = -14;
pub const SNMPERR_BAD_SRC_PARTY:                libc::c_int = -15;
pub const SNMPERR_BAD_DST_PARTY:                libc::c_int = -16;
pub const SNMPERR_BAD_CONTEXT:                  libc::c_int = -17;
pub const SNMPERR_BAD_COMMUNITY:                libc::c_int = -18;
pub const SNMPERR_NOAUTH_DESPRIV:               libc::c_int = -19;
pub const SNMPERR_BAD_ACL:                      libc::c_int = -20;
pub const SNMPERR_BAD_PARTY:                    libc::c_int = -21;
pub const SNMPERR_ABORT:                        libc::c_int = -22;
pub const SNMPERR_UNKNOWN_PDU:                  libc::c_int = -23;
pub const SNMPERR_TIMEOUT:                      libc::c_int = -24;
pub const SNMPERR_BAD_RECVFROM:                 libc::c_int = -25;
pub const SNMPERR_BAD_ENG_ID:                   libc::c_int = -26;
pub const SNMPERR_BAD_SEC_NAME:                 libc::c_int = -27;
pub const SNMPERR_BAD_SEC_LEVEL:                libc::c_int = -28;
pub const SNMPERR_ASN_PARSE_ERR:                libc::c_int = -29;
pub const SNMPERR_UNKNOWN_SEC_MODEL:            libc::c_int = -30;
pub const SNMPERR_INVALID_MSG:                  libc::c_int = -31;
pub const SNMPERR_UNKNOWN_ENG_ID:               libc::c_int = -32;
pub const SNMPERR_UNKNOWN_USER_NAME:            libc::c_int = -33;
pub const SNMPERR_UNSUPPORTED_SEC_LEVEL:        libc::c_int = -34;
pub const SNMPERR_AUTHENTICATION_FAILURE:       libc::c_int = -35;
pub const SNMPERR_NOT_IN_TIME_WINDOW:           libc::c_int = -36;
pub const SNMPERR_DECRYPTION_ERR:               libc::c_int = -37;
pub const SNMPERR_SC_GENERAL_FAILURE:           libc::c_int = -38;
pub const SNMPERR_SC_NOT_CONFIGURED:            libc::c_int = -39;
pub const SNMPERR_KT_NOT_AVAILABLE:             libc::c_int = -40;
pub const SNMPERR_UNKNOWN_REPORT:               libc::c_int = -41;
pub const SNMPERR_USM_GENERICERROR:             libc::c_int = -42;
pub const SNMPERR_USM_UNKNOWNSECURITYNAME:      libc::c_int = -43;
pub const SNMPERR_USM_UNSUPPORTEDSECURITYLEVEL: libc::c_int = -44;
pub const SNMPERR_USM_ENCRYPTIONERROR:          libc::c_int = -45;
pub const SNMPERR_USM_AUTHENTICATIONFAILURE:    libc::c_int = -46;
pub const SNMPERR_USM_PARSEERROR:               libc::c_int = -47;
pub const SNMPERR_USM_UNKNOWNENGINEID:          libc::c_int = -48;
pub const SNMPERR_USM_NOTINTIMEWINDOW:          libc::c_int = -49;
pub const SNMPERR_USM_DECRYPTIONERROR:          libc::c_int = -50;
pub const SNMPERR_NOMIB:                        libc::c_int = -51;
pub const SNMPERR_RANGE:                        libc::c_int = -52;
pub const SNMPERR_MAX_SUBID:                    libc::c_int = -53;
pub const SNMPERR_BAD_SUBID:                    libc::c_int = -54;
pub const SNMPERR_LONG_OID:                     libc::c_int = -55;
pub const SNMPERR_BAD_NAME:                     libc::c_int = -56;
pub const SNMPERR_VALUE:                        libc::c_int = -57;
pub const SNMPERR_UNKNOWN_OBJID:                libc::c_int = -58;
pub const SNMPERR_NULL_PDU:                     libc::c_int = -59;
pub const SNMPERR_NO_VARS:                      libc::c_int = -60;
pub const SNMPERR_VAR_TYPE:                     libc::c_int = -61;
pub const SNMPERR_MALLOC:                       libc::c_int = -62;
pub const SNMPERR_KRB5:                         libc::c_int = -63;
pub const SNMPERR_PROTOCOL:                     libc::c_int = -64;
pub const SNMPERR_OID_NONINCREASING:            libc::c_int = -65;
pub const SNMPERR_JUST_A_CONTEXT_PROBE:         libc::c_int = -66;
pub const SNMPERR_TRANSPORT_NO_CONFIG:          libc::c_int = -67;
pub const SNMPERR_TRANSPORT_CONFIG_ERROR:       libc::c_int = -68;
pub const SNMPERR_TLS_NO_CERTIFICATE:           libc::c_int = -69;
pub const SNMPERR_MAX:                          libc::c_int = -69;

pub const ASN_BOOLEAN:	    libc::c_uchar = 0x01;
pub const ASN_INTEGER:	    libc::c_uchar = 0x02;
pub const ASN_BIT_STR:	    libc::c_uchar = 0x03;
pub const ASN_OCTET_STR:    libc::c_uchar = 0x04;
pub const ASN_NULL:	        libc::c_uchar = 0x05;
pub const ASN_OBJECT_ID:	  libc::c_uchar = 0x06;
pub const ASN_SEQUENCE:	    libc::c_uchar = 0x10;
pub const ASN_SET:		      libc::c_uchar = 0x11;
pub const ASN_UNIVERSAL:	  libc::c_uchar = 0x00;
pub const ASN_APPLICATION:  libc::c_uchar = 0x40;
pub const ASN_CONTEXT:	    libc::c_uchar = 0x80;
pub const ASN_PRIVATE:	    libc::c_uchar = 0xC0;
pub const ASN_PRIMITIVE:	  libc::c_uchar = 0x00;
pub const ASN_CONSTRUCTOR:	libc::c_uchar = 0x20;

pub const ASN_IPADDRESS:    libc::c_uchar = (ASN_APPLICATION |  0);
pub const ASN_COUNTER:      libc::c_uchar = (ASN_APPLICATION |  1);
pub const ASN_GAUGE:        libc::c_uchar = (ASN_APPLICATION |  2);
pub const ASN_UNSIGNED:     libc::c_uchar = (ASN_APPLICATION |  2); // RFC 1902 - same as GAUGE
pub const ASN_TIMETICKS:    libc::c_uchar = (ASN_APPLICATION |  3);
pub const ASN_OPAQUE:       libc::c_uchar = (ASN_APPLICATION |  4); // changed so no conflict with other includes
pub const ASN_NSAP:         libc::c_uchar = (ASN_APPLICATION |  5); // historic - don't use
pub const ASN_COUNTER64:    libc::c_uchar = (ASN_APPLICATION |  6);
pub const ASN_UINTEGER:     libc::c_uchar = (ASN_APPLICATION |  7); // historic - don't use
pub const ASN_FLOAT:        libc::c_uchar = (ASN_APPLICATION |  8);
pub const ASN_DOUBLE:       libc::c_uchar = (ASN_APPLICATION |  9);
pub const ASN_INTEGER64:    libc::c_uchar = (ASN_APPLICATION | 10);
pub const ASN_UNSIGNED64:   libc::c_uchar = (ASN_APPLICATION | 11);

mod auto;
pub use auto::*;

static INIT: Once = ONCE_INIT;

pub fn init(typ: &[u8]) {
    let mut type_dup = [0u8; 256];
    type_dup[..typ.len()].clone_from_slice(typ);
    unsafe {
        INIT.call_once(||{
            init_snmp(type_dup.as_ptr() as *mut libc::c_char);
            netsnmp_init_mib();
        })
    }
}

#[cfg(test)]
mod tests;
