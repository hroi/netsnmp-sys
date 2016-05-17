#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals)]
extern crate libc;

use std::os::raw;
use std::sync::{Once, ONCE_INIT};

pub const MAX_OID_LEN:       libc::size_t = 128;

pub const STAT_SUCCESS:      raw::c_int = 0;
pub const STAT_ERROR:        raw::c_int = 1;
pub const STAT_TIMEOUT:      raw::c_int = 2;

pub const SNMP_VERSION_1:    raw::c_long = 0;
pub const SNMP_VERSION_2c:   raw::c_long = 1;
pub const SNMP_VERSION_3:    raw::c_long = 3;

pub const SNMP_MSG_GET:      raw::c_int = 160;
pub const SNMP_MSG_GETNEXT:  raw::c_int = 161;
pub const SNMP_MSG_RESPONSE: raw::c_int = 162;
pub const SNMP_MSG_SET:      raw::c_int = 163;
pub const SNMP_MSG_TRAP:     raw::c_int = 164;
pub const SNMP_MSG_GETBULK:  raw::c_int = 165;
pub const SNMP_MSG_INFORM:   raw::c_int = 166;
pub const SNMP_MSG_TRAP2:    raw::c_int = 167;
pub const SNMP_MSG_REPORT:   raw::c_int = 168;

pub const SNMP_NOSUCHOBJECT:     raw::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x0);
pub const SNMP_NOSUCHINSTANCE:   raw::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x1);
pub const SNMP_ENDOFMIBVIEW:     raw::c_uchar = (ASN_CONTEXT | ASN_PRIMITIVE | 0x2);

pub const SNMP_ERR_NOERROR:             raw::c_long =  0;
pub const SNMP_ERR_TOOBIG:	            raw::c_long =  1;
pub const SNMP_ERR_NOSUCHNAME:          raw::c_long =  2;
pub const SNMP_ERR_BADVALUE:            raw::c_long =  3;
pub const SNMP_ERR_READONLY:            raw::c_long =  4;
pub const SNMP_ERR_GENERR:	            raw::c_long =  5;
pub const SNMP_ERR_NOACCESS:	          raw::c_long =  6;
pub const SNMP_ERR_WRONGTYPE:	          raw::c_long =  7;
pub const SNMP_ERR_WRONGLENGTH:		      raw::c_long =  8;
pub const SNMP_ERR_WRONGENCODING:	      raw::c_long =  9;
pub const SNMP_ERR_WRONGVALUE:		      raw::c_long = 10;
pub const SNMP_ERR_NOCREATION:		      raw::c_long = 11;
pub const SNMP_ERR_INCONSISTENTVALUE:	  raw::c_long = 12;
pub const SNMP_ERR_RESOURCEUNAVAILABLE: raw::c_long = 13;
pub const SNMP_ERR_COMMITFAILED:        raw::c_long = 14;
pub const SNMP_ERR_UNDOFAILED:	        raw::c_long = 15;
pub const SNMP_ERR_AUTHORIZATIONERROR:  raw::c_long = 16;
pub const SNMP_ERR_NOTWRITABLE:         raw::c_long = 17;
pub const SNMP_ERR_INCONSISTENTNAME:    raw::c_long = 18;

pub const SNMPERR_SUCCESS:                      raw::c_int =   0;     /* XXX  Non-PDU "success" code. */
pub const SNMPERR_GENERR:                       raw::c_int =  -1;
pub const SNMPERR_BAD_LOCPORT:                  raw::c_int =  -2;
pub const SNMPERR_BAD_ADDRESS:                  raw::c_int =  -3;
pub const SNMPERR_BAD_SESSION:                  raw::c_int =  -4;
pub const SNMPERR_TOO_LONG:                     raw::c_int =  -5;
pub const SNMPERR_NO_SOCKET:                    raw::c_int =  -6;
pub const SNMPERR_V2_IN_V1:                     raw::c_int =  -7;
pub const SNMPERR_V1_IN_V2:                     raw::c_int =  -8;
pub const SNMPERR_BAD_REPEATERS:                raw::c_int =  -9;
pub const SNMPERR_BAD_REPETITIONS:              raw::c_int = -10;
pub const SNMPERR_BAD_ASN1_BUILD:               raw::c_int = -11;
pub const SNMPERR_BAD_SENDTO:                   raw::c_int = -12;
pub const SNMPERR_BAD_PARSE:                    raw::c_int = -13;
pub const SNMPERR_BAD_VERSION:                  raw::c_int = -14;
pub const SNMPERR_BAD_SRC_PARTY:                raw::c_int = -15;
pub const SNMPERR_BAD_DST_PARTY:                raw::c_int = -16;
pub const SNMPERR_BAD_CONTEXT:                  raw::c_int = -17;
pub const SNMPERR_BAD_COMMUNITY:                raw::c_int = -18;
pub const SNMPERR_NOAUTH_DESPRIV:               raw::c_int = -19;
pub const SNMPERR_BAD_ACL:                      raw::c_int = -20;
pub const SNMPERR_BAD_PARTY:                    raw::c_int = -21;
pub const SNMPERR_ABORT:                        raw::c_int = -22;
pub const SNMPERR_UNKNOWN_PDU:                  raw::c_int = -23;
pub const SNMPERR_TIMEOUT:                      raw::c_int = -24;
pub const SNMPERR_BAD_RECVFROM:                 raw::c_int = -25;
pub const SNMPERR_BAD_ENG_ID:                   raw::c_int = -26;
pub const SNMPERR_BAD_SEC_NAME:                 raw::c_int = -27;
pub const SNMPERR_BAD_SEC_LEVEL:                raw::c_int = -28;
pub const SNMPERR_ASN_PARSE_ERR:                raw::c_int = -29;
pub const SNMPERR_UNKNOWN_SEC_MODEL:            raw::c_int = -30;
pub const SNMPERR_INVALID_MSG:                  raw::c_int = -31;
pub const SNMPERR_UNKNOWN_ENG_ID:               raw::c_int = -32;
pub const SNMPERR_UNKNOWN_USER_NAME:            raw::c_int = -33;
pub const SNMPERR_UNSUPPORTED_SEC_LEVEL:        raw::c_int = -34;
pub const SNMPERR_AUTHENTICATION_FAILURE:       raw::c_int = -35;
pub const SNMPERR_NOT_IN_TIME_WINDOW:           raw::c_int = -36;
pub const SNMPERR_DECRYPTION_ERR:               raw::c_int = -37;
pub const SNMPERR_SC_GENERAL_FAILURE:           raw::c_int = -38;
pub const SNMPERR_SC_NOT_CONFIGURED:            raw::c_int = -39;
pub const SNMPERR_KT_NOT_AVAILABLE:             raw::c_int = -40;
pub const SNMPERR_UNKNOWN_REPORT:               raw::c_int = -41;
pub const SNMPERR_USM_GENERICERROR:             raw::c_int = -42;
pub const SNMPERR_USM_UNKNOWNSECURITYNAME:      raw::c_int = -43;
pub const SNMPERR_USM_UNSUPPORTEDSECURITYLEVEL: raw::c_int = -44;
pub const SNMPERR_USM_ENCRYPTIONERROR:          raw::c_int = -45;
pub const SNMPERR_USM_AUTHENTICATIONFAILURE:    raw::c_int = -46;
pub const SNMPERR_USM_PARSEERROR:               raw::c_int = -47;
pub const SNMPERR_USM_UNKNOWNENGINEID:          raw::c_int = -48;
pub const SNMPERR_USM_NOTINTIMEWINDOW:          raw::c_int = -49;
pub const SNMPERR_USM_DECRYPTIONERROR:          raw::c_int = -50;
pub const SNMPERR_NOMIB:                        raw::c_int = -51;
pub const SNMPERR_RANGE:                        raw::c_int = -52;
pub const SNMPERR_MAX_SUBID:                    raw::c_int = -53;
pub const SNMPERR_BAD_SUBID:                    raw::c_int = -54;
pub const SNMPERR_LONG_OID:                     raw::c_int = -55;
pub const SNMPERR_BAD_NAME:                     raw::c_int = -56;
pub const SNMPERR_VALUE:                        raw::c_int = -57;
pub const SNMPERR_UNKNOWN_OBJID:                raw::c_int = -58;
pub const SNMPERR_NULL_PDU:                     raw::c_int = -59;
pub const SNMPERR_NO_VARS:                      raw::c_int = -60;
pub const SNMPERR_VAR_TYPE:                     raw::c_int = -61;
pub const SNMPERR_MALLOC:                       raw::c_int = -62;
pub const SNMPERR_KRB5:                         raw::c_int = -63;
pub const SNMPERR_PROTOCOL:                     raw::c_int = -64;
pub const SNMPERR_OID_NONINCREASING:            raw::c_int = -65;
pub const SNMPERR_JUST_A_CONTEXT_PROBE:         raw::c_int = -66;
pub const SNMPERR_TRANSPORT_NO_CONFIG:          raw::c_int = -67;
pub const SNMPERR_TRANSPORT_CONFIG_ERROR:       raw::c_int = -68;
pub const SNMPERR_TLS_NO_CERTIFICATE:           raw::c_int = -69;
pub const SNMPERR_MAX:                          raw::c_int = -69;

pub const ASN_BOOLEAN:	    raw::c_uchar = 0x01;
pub const ASN_INTEGER:	    raw::c_uchar = 0x02;
pub const ASN_BIT_STR:	    raw::c_uchar = 0x03;
pub const ASN_OCTET_STR:    raw::c_uchar = 0x04;
pub const ASN_NULL:	        raw::c_uchar = 0x05;
pub const ASN_OBJECT_ID:	  raw::c_uchar = 0x06;
pub const ASN_SEQUENCE:	    raw::c_uchar = 0x10;
pub const ASN_SET:		      raw::c_uchar = 0x11;
pub const ASN_UNIVERSAL:	  raw::c_uchar = 0x00;
pub const ASN_APPLICATION:  raw::c_uchar = 0x40;
pub const ASN_CONTEXT:	    raw::c_uchar = 0x80;
pub const ASN_PRIVATE:	    raw::c_uchar = 0xC0;
pub const ASN_PRIMITIVE:	  raw::c_uchar = 0x00;
pub const ASN_CONSTRUCTOR:	raw::c_uchar = 0x20;

pub const ASN_IPADDRESS:    raw::c_uchar = (ASN_APPLICATION |  0);
pub const ASN_COUNTER:      raw::c_uchar = (ASN_APPLICATION |  1);
pub const ASN_GAUGE:        raw::c_uchar = (ASN_APPLICATION |  2);
pub const ASN_UNSIGNED:     raw::c_uchar = (ASN_APPLICATION |  2); // RFC 1902 - same as GAUGE
pub const ASN_TIMETICKS:    raw::c_uchar = (ASN_APPLICATION |  3);
pub const ASN_OPAQUE:       raw::c_uchar = (ASN_APPLICATION |  4); // changed so no conflict with other includes
pub const ASN_NSAP:         raw::c_uchar = (ASN_APPLICATION |  5); // historic - don't use
pub const ASN_COUNTER64:    raw::c_uchar = (ASN_APPLICATION |  6);
pub const ASN_UINTEGER:     raw::c_uchar = (ASN_APPLICATION |  7); // historic - don't use
pub const ASN_FLOAT:        raw::c_uchar = (ASN_APPLICATION |  8);
pub const ASN_DOUBLE:       raw::c_uchar = (ASN_APPLICATION |  9);
pub const ASN_INTEGER64:    raw::c_uchar = (ASN_APPLICATION | 10);
pub const ASN_UNSIGNED64:   raw::c_uchar = (ASN_APPLICATION | 11);

mod auto;
pub use auto::*;

static INIT: Once = ONCE_INIT;

pub fn init(typ: &[u8]) {
    let mut type_dup = [0u8; 256];
    type_dup[..typ.len()].clone_from_slice(typ);
    unsafe {
        INIT.call_once(||{
            init_snmp(type_dup.as_ptr() as *mut raw::c_char);
            netsnmp_init_mib();
        })
    }
}

#[cfg(test)]
mod tests;
