use libc;
use std::os::raw;
use auto::*;

pub type marker_t = *mut raw::c_void;
pub type const_marker_t = *const raw::c_void;

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}


#[repr(C)]
#[derive(Copy)]
pub struct div_t {
    pub quot:  raw::c_int,
    pub rem:  raw::c_int,
}
impl Clone for div_t {
    fn clone(&self) -> Self { *self }
}

pub type Netsnmp_Free_List_Data =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut  raw::c_void)>;
pub type Netsnmp_Save_List_Data =
    ::std::option::Option<unsafe extern "C" fn(buf:
                                                   *mut  raw::c_char,
                                               buf_len: usize,
                                               arg1:
                                                   *mut  raw::c_void)
                              ->  raw::c_int>;
pub type Netsnmp_Read_List_Data =
    ::std::option::Option<unsafe extern "C" fn(buf:
                                                   *mut  raw::c_char,
                                               buf_len: usize)
                              -> *mut  raw::c_void>;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_data_list_s {
    pub next: *mut netsnmp_data_list_s,
    pub name: *mut  raw::c_char,
    pub data: *mut  raw::c_void,
    pub free_func: Netsnmp_Free_List_Data,
}
impl Clone for netsnmp_data_list_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_data_list = netsnmp_data_list_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_data_list_saveinfo_s {
    pub datalist: *mut *mut netsnmp_data_list,
    pub type_: *const  raw::c_char,
    pub token: *const  raw::c_char,
    pub data_list_save_ptr: Netsnmp_Save_List_Data,
    pub data_list_read_ptr: Netsnmp_Read_List_Data,
    pub data_list_free_ptr: Netsnmp_Free_List_Data,
}

impl Clone for netsnmp_data_list_saveinfo_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_data_list_saveinfo = netsnmp_data_list_saveinfo_s;
extern "C" {
    pub fn netsnmp_create_data_list(arg1: *const  raw::c_char,
                                    arg2: *mut  raw::c_void,
                                    arg3: Netsnmp_Free_List_Data)
     -> *mut netsnmp_data_list;

    pub fn netsnmp_data_list_add_node(head: *mut *mut netsnmp_data_list,
                                      node: *mut netsnmp_data_list);

    pub fn netsnmp_data_list_add_data(head: *mut *mut netsnmp_data_list,
                                      name: *const  raw::c_char,
                                      data: *mut  raw::c_void,
                                      beer: Netsnmp_Free_List_Data)
     -> *mut netsnmp_data_list;

    pub fn netsnmp_get_list_data(head: *mut netsnmp_data_list,
                                 node: *const  raw::c_char)
     -> *mut  raw::c_void;

    pub fn netsnmp_free_list_data(head: *mut netsnmp_data_list);

    pub fn netsnmp_free_all_list_data(head: *mut netsnmp_data_list);

    pub fn netsnmp_remove_list_node(realhead: *mut *mut netsnmp_data_list,
                                    name: *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_get_list_node(head: *mut netsnmp_data_list,
                                 name: *const  raw::c_char)
     -> *mut netsnmp_data_list;

    pub fn netsnmp_add_list_data(head: *mut *mut netsnmp_data_list,
                                 node: *mut netsnmp_data_list);

    pub fn netsnmp_register_save_list(datalist: *mut *mut netsnmp_data_list,
                                      type_: *const  raw::c_char,
                                      token: *const  raw::c_char,
                                      data_list_save_ptr:
                                          Netsnmp_Save_List_Data,
                                      data_list_read_ptr:
                                          Netsnmp_Read_List_Data,
                                      data_list_free_ptr:
                                          Netsnmp_Free_List_Data);

    pub fn netsnmp_save_all_data(head: *mut netsnmp_data_list,
                                 type_: *const  raw::c_char,
                                 token: *const  raw::c_char,
                                 data_list_save_ptr: Netsnmp_Save_List_Data)
     ->  raw::c_int;

    pub fn netsnmp_save_all_data_callback(majorID:  raw::c_int,
                                          minorID:  raw::c_int,
                                          serverarg:
                                              *mut  raw::c_void,
                                          clientarg:
                                              *mut  raw::c_void)
     ->  raw::c_int;

    pub fn netsnmp_read_data_callback(token: *const  raw::c_char,
                                      line: *mut  raw::c_char);

    #[link_name = "log_addresses"]
    pub static mut log_addresses:  raw::c_int;

    #[link_name = "lastAddrAge"]
    pub static mut lastAddrAge:  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_request_info_s {
    pub requestvb: *mut netsnmp_variable_list,
    pub parent_data: *mut netsnmp_data_list,
    pub agent_req_info: *mut netsnmp_agent_request_info_s,
    pub range_end: *mut oid,
    pub range_end_len: usize,
    pub delegated:  raw::c_int,
    pub processed:  raw::c_int,
    pub inclusive:  raw::c_int,
    pub status:  raw::c_int,
    pub index:  raw::c_int,
    pub repeat:  raw::c_int,
    pub orig_repeat:  raw::c_int,
    pub requestvb_start: *mut netsnmp_variable_list,
    pub next: *mut netsnmp_request_info_s,
    pub prev: *mut netsnmp_request_info_s,
    pub subtree: *mut netsnmp_subtree_s,
}

impl Clone for netsnmp_request_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_request_info = netsnmp_request_info_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_set_info_s {
    pub action:  raw::c_int,
    pub stateRef: *mut  raw::c_void,
    pub oldData: *mut *mut  raw::c_void,
    pub setCleanupFlags:  raw::c_int,
}

impl Clone for netsnmp_set_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_set_info = netsnmp_set_info_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_tree_cache_s {
    pub subtree: *mut netsnmp_subtree_s,
    pub requests_begin: *mut netsnmp_request_info,
    pub requests_end: *mut netsnmp_request_info,
}

impl Clone for netsnmp_tree_cache_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_tree_cache = netsnmp_tree_cache_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_agent_request_info_s {
    pub mode:  raw::c_int,
    pub asp: *mut netsnmp_agent_session_s,
    pub agent_data: *mut netsnmp_data_list,
}

impl Clone for netsnmp_agent_request_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_agent_request_info = netsnmp_agent_request_info_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_cachemap_s {
    pub globalid:  raw::c_int,
    pub cacheid:  raw::c_int,
    pub next: *mut netsnmp_cachemap_s,
}

impl Clone for netsnmp_cachemap_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_cachemap = netsnmp_cachemap_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_agent_session_s {
    pub mode:  raw::c_int,
    pub session: *mut netsnmp_session,
    pub pdu: *mut netsnmp_pdu,
    pub orig_pdu: *mut netsnmp_pdu,
    pub rw:  raw::c_int,
    pub exact:  raw::c_int,
    pub status:  raw::c_int,
    pub index:  raw::c_int,
    pub oldmode:  raw::c_int,
    pub next: *mut netsnmp_agent_session_s,
    pub reqinfo: *mut netsnmp_agent_request_info,
    pub requests: *mut netsnmp_request_info,
    pub treecache: *mut netsnmp_tree_cache,
    pub bulkcache: *mut *mut netsnmp_variable_list,
    pub treecache_len:  raw::c_int,
    pub treecache_num:  raw::c_int,
    pub cache_store: *mut netsnmp_cachemap,
    pub vbcount:  raw::c_int,
}

impl Clone for netsnmp_agent_session_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_agent_session = netsnmp_agent_session_s;
extern "C" {
    pub fn netsnmp_addrcache_initialise();

    pub fn netsnmp_addrcache_destroy();

    pub fn netsnmp_addrcache_age();

    pub fn handle_snmp_packet(arg1:  raw::c_int,
                              arg2: *mut netsnmp_session,
                              arg3:  raw::c_int,
                              arg4: *mut netsnmp_pdu,
                              arg5: *mut  raw::c_void)
     ->  raw::c_int;

    pub fn snmp_agent_parse_config(arg1: *mut  raw::c_char,
                                   arg2: *mut  raw::c_char);

    pub fn init_agent_snmp_session(arg1: *mut netsnmp_session,
                                   arg2: *mut netsnmp_pdu)
     -> *mut netsnmp_agent_session;

    pub fn free_agent_snmp_session(arg1: *mut netsnmp_agent_session);

    pub fn netsnmp_remove_and_free_agent_snmp_session(asp:
                                                          *mut netsnmp_agent_session);

    pub fn getNextSessID() ->  raw::c_int;

    pub fn dump_sess_list();

    pub fn init_master_agent() ->  raw::c_int;

    pub fn shutdown_master_agent();

    pub fn agent_check_and_process(block:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_check_outstanding_agent_requests();

    pub fn netsnmp_request_set_error(request: *mut netsnmp_request_info,
                                     error_value:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_check_requests_error(reqs: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_check_all_requests_error(asp: *mut netsnmp_agent_session,
                                            look_for_specific:
                                                 raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_set_all_requests_error(reqinfo:
                                              *mut netsnmp_agent_request_info,
                                          requests: *mut netsnmp_request_info,
                                          error_value:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_request_set_error_idx(requests: *mut netsnmp_request_info,
                                         error_value:  raw::c_int,
                                         idx:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_request_set_error_all(requests: *mut netsnmp_request_info,
                                         error_value:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_set_request_error(reqinfo: *mut netsnmp_agent_request_info,
                                     request: *mut netsnmp_request_info,
                                     error_value:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_set_mode_request_error(mode:  raw::c_int,
                                          request: *mut netsnmp_request_info,
                                          error_value:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_marker_uptime(pm: marker_t) -> raw::c_ulong;

    pub fn netsnmp_timeval_uptime(tv: *mut libc::timeval) -> raw::c_ulong;

    pub fn netsnmp_get_agent_starttime() -> const_marker_t;

    pub fn netsnmp_get_agent_runtime() -> u64;

    pub fn netsnmp_set_agent_starttime(s: marker_t);

    pub fn netsnmp_get_agent_uptime() -> raw::c_ulong;

    pub fn netsnmp_set_agent_uptime(hsec: raw::c_ulong);

    pub fn netsnmp_check_transaction_id(transaction_id:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_agent_check_packet(arg1: *mut netsnmp_session,
                                      arg2: *mut Struct_netsnmp_transport_s,
                                      arg3: *mut  raw::c_void,
                                      arg4:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_agent_check_parse(arg1: *mut netsnmp_session,
                                     arg2: *mut netsnmp_pdu,
                                     arg3:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_allocate_globalcacheid() ->  raw::c_int;

    pub fn netsnmp_remove_delegated_requests_for_session(sess:
                                                             *mut netsnmp_session)
     ->  raw::c_int;

    pub fn netsnmp_register_agent_nsap(t: *mut Struct_netsnmp_transport_s)
     ->  raw::c_int;

    pub fn netsnmp_deregister_agent_nsap(handle:  raw::c_int);

    pub fn netsnmp_agent_add_list_data(agent: *mut netsnmp_agent_request_info,
                                       node: *mut netsnmp_data_list);

    pub fn netsnmp_agent_remove_list_data(ari:
                                              *mut netsnmp_agent_request_info,
                                          name: *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_agent_get_list_data(agent: *mut netsnmp_agent_request_info,
                                       name: *const  raw::c_char)
     -> *mut  raw::c_void;

    pub fn netsnmp_free_agent_data_set(agent:
                                           *mut netsnmp_agent_request_info);

    pub fn netsnmp_free_agent_data_sets(agent:
                                            *mut netsnmp_agent_request_info);

    pub fn netsnmp_free_agent_request_info(ari:
                                               *mut netsnmp_agent_request_info);
}
#[repr(C)]
pub struct variable {
    pub magic: raw::c_uchar,
    pub type_: raw::c_char,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 128usize],
}

extern "C" {
    pub fn netsnmp_duplicate_variable(var: *const variable) -> *mut variable;
}
pub type WriteMethod =
    ::std::option::Option<unsafe extern "C" fn(action:  raw::c_int,
                                               var_val: *mut raw::c_uchar,
                                               var_val_type: raw::c_uchar,
                                               var_val_len: usize,
                                               statP: *mut raw::c_uchar,
                                               name: *mut oid, length: usize)
                              ->  raw::c_int>;
pub type FindVarMethod =
    ::std::option::Option<unsafe extern "C" fn(vp: *mut variable,
                                               name: *mut oid,
                                               length: *mut usize,
                                               exact:  raw::c_int,
                                               var_len: *mut usize,
                                               write_method: *mut WriteMethod)
                              -> *mut raw::c_uchar>;
pub type AddVarMethod =
    ::std::option::Option<unsafe extern "C" fn(asp:
                                                   *mut netsnmp_agent_session,
                                               vbp:
                                                   *mut netsnmp_variable_list)
                              ->  raw::c_int>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nlist([u8; 0]);
extern "C" {
    #[link_name = "long_return"]
    pub static mut long_return:  raw::c_long;

    #[link_name = "return_buf"]
    pub static mut return_buf: [raw::c_uchar; 0usize];

    #[link_name = "nullOid"]
    pub static mut nullOid: [oid; 0usize];

    #[link_name = "nullOidLen"]
    pub static mut nullOidLen:  raw::c_int;

    pub fn init_agent(arg1: *const  raw::c_char)
     ->  raw::c_int;

    pub fn shutdown_agent();

    pub fn should_init(module_name: *const  raw::c_char)
     ->  raw::c_int;

    pub fn add_to_init_list(module_list: *mut  raw::c_char);

    pub fn netsnmp_enable_subagent();
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_handler_registration_s {
    pub handlerName: *mut  raw::c_char,
    pub contextName: *mut  raw::c_char,
    pub rootoid: *mut oid,
    pub rootoid_len: usize,
    pub handler: *mut netsnmp_mib_handler,
    pub modes:  raw::c_int,
    pub priority:  raw::c_int,
    pub range_subid:  raw::c_int,
    pub range_ubound: oid,
    pub timeout:  raw::c_int,
    pub global_cacheid:  raw::c_int,
    pub my_reg_void: *mut  raw::c_void,
}

impl Clone for netsnmp_handler_registration_s {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_mib_handler_s {
    pub handler_name: *mut  raw::c_char,
    pub myvoid: *mut  raw::c_void,
    pub flags:  raw::c_int,
    pub access_method: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut netsnmp_mib_handler_s,
                                                                  arg2:
                                                                      *mut netsnmp_handler_registration_s,
                                                                  arg3:
                                                                      *mut netsnmp_agent_request_info_s,
                                                                  arg4:
                                                                      *mut netsnmp_request_info_s)
                                                 ->  raw::c_int>,
    pub data_clone: ::std::option::Option<unsafe extern "C" fn(myvoid:
                                                                   *mut  raw::c_void)
                                              -> *mut  raw::c_void>,
    pub data_free: ::std::option::Option<unsafe extern "C" fn(myvoid:
                                                                  *mut  raw::c_void)>,
    pub next: *mut netsnmp_mib_handler_s,
    pub prev: *mut netsnmp_mib_handler_s,
}

impl Clone for netsnmp_mib_handler_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_mib_handler = netsnmp_mib_handler_s;
pub type netsnmp_handler_registration = netsnmp_handler_registration_s;
pub type Netsnmp_Node_Handler =
    ::std::option::Option<unsafe extern "C" fn(handler:
                                                   *mut netsnmp_mib_handler,
                                               reginfo:
                                                   *mut netsnmp_handler_registration,
                                               reqinfo:
                                                   *mut netsnmp_agent_request_info,
                                               requests:
                                                   *mut netsnmp_request_info)
                              ->  raw::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_handler_args_s {
    pub handler: *mut netsnmp_mib_handler,
    pub reginfo: *mut netsnmp_handler_registration,
    pub reqinfo: *mut netsnmp_agent_request_info,
    pub requests: *mut netsnmp_request_info,
}

impl Clone for netsnmp_handler_args_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_handler_args = netsnmp_handler_args_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_delegated_cache_s {
    pub transaction_id:  raw::c_int,
    pub handler: *mut netsnmp_mib_handler,
    pub reginfo: *mut netsnmp_handler_registration,
    pub reqinfo: *mut netsnmp_agent_request_info,
    pub requests: *mut netsnmp_request_info,
    pub localinfo: *mut  raw::c_void,
}

impl Clone for netsnmp_delegated_cache_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_delegated_cache = netsnmp_delegated_cache_s;
extern "C" {
    pub fn netsnmp_init_handler_conf();

    pub fn netsnmp_register_handler(reginfo:
                                        *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_unregister_handler(reginfo:
                                          *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_register_handler_nocallback(reginfo:
                                                   *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_inject_handler(reginfo: *mut netsnmp_handler_registration,
                                  handler: *mut netsnmp_mib_handler)
     ->  raw::c_int;

    pub fn netsnmp_inject_handler_before(reginfo:
                                             *mut netsnmp_handler_registration,
                                         handler: *mut netsnmp_mib_handler,
                                         before_what:
                                             *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_find_handler_by_name(reginfo:
                                            *mut netsnmp_handler_registration,
                                        name: *const  raw::c_char)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_find_handler_data_by_name(reginfo:
                                                 *mut netsnmp_handler_registration,
                                             name:
                                                 *const  raw::c_char)
     -> *mut  raw::c_void;

    pub fn netsnmp_call_handlers(reginfo: *mut netsnmp_handler_registration,
                                 reqinfo: *mut netsnmp_agent_request_info,
                                 requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_call_handler(next_handler: *mut netsnmp_mib_handler,
                                reginfo: *mut netsnmp_handler_registration,
                                reqinfo: *mut netsnmp_agent_request_info,
                                requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_call_next_handler(current: *mut netsnmp_mib_handler,
                                     reginfo:
                                         *mut netsnmp_handler_registration,
                                     reqinfo: *mut netsnmp_agent_request_info,
                                     requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_call_next_handler_one_request(current:
                                                     *mut netsnmp_mib_handler,
                                                 reginfo:
                                                     *mut netsnmp_handler_registration,
                                                 reqinfo:
                                                     *mut netsnmp_agent_request_info,
                                                 requests:
                                                     *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_create_handler(name: *const  raw::c_char,
                                  handler_access_method: Netsnmp_Node_Handler)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_handler_registration_create(name:
                                                   *const  raw::c_char,
                                               handler:
                                                   *mut netsnmp_mib_handler,
                                               reg_oid: *const oid,
                                               reg_oid_len: usize,
                                               modes:  raw::c_int)
     -> *mut netsnmp_handler_registration;

    pub fn netsnmp_create_handler_registration(name:
                                                   *const  raw::c_char,
                                               handler_access_method:
                                                   Netsnmp_Node_Handler,
                                               reg_oid: *const oid,
                                               reg_oid_len: usize,
                                               modes:  raw::c_int)
     -> *mut netsnmp_handler_registration;

    pub fn netsnmp_create_delegated_cache(arg1: *mut netsnmp_mib_handler,
                                          arg2:
                                              *mut netsnmp_handler_registration,
                                          arg3:
                                              *mut netsnmp_agent_request_info,
                                          arg4: *mut netsnmp_request_info,
                                          arg5: *mut  raw::c_void)
     -> *mut netsnmp_delegated_cache;

    pub fn netsnmp_free_delegated_cache(dcache: *mut netsnmp_delegated_cache);

    pub fn netsnmp_handler_check_cache(dcache: *mut netsnmp_delegated_cache)
     -> *mut netsnmp_delegated_cache;

    pub fn netsnmp_register_handler_by_name(arg1:
                                                *const  raw::c_char,
                                            arg2: *mut netsnmp_mib_handler);

    pub fn netsnmp_clear_handler_list();

    pub fn netsnmp_request_add_list_data(request: *mut netsnmp_request_info,
                                         node: *mut netsnmp_data_list);

    pub fn netsnmp_request_remove_list_data(request:
                                                *mut netsnmp_request_info,
                                            name:
                                                *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_request_get_list_data(request: *mut netsnmp_request_info,
                                         name: *const  raw::c_char)
     -> *mut  raw::c_void;

    pub fn netsnmp_free_request_data_set(request: *mut netsnmp_request_info);

    pub fn netsnmp_free_request_data_sets(request: *mut netsnmp_request_info);

    pub fn netsnmp_handler_free(arg1: *mut netsnmp_mib_handler);

    pub fn netsnmp_handler_dup(arg1: *mut netsnmp_mib_handler)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_handler_registration_dup(arg1:
                                                *mut netsnmp_handler_registration)
     -> *mut netsnmp_handler_registration;

    pub fn netsnmp_handler_registration_free(arg1:
                                                 *mut netsnmp_handler_registration);

    pub fn netsnmp_handler_mark_requests_as_delegated(arg1:
                                                          *mut netsnmp_request_info,
                                                      arg2:
                                                           raw::c_int);

    pub fn netsnmp_handler_get_parent_data(arg1: *mut netsnmp_request_info,
                                           arg2:
                                               *const  raw::c_char)
     -> *mut  raw::c_void;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_subtree_s {
    pub name_a: *mut oid,
    pub namelen: raw::c_uchar,
    pub start_a: *mut oid,
    pub start_len: raw::c_uchar,
    pub end_a: *mut oid,
    pub end_len: raw::c_uchar,
    pub variables: *mut variable,
    pub variables_len:  raw::c_int,
    pub variables_width:  raw::c_int,
    pub label_a: *mut  raw::c_char,
    pub session: *mut netsnmp_session,
    pub flags: raw::c_uchar,
    pub priority: raw::c_uchar,
    pub timeout:  raw::c_int,
    pub next: *mut netsnmp_subtree_s,
    pub prev: *mut netsnmp_subtree_s,
    pub children: *mut netsnmp_subtree_s,
    pub range_subid:  raw::c_int,
    pub range_ubound: oid,
    pub reginfo: *mut netsnmp_handler_registration,
    pub cacheid:  raw::c_int,
    pub global_cacheid:  raw::c_int,
    pub oid_off: usize,
}

impl Clone for netsnmp_subtree_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_subtree = netsnmp_subtree_s;
#[repr(C)]
#[derive(Copy)]
pub struct variable1 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 1usize],
}

impl Clone for variable1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable2 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 2usize],
}

impl Clone for variable2 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable3 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 3usize],
}

impl Clone for variable3 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable4 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 4usize],
}

impl Clone for variable4 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable7 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 7usize],
}

impl Clone for variable7 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable8 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 8usize],
}

impl Clone for variable8 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct variable13 {
    pub magic: raw::c_uchar,
    pub type_: raw::c_uchar,
    pub acl: raw::c_ushort,
    pub findVar: FindVarMethod,
    pub namelen: raw::c_uchar,
    pub name: [oid; 13usize],
}

impl Clone for variable13 {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "external_readfd"]
    pub static mut external_readfd: [ raw::c_int; 32usize];

    #[link_name = "external_readfdlen"]
    pub static mut external_readfdlen:  raw::c_int;

    #[link_name = "external_writefd"]
    pub static mut external_writefd: [ raw::c_int; 32usize];

    #[link_name = "external_writefdlen"]
    pub static mut external_writefdlen:  raw::c_int;

    #[link_name = "external_exceptfd"]
    pub static mut external_exceptfd: [ raw::c_int; 32usize];

    #[link_name = "external_exceptfdlen"]
    pub static mut external_exceptfdlen:  raw::c_int;

    #[link_name = "external_readfdfunc"]
    pub static mut external_readfdfunc:
               [::std::option::Option<unsafe extern "C" fn(arg1:
                                                                raw::c_int,
                                                           arg2:
                                                               *mut  raw::c_void)>; 32usize];

    #[link_name = "external_writefdfunc"]
    pub static mut external_writefdfunc:
               [::std::option::Option<unsafe extern "C" fn(arg1:
                                                                raw::c_int,
                                                           arg2:
                                                               *mut  raw::c_void)>; 32usize];

    #[link_name = "external_exceptfdfunc"]
    pub static mut external_exceptfdfunc:
               [::std::option::Option<unsafe extern "C" fn(arg1:
                                                                raw::c_int,
                                                           arg2:
                                                               *mut  raw::c_void)>; 32usize];

    #[link_name = "external_readfd_data"]
    pub static mut external_readfd_data:
               [*mut  raw::c_void; 32usize];

    #[link_name = "external_writefd_data"]
    pub static mut external_writefd_data:
               [*mut  raw::c_void; 32usize];

    #[link_name = "external_exceptfd_data"]
    pub static mut external_exceptfd_data:
               [*mut  raw::c_void; 32usize];

    pub fn register_readfd(arg1:  raw::c_int,
                           func:
                               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                               raw::c_int,
                                                                          arg2:
                                                                              *mut  raw::c_void)>,
                           arg2: *mut  raw::c_void)
     ->  raw::c_int;

    pub fn register_writefd(arg1:  raw::c_int,
                            func:
                                ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                raw::c_int,
                                                                           arg2:
                                                                               *mut  raw::c_void)>,
                            arg2: *mut  raw::c_void)
     ->  raw::c_int;

    pub fn register_exceptfd(arg1:  raw::c_int,
                             func:
                                 ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                 raw::c_int,
                                                                            arg2:
                                                                                *mut  raw::c_void)>,
                             arg2: *mut  raw::c_void)
     ->  raw::c_int;

    pub fn unregister_readfd(arg1:  raw::c_int)
     ->  raw::c_int;

    pub fn unregister_writefd(arg1:  raw::c_int)
     ->  raw::c_int;

    pub fn unregister_exceptfd(arg1:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_external_event_info(numfds: *mut  raw::c_int,
                                       readfds: *mut libc::fd_set,
                                       writefds: *mut libc::fd_set,
                                       exceptfds: *mut libc::fd_set);

    pub fn netsnmp_external_event_info2(numfds: *mut  raw::c_int,
                                        readfds: *mut netsnmp_large_fd_set,
                                        writefds: *mut netsnmp_large_fd_set,
                                        exceptfds: *mut netsnmp_large_fd_set);

    pub fn netsnmp_dispatch_external_events(count: *mut  raw::c_int,
                                            readfds: *mut libc::fd_set,
                                            writefds: *mut libc::fd_set,
                                            exceptfds: *mut libc::fd_set);

    pub fn netsnmp_dispatch_external_events2(count:
                                                 *mut  raw::c_int,
                                             readfds:
                                                 *mut netsnmp_large_fd_set,
                                             writefds:
                                                 *mut netsnmp_large_fd_set,
                                             exceptfds:
                                                 *mut netsnmp_large_fd_set);
}
#[repr(C)]
#[derive(Copy)]
pub struct view_parameters {
    pub pdu: *mut netsnmp_pdu,
    pub name: *mut oid,
    pub namelen: usize,
    pub test:  raw::c_int,
    pub errorcode:  raw::c_int,
    pub check_subtree:  raw::c_int,
}

impl Clone for view_parameters {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct register_parameters {
    pub name: *mut oid,
    pub namelen: usize,
    pub priority:  raw::c_int,
    pub range_subid:  raw::c_int,
    pub range_ubound: oid,
    pub timeout:  raw::c_int,
    pub flags: raw::c_uchar,
    pub contextName: *const  raw::c_char,
    pub session: *mut netsnmp_session,
    pub reginfo: *mut netsnmp_handler_registration,
}

impl Clone for register_parameters {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct subtree_context_cache_s {
    pub context_name: *const  raw::c_char,
    pub first_subtree: *mut netsnmp_subtree_s,
    pub next: *mut subtree_context_cache_s,
}

impl Clone for subtree_context_cache_s {
    fn clone(&self) -> Self { *self }
}
pub type subtree_context_cache = subtree_context_cache_s;
extern "C" {
    pub fn setup_tree();

    pub fn shutdown_tree();

    pub fn netsnmp_subtree_find(arg1: *const oid, arg2: usize,
                                arg3: *mut netsnmp_subtree,
                                context_name: *const  raw::c_char)
     -> *mut netsnmp_subtree;

    pub fn netsnmp_subtree_find_next(arg1: *const oid, arg2: usize,
                                     arg3: *mut netsnmp_subtree,
                                     context_name:
                                         *const  raw::c_char)
     -> *mut netsnmp_subtree;

    pub fn netsnmp_subtree_find_prev(arg1: *const oid, arg2: usize,
                                     arg3: *mut netsnmp_subtree,
                                     context_name:
                                         *const  raw::c_char)
     -> *mut netsnmp_subtree;

    pub fn netsnmp_subtree_find_first(context_name:
                                          *const  raw::c_char)
     -> *mut netsnmp_subtree;

    pub fn get_session_for_oid(arg1: *const oid, arg2: usize,
                               context_name: *const  raw::c_char)
     -> *mut netsnmp_session;

    pub fn get_top_context_cache() -> *mut subtree_context_cache;

    pub fn netsnmp_set_lookup_cache_size(newsize:  raw::c_int);

    pub fn netsnmp_get_lookup_cache_size() ->  raw::c_int;

    pub fn register_mib(arg1: *const  raw::c_char,
                        arg2: *const variable, arg3: usize, arg4: usize,
                        arg5: *const oid, arg6: usize)
     ->  raw::c_int;

    pub fn register_mib_priority(arg1: *const  raw::c_char,
                                 arg2: *const variable, arg3: usize,
                                 arg4: usize, arg5: *const oid, arg6: usize,
                                 arg7:  raw::c_int)
     ->  raw::c_int;

    pub fn register_mib_range(arg1: *const  raw::c_char,
                              arg2: *const variable, arg3: usize, arg4: usize,
                              arg5: *const oid, arg6: usize,
                              arg7:  raw::c_int,
                              arg8:  raw::c_int, arg9: oid,
                              arg10: *mut netsnmp_session)
     ->  raw::c_int;

    pub fn register_mib_context(arg1: *const  raw::c_char,
                                arg2: *const variable, arg3: usize,
                                arg4: usize, arg5: *const oid, arg6: usize,
                                arg7:  raw::c_int,
                                arg8:  raw::c_int, arg9: oid,
                                arg10: *mut netsnmp_session,
                                arg11: *const  raw::c_char,
                                arg12:  raw::c_int,
                                arg13:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_register_mib_table_row(arg1: *const  raw::c_char,
                                          arg2: *const variable, arg3: usize,
                                          arg4: usize, arg5: *mut oid,
                                          arg6: usize,
                                          arg7:  raw::c_int,
                                          arg8:  raw::c_int,
                                          arg9: *mut netsnmp_session,
                                          arg10:
                                              *const  raw::c_char,
                                          arg11:  raw::c_int,
                                          arg12:  raw::c_int)
     ->  raw::c_int;

    pub fn unregister_mib(arg1: *mut oid, arg2: usize)
     ->  raw::c_int;

    pub fn unregister_mib_priority(arg1: *mut oid, arg2: usize,
                                   arg3:  raw::c_int)
     ->  raw::c_int;

    pub fn unregister_mib_range(arg1: *mut oid, arg2: usize,
                                arg3:  raw::c_int,
                                arg4:  raw::c_int, arg5: oid)
     ->  raw::c_int;

    pub fn unregister_mib_context(arg1: *mut oid, arg2: usize,
                                  arg3:  raw::c_int,
                                  arg4:  raw::c_int, arg5: oid,
                                  arg6: *const  raw::c_char)
     ->  raw::c_int;

    pub fn clear_context();

    pub fn unregister_mibs_by_session(arg1: *mut netsnmp_session);

    pub fn netsnmp_unregister_mib_table_row(mibloc: *mut oid,
                                            mibloclen: usize,
                                            priority:  raw::c_int,
                                            var_subid:  raw::c_int,
                                            range_ubound: oid,
                                            context:
                                                *const  raw::c_char)
     ->  raw::c_int;

    pub fn compare_tree(arg1: *const oid, arg2: usize, arg3: *const oid,
                        arg4: usize) ->  raw::c_int;

    pub fn in_a_view(arg1: *mut oid, arg2: *mut usize, arg3: *mut netsnmp_pdu,
                     arg4:  raw::c_int) ->  raw::c_int;

    pub fn check_access(pdu: *mut netsnmp_pdu) ->  raw::c_int;

    pub fn netsnmp_acm_check_subtree(arg1: *mut netsnmp_pdu, arg2: *mut oid,
                                     arg3: usize) ->  raw::c_int;

    pub fn register_mib_reattach();

    pub fn register_mib_detach();

    #[link_name = "external_signal_scheduled"]
    pub static mut external_signal_scheduled:
               [ raw::c_int; 32usize];

    #[link_name = "external_signal_handler"]
    pub static mut external_signal_handler:
               [::std::option::Option<unsafe extern "C" fn(arg1:
                                                                raw::c_int)>; 32usize];

    pub fn register_signal(arg1:  raw::c_int,
                           func:
                               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                               raw::c_int)>)
     ->  raw::c_int;

    pub fn unregister_signal(arg1:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_register_mib(arg1: *const  raw::c_char,
                                arg2: *mut variable, arg3: usize, arg4: usize,
                                arg5: *mut oid, arg6: usize,
                                arg7:  raw::c_int,
                                arg8:  raw::c_int, arg9: oid,
                                arg10: *mut netsnmp_session,
                                arg11: *const  raw::c_char,
                                arg12:  raw::c_int,
                                arg13:  raw::c_int,
                                arg14: *mut netsnmp_handler_registration_s,
                                arg15:  raw::c_int)
     ->  raw::c_int;

    pub fn init_agent_read_config(arg1: *const  raw::c_char);

    pub fn update_config();

    pub fn snmpd_register_config_handler(token: *const  raw::c_char,
                                         parser:
                                             ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                            *const  raw::c_char,
                                                                                        arg2:
                                                                                            *mut  raw::c_char)>,
                                         releaser:
                                             ::std::option::Option<unsafe extern "C" fn()>,
                                         help: *const  raw::c_char);

    pub fn snmpd_register_const_config_handler(arg1:
                                                   *const  raw::c_char,
                                               parser:
                                                   ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                  *const  raw::c_char,
                                                                                              arg2:
                                                                                                  *const  raw::c_char)>,
                                               releaser:
                                                   ::std::option::Option<unsafe extern "C" fn()>,
                                               arg2:
                                                   *const  raw::c_char);

    pub fn snmpd_unregister_config_handler(arg1:
                                               *const  raw::c_char);

    pub fn snmpd_store_config(arg1: *const  raw::c_char);
}
#[repr(C)]
#[derive(Copy)]
pub struct agent_add_trap_args {
    pub ss: *mut netsnmp_session,
    pub confirm:  raw::c_int,
}

impl Clone for agent_add_trap_args {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn init_traps();

    pub fn send_easy_trap(arg1:  raw::c_int,
                          arg2:  raw::c_int);

    pub fn send_trap_pdu(arg1: *mut netsnmp_pdu);

    pub fn send_v2trap(arg1: *mut netsnmp_variable_list);

    pub fn send_v3trap(vars: *mut netsnmp_variable_list,
                       context: *const  raw::c_char);

    pub fn send_trap_vars(arg1:  raw::c_int,
                          arg2:  raw::c_int,
                          arg3: *mut netsnmp_variable_list);

    pub fn send_trap_vars_with_context(trap:  raw::c_int,
                                       specific:  raw::c_int,
                                       vars: *mut netsnmp_variable_list,
                                       context:
                                           *const  raw::c_char);

    pub fn send_enterprise_trap_vars(trap:  raw::c_int,
                                     specific:  raw::c_int,
                                     enterprise: *const oid,
                                     enterprise_length:  raw::c_int,
                                     vars: *mut netsnmp_variable_list);

    pub fn netsnmp_send_traps(trap:  raw::c_int,
                              specific:  raw::c_int,
                              enterprise: *const oid,
                              enterprise_length:  raw::c_int,
                              vars: *mut netsnmp_variable_list,
                              context: *const  raw::c_char,
                              flags:  raw::c_int)
     ->  raw::c_int;

    pub fn snmpd_parse_config_authtrap(arg1: *const  raw::c_char,
                                       arg2: *mut  raw::c_char);

    pub fn snmpd_parse_config_trapsink(arg1: *const  raw::c_char,
                                       arg2: *mut  raw::c_char);

    pub fn snmpd_parse_config_trap2sink(arg1: *const  raw::c_char,
                                        arg2: *mut  raw::c_char);

    pub fn snmpd_parse_config_informsink(arg1: *const  raw::c_char,
                                         arg2: *mut  raw::c_char);

    pub fn snmpd_parse_config_trapsess(arg1: *const  raw::c_char,
                                       arg2: *mut  raw::c_char);

    pub fn snmpd_free_trapsinks();

    pub fn snmpd_parse_config_trapcommunity(arg1:
                                                *const  raw::c_char,
                                            arg2:
                                                *mut  raw::c_char);

    pub fn snmpd_free_trapcommunity();

    pub fn send_trap_to_sess(sess: *mut netsnmp_session,
                             template_pdu: *mut netsnmp_pdu);

    pub fn create_trap_session(arg1: *mut  raw::c_char,
                               arg2: raw::c_ushort,
                               arg3: *mut  raw::c_char,
                               arg4:  raw::c_int,
                               arg5:  raw::c_int)
     ->  raw::c_int;

    pub fn add_trap_session(arg1: *mut netsnmp_session,
                            arg2:  raw::c_int,
                            arg3:  raw::c_int,
                            arg4:  raw::c_int)
     ->  raw::c_int;

    pub fn remove_trap_session(arg1: *mut netsnmp_session)
     ->  raw::c_int;

    pub fn convert_v2_to_v1(arg1: *mut netsnmp_variable_list,
                            arg2: *mut netsnmp_pdu);

    pub fn convert_v1_to_v2(arg1: *mut netsnmp_pdu)
     -> *mut netsnmp_variable_list;

    pub fn convert_v2pdu_to_v1(arg1: *mut netsnmp_pdu) -> *mut netsnmp_pdu;

    pub fn convert_v1pdu_to_v2(arg1: *mut netsnmp_pdu) -> *mut netsnmp_pdu;

    pub fn netsnmp_register_instance(reginfo:
                                         *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_instance(reginfo:
                                                   *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_get_instance_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_read_only_ulong_instance(name:
                                                         *const  raw::c_char,
                                                     reg_oid: *const oid,
                                                     reg_oid_len: usize,
                                                     it: *mut raw::c_ulong,
                                                     subhandler:
                                                         Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_ulong_instance(name:
                                               *const  raw::c_char,
                                           reg_oid: *const oid,
                                           reg_oid_len: usize,
                                           it: *mut raw::c_ulong,
                                           subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_counter32_instance(name:
                                                             *const  raw::c_char,
                                                         reg_oid: *const oid,
                                                         reg_oid_len: usize,
                                                         it: *mut raw::c_ulong,
                                                         subhandler:
                                                             Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_long_instance(name:
                                                        *const  raw::c_char,
                                                    reg_oid: *const oid,
                                                    reg_oid_len: usize,
                                                    it:
                                                        *mut  raw::c_long,
                                                    subhandler:
                                                        Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_long_instance(name: *const  raw::c_char,
                                          reg_oid: *const oid,
                                          reg_oid_len: usize,
                                          it: *mut  raw::c_long,
                                          subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_int_instance(name:
                                                       *const  raw::c_char,
                                                   reg_oid: *const oid,
                                                   reg_oid_len: usize,
                                                   it:
                                                       *mut  raw::c_int,
                                                   subhandler:
                                                       Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_int_instance(name: *const  raw::c_char,
                                         reg_oid: *const oid,
                                         reg_oid_len: usize,
                                         it: *mut  raw::c_int,
                                         subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_ulong_instance_context(name:
                                                                 *const  raw::c_char,
                                                             reg_oid:
                                                                 *const oid,
                                                             reg_oid_len:
                                                                 usize,
                                                             it: *mut raw::c_ulong,
                                                             subhandler:
                                                                 Netsnmp_Node_Handler,
                                                             contextName:
                                                                 *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_ulong_instance_context(name:
                                                       *const  raw::c_char,
                                                   reg_oid: *const oid,
                                                   reg_oid_len: usize,
                                                   it: *mut raw::c_ulong,
                                                   subhandler:
                                                       Netsnmp_Node_Handler,
                                                   contextName:
                                                       *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_counter32_instance_context(name:
                                                                     *const  raw::c_char,
                                                                 reg_oid:
                                                                     *const oid,
                                                                 reg_oid_len:
                                                                     usize,
                                                                 it:
                                                                     *mut raw::c_ulong,
                                                                 subhandler:
                                                                     Netsnmp_Node_Handler,
                                                                 contextName:
                                                                     *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_long_instance_context(name:
                                                                *const  raw::c_char,
                                                            reg_oid:
                                                                *const oid,
                                                            reg_oid_len:
                                                                usize,
                                                            it:
                                                                *mut  raw::c_long,
                                                            subhandler:
                                                                Netsnmp_Node_Handler,
                                                            contextName:
                                                                *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_long_instance_context(name:
                                                      *const  raw::c_char,
                                                  reg_oid: *const oid,
                                                  reg_oid_len: usize,
                                                  it:
                                                      *mut  raw::c_long,
                                                  subhandler:
                                                      Netsnmp_Node_Handler,
                                                  contextName:
                                                      *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_int_instance_context(name:
                                                               *const  raw::c_char,
                                                           reg_oid:
                                                               *const oid,
                                                           reg_oid_len: usize,
                                                           it:
                                                               *mut  raw::c_int,
                                                           subhandler:
                                                               Netsnmp_Node_Handler,
                                                           contextName:
                                                               *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_int_instance_context(name:
                                                     *const  raw::c_char,
                                                 reg_oid: *const oid,
                                                 reg_oid_len: usize,
                                                 it:
                                                     *mut  raw::c_int,
                                                 subhandler:
                                                     Netsnmp_Node_Handler,
                                                 contextName:
                                                     *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_register_num_file_instance(name:
                                                  *const  raw::c_char,
                                              reg_oid: *const oid,
                                              reg_oid_len: usize,
                                              file_name:
                                                  *const  raw::c_char,
                                              asn_type:  raw::c_int,
                                              mode:  raw::c_int,
                                              subhandler:
                                                  Netsnmp_Node_Handler,
                                              contextName:
                                                  *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_instance_helper_handler(handler: *mut netsnmp_mib_handler,
                                           reginfo:
                                               *mut netsnmp_handler_registration,
                                           reqinfo:
                                               *mut netsnmp_agent_request_info,
                                           requests:
                                               *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_instance_num_file_handler(handler:
                                                 *mut netsnmp_mib_handler,
                                             reginfo:
                                                 *mut netsnmp_handler_registration,
                                             reqinfo:
                                                 *mut netsnmp_agent_request_info,
                                             requests:
                                                 *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_baby_steps_modes_s {
    pub refcnt:  raw::c_int,
    pub registered: raw::c_uint,
    pub completed: raw::c_uint,
}

impl Clone for netsnmp_baby_steps_modes_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_baby_steps_modes = netsnmp_baby_steps_modes_s;
extern "C" {
    pub fn netsnmp_baby_steps_init();

    pub fn netsnmp_baby_steps_handler_get(modes: raw::c_ulong)
     -> *mut netsnmp_mib_handler;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_baby_steps_access_methods_s {
    pub pre_request: Netsnmp_Node_Handler,
    pub object_lookup: Netsnmp_Node_Handler,
    pub get_values: Netsnmp_Node_Handler,
    pub object_syntax_checks: Netsnmp_Node_Handler,
    pub row_creation: Netsnmp_Node_Handler,
    pub undo_setup: Netsnmp_Node_Handler,
    pub set_values: Netsnmp_Node_Handler,
    pub consistency_checks: Netsnmp_Node_Handler,
    pub commit: Netsnmp_Node_Handler,
    pub undo_sets: Netsnmp_Node_Handler,
    pub undo_cleanup: Netsnmp_Node_Handler,
    pub undo_commit: Netsnmp_Node_Handler,
    pub irreversible_commit: Netsnmp_Node_Handler,
    pub post_request: Netsnmp_Node_Handler,
    pub my_access_void: *mut  raw::c_void,
}

impl Clone for netsnmp_baby_steps_access_methods_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_baby_steps_access_methods =
    netsnmp_baby_steps_access_methods_s;
extern "C" {
    pub fn netsnmp_baby_steps_access_multiplexer_get(arg1:
                                                         *mut netsnmp_baby_steps_access_methods)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_baby_step_mode2flag(mode: raw::c_uint) ->  raw::c_int;

    pub fn netsnmp_register_scalar(reginfo: *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_scalar(reginfo:
                                                 *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_get_scalar_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_scalar_helper_handler(handler: *mut netsnmp_mib_handler,
                                         reginfo:
                                             *mut netsnmp_handler_registration,
                                         reqinfo:
                                             *mut netsnmp_agent_request_info,
                                         requests: *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_scalar_group_s {
    pub lbound: oid,
    pub ubound: oid,
}

impl Clone for netsnmp_scalar_group_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_scalar_group = netsnmp_scalar_group_s;
extern "C" {
    pub fn netsnmp_register_scalar_group(reginfo:
                                             *mut netsnmp_handler_registration,
                                         first: oid, last: oid)
     ->  raw::c_int;

    pub fn netsnmp_get_scalar_group_handler(first: oid, last: oid)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_scalar_group_helper_handler(handler:
                                                   *mut netsnmp_mib_handler,
                                               reginfo:
                                                   *mut netsnmp_handler_registration,
                                               reqinfo:
                                                   *mut netsnmp_agent_request_info,
                                               requests:
                                                   *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_watcher_info_s {
    pub data: *mut  raw::c_void,
    pub data_size: usize,
    pub max_size: usize,
    pub type_: raw::c_uchar,
    pub flags:  raw::c_int,
    pub data_size_p: *mut usize,
}

impl Clone for netsnmp_watcher_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_watcher_info = netsnmp_watcher_info_s;
extern "C" {
    pub fn netsnmp_register_watched_instance(reginfo:
                                                 *mut netsnmp_handler_registration,
                                             winfo: *mut netsnmp_watcher_info)
     ->  raw::c_int;

    pub fn netsnmp_register_watched_instance2(reginfo:
                                                  *mut netsnmp_handler_registration,
                                              winfo:
                                                  *mut netsnmp_watcher_info)
     ->  raw::c_int;

    pub fn netsnmp_register_watched_scalar(reginfo:
                                               *mut netsnmp_handler_registration,
                                           winfo: *mut netsnmp_watcher_info)
     ->  raw::c_int;

    pub fn netsnmp_register_watched_scalar2(reginfo:
                                                *mut netsnmp_handler_registration,
                                            winfo: *mut netsnmp_watcher_info)
     ->  raw::c_int;

    pub fn netsnmp_register_watched_timestamp(reginfo:
                                                  *mut netsnmp_handler_registration,
                                              timestamp: marker_t)
     ->  raw::c_int;

    pub fn netsnmp_watched_timestamp_register(whandler:
                                                  *mut netsnmp_mib_handler,
                                              reginfo:
                                                  *mut netsnmp_handler_registration,
                                              timestamp: marker_t)
     ->  raw::c_int;

    pub fn netsnmp_register_watched_spinlock(reginfo:
                                                 *mut netsnmp_handler_registration,
                                             spinlock:
                                                 *mut  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_register_ulong_scalar(name: *const  raw::c_char,
                                         reg_oid: *const oid,
                                         reg_oid_len: usize, it: *mut raw::c_ulong,
                                         subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_ulong_scalar(name:
                                                       *const  raw::c_char,
                                                   reg_oid: *const oid,
                                                   reg_oid_len: usize,
                                                   it: *mut raw::c_ulong,
                                                   subhandler:
                                                       Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_long_scalar(name: *const  raw::c_char,
                                        reg_oid: *const oid,
                                        reg_oid_len: usize,
                                        it: *mut  raw::c_long,
                                        subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_long_scalar(name:
                                                      *const  raw::c_char,
                                                  reg_oid: *const oid,
                                                  reg_oid_len: usize,
                                                  it:
                                                      *mut  raw::c_long,
                                                  subhandler:
                                                      Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_int_scalar(name: *const  raw::c_char,
                                       reg_oid: *const oid,
                                       reg_oid_len: usize,
                                       it: *mut  raw::c_int,
                                       subhandler: Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_int_scalar(name:
                                                     *const  raw::c_char,
                                                 reg_oid: *const oid,
                                                 reg_oid_len: usize,
                                                 it:
                                                     *mut  raw::c_int,
                                                 subhandler:
                                                     Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_counter32_scalar(name:
                                                           *const  raw::c_char,
                                                       reg_oid: *const oid,
                                                       reg_oid_len: usize,
                                                       it: *mut raw::c_ulong,
                                                       subhandler:
                                                           Netsnmp_Node_Handler)
     ->  raw::c_int;

    pub fn netsnmp_get_watcher_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_init_watcher_info(arg1: *mut netsnmp_watcher_info,
                                     arg2: *mut  raw::c_void,
                                     arg3: usize, arg4: raw::c_uchar,
                                     arg5:  raw::c_int)
     -> *mut netsnmp_watcher_info;

    pub fn netsnmp_init_watcher_info6(arg1: *mut netsnmp_watcher_info,
                                      arg2: *mut  raw::c_void,
                                      arg3: usize, arg4: raw::c_uchar,
                                      arg5:  raw::c_int,
                                      arg6: usize, arg7: *mut usize)
     -> *mut netsnmp_watcher_info;

    pub fn netsnmp_create_watcher_info(arg1: *mut  raw::c_void,
                                       arg2: usize, arg3: raw::c_uchar,
                                       arg4:  raw::c_int)
     -> *mut netsnmp_watcher_info;

    pub fn netsnmp_create_watcher_info6(arg1: *mut  raw::c_void,
                                        arg2: usize, arg3: raw::c_uchar,
                                        arg4:  raw::c_int,
                                        arg5: usize, arg6: *mut usize)
     -> *mut netsnmp_watcher_info;

    pub fn netsnmp_clone_watcher_info(winfo: *mut netsnmp_watcher_info)
     -> *mut netsnmp_watcher_info;

    pub fn netsnmp_owns_watcher_info(handler: *mut netsnmp_mib_handler);

    pub fn netsnmp_watcher_helper_handler(handler: *mut netsnmp_mib_handler,
                                          reginfo:
                                              *mut netsnmp_handler_registration,
                                          reqinfo:
                                              *mut netsnmp_agent_request_info,
                                          requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_watched_timestamp_handler()
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_watched_timestamp_handler(handler:
                                                 *mut netsnmp_mib_handler,
                                             reginfo:
                                                 *mut netsnmp_handler_registration,
                                             reqinfo:
                                                 *mut netsnmp_agent_request_info,
                                             requests:
                                                 *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_watched_spinlock_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_watched_spinlock_handler(handler: *mut netsnmp_mib_handler,
                                            reginfo:
                                                *mut netsnmp_handler_registration,
                                            reqinfo:
                                                *mut netsnmp_agent_request_info,
                                            requests:
                                                *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_mib_handler_methods_s {
    pub get_handler: *mut netsnmp_mib_handler,
    pub getnext_handler: *mut netsnmp_mib_handler,
    pub getbulk_handler: *mut netsnmp_mib_handler,
    pub set_handler: *mut netsnmp_mib_handler,
}

impl Clone for netsnmp_mib_handler_methods_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_mib_handler_methods = netsnmp_mib_handler_methods_s;
extern "C" {
    pub fn netsnmp_get_multiplexer_handler(arg1:
                                               *mut netsnmp_mib_handler_methods)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_multiplexer_helper_handler(handler:
                                                  *mut netsnmp_mib_handler,
                                              reginfo:
                                                  *mut netsnmp_handler_registration,
                                              reqinfo:
                                                  *mut netsnmp_agent_request_info,
                                              requests:
                                                  *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_register_null(arg1: *mut oid, arg2: usize)
     ->  raw::c_int;

    pub fn netsnmp_register_null_context(arg1: *mut oid, arg2: usize,
                                         contextName:
                                             *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_null_handler(handler: *mut netsnmp_mib_handler,
                                reginfo: *mut netsnmp_handler_registration,
                                reqinfo: *mut netsnmp_agent_request_info,
                                requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_debug_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_init_debug_helper();

    pub fn netsnmp_debug_helper(handler: *mut netsnmp_mib_handler,
                                reginfo: *mut netsnmp_handler_registration,
                                reqinfo: *mut netsnmp_agent_request_info,
                                requests: *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_cache_s {
    pub refcnt:  raw::c_int,
    pub flags:  raw::c_int,
    pub enabled:  raw::c_int,
    pub valid:  raw::c_int,
    pub expired:  raw::c_char,
    pub timeout:  raw::c_int,
    pub timestampM: marker_t,
    pub timer_id: raw::c_ulong,
    pub load_cache: NetsnmpCacheLoad,
    pub free_cache: NetsnmpCacheFree,
    pub magic: *mut  raw::c_void,
    pub cache_hint: *mut netsnmp_handler_args,
    pub next: *mut netsnmp_cache,
    pub prev: *mut netsnmp_cache,
    pub rootoid: *mut oid,
    pub rootoid_len:  raw::c_int,
}

impl Clone for netsnmp_cache_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_cache = netsnmp_cache_s;
pub type NetsnmpCacheLoad =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut netsnmp_cache,
                                               arg2:
                                                   *mut  raw::c_void)
                              ->  raw::c_int>;
pub type NetsnmpCacheFree =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut netsnmp_cache,
                                               arg2:
                                                   *mut  raw::c_void)>;
extern "C" {
    pub fn netsnmp_cache_reqinfo_insert(cache: *mut netsnmp_cache,
                                        reqinfo:
                                            *mut netsnmp_agent_request_info,
                                        name: *const  raw::c_char);

    pub fn netsnmp_cache_reqinfo_extract(reqinfo:
                                             *mut netsnmp_agent_request_info,
                                         name: *const  raw::c_char)
     -> *mut netsnmp_cache;

    pub fn netsnmp_extract_cache_info(arg1: *mut netsnmp_agent_request_info)
     -> *mut netsnmp_cache;

    pub fn netsnmp_cache_check_and_reload(cache: *mut netsnmp_cache)
     ->  raw::c_int;

    pub fn netsnmp_cache_check_expired(cache: *mut netsnmp_cache)
     ->  raw::c_int;

    pub fn netsnmp_cache_is_valid(arg1: *mut netsnmp_agent_request_info,
                                  name: *const  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_is_cache_valid(arg1: *mut netsnmp_agent_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_cache_handler(arg1:  raw::c_int,
                                     arg2: NetsnmpCacheLoad,
                                     arg3: NetsnmpCacheFree, arg4: *const oid,
                                     arg5:  raw::c_int)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_cache_handler(reginfo:
                                              *mut netsnmp_handler_registration,
                                          arg1:  raw::c_int,
                                          arg2: NetsnmpCacheLoad,
                                          arg3: NetsnmpCacheFree)
     ->  raw::c_int;

    pub fn netsnmp_cache_helper_handler(handler: *mut netsnmp_mib_handler,
                                        reginfo:
                                            *mut netsnmp_handler_registration,
                                        reqinfo:
                                            *mut netsnmp_agent_request_info,
                                        requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_cache_create(timeout:  raw::c_int,
                                load_hook: NetsnmpCacheLoad,
                                free_hook: NetsnmpCacheFree,
                                rootoid: *const oid,
                                rootoid_len:  raw::c_int)
     -> *mut netsnmp_cache;

    pub fn netsnmp_cache_remove(cache: *mut netsnmp_cache)
     ->  raw::c_int;

    pub fn netsnmp_cache_free(cache: *mut netsnmp_cache)
     ->  raw::c_int;

    pub fn netsnmp_cache_handler_get(cache: *mut netsnmp_cache)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_cache_handler_owns_cache(handler:
                                                *mut netsnmp_mib_handler);

    pub fn netsnmp_cache_find_by_oid(rootoid: *const oid,
                                     rootoid_len:  raw::c_int)
     -> *mut netsnmp_cache;

    pub fn netsnmp_cache_timer_start(cache: *mut netsnmp_cache)
     ->  raw::c_uint;

    pub fn netsnmp_cache_timer_stop(cache: *mut netsnmp_cache);
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_old_api_info_s {
    pub var: *mut variable,
    pub varsize: usize,
    pub numvars: usize,
    pub ss: *mut netsnmp_session,
    pub flags:  raw::c_int,
}

impl Clone for netsnmp_old_api_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_old_api_info = netsnmp_old_api_info_s;
#[repr(C)]
#[derive(Copy)]
pub struct old_opi_cache_s {
    pub data: *mut raw::c_uchar,
    pub write_method: WriteMethod,
}

impl Clone for old_opi_cache_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_old_api_cache = old_opi_cache_s;
extern "C" {
    pub fn netsnmp_register_old_api(moduleName: *const  raw::c_char,
                                    var: *const variable, varsize: usize,
                                    numvars: usize, mibloc: *const oid,
                                    mibloclen: usize,
                                    priority:  raw::c_int,
                                    range_subid:  raw::c_int,
                                    range_ubound: oid,
                                    ss: *mut netsnmp_session,
                                    context: *const  raw::c_char,
                                    timeout:  raw::c_int,
                                    flags:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_old_api_helper(handler: *mut netsnmp_mib_handler,
                                  reginfo: *mut netsnmp_handler_registration,
                                  reqinfo: *mut netsnmp_agent_request_info,
                                  requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_current_agent_session() -> *mut netsnmp_agent_session;

    pub fn netsnmp_get_read_only_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_init_read_only_helper();

    pub fn netsnmp_read_only_helper(handler: *mut netsnmp_mib_handler,
                                    reginfo:
                                        *mut netsnmp_handler_registration,
                                    reqinfo: *mut netsnmp_agent_request_info,
                                    requests: *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_row_merge_status_x {
    pub count:  raw::c_char,
    pub rows:  raw::c_char,
    pub current:  raw::c_char,
    pub reserved:  raw::c_char,
    pub saved_requests: *mut *mut netsnmp_request_info,
    pub saved_status: *mut  raw::c_char,
}

impl Clone for netsnmp_row_merge_status_x {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_row_merge_status = netsnmp_row_merge_status_x;
extern "C" {
    pub fn netsnmp_get_row_merge_handler(arg1:  raw::c_int)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_row_merge(reginfo:
                                          *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_init_row_merge();

    pub fn netsnmp_row_merge_status_first(reginfo:
                                              *mut netsnmp_handler_registration,
                                          reqinfo:
                                              *mut netsnmp_agent_request_info)
     ->  raw::c_int;

    pub fn netsnmp_row_merge_status_last(reginfo:
                                             *mut netsnmp_handler_registration,
                                         reqinfo:
                                             *mut netsnmp_agent_request_info)
     ->  raw::c_int;

    pub fn netsnmp_row_merge_helper_handler(handler: *mut netsnmp_mib_handler,
                                            reginfo:
                                                *mut netsnmp_handler_registration,
                                            reqinfo:
                                                *mut netsnmp_agent_request_info,
                                            requests:
                                                *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_serialize_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_serialize(reginfo:
                                          *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_init_serialize();

    pub fn netsnmp_serialize_helper_handler(handler: *mut netsnmp_mib_handler,
                                            reginfo:
                                                *mut netsnmp_handler_registration,
                                            reqinfo:
                                                *mut netsnmp_agent_request_info,
                                            requests:
                                                *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_get_bulk_to_next_handler() -> *mut netsnmp_mib_handler;

    pub fn netsnmp_init_bulk_to_next_helper();

    pub fn netsnmp_bulk_to_next_fix_requests(requests:
                                                 *mut netsnmp_request_info);

    pub fn netsnmp_bulk_to_next_helper(handler: *mut netsnmp_mib_handler,
                                       reginfo:
                                           *mut netsnmp_handler_registration,
                                       reqinfo:
                                           *mut netsnmp_agent_request_info,
                                       requests: *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_mode_handler_list_s {
    pub next: *mut netsnmp_mode_handler_list_s,
    pub mode:  raw::c_int,
    pub callback_handler: *mut netsnmp_mib_handler,
}

impl Clone for netsnmp_mode_handler_list_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_mode_handler_list = netsnmp_mode_handler_list_s;
extern "C" {
    pub fn netsnmp_get_mode_end_call_handler(endlist:
                                                 *mut netsnmp_mode_handler_list)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_mode_end_call_add_mode_callback(endlist:
                                                       *mut netsnmp_mode_handler_list,
                                                   mode:
                                                        raw::c_int,
                                                   callbackh:
                                                       *mut netsnmp_mib_handler)
     -> *mut netsnmp_mode_handler_list;

    pub fn netsnmp_mode_end_call_helper(handler: *mut netsnmp_mib_handler,
                                        reginfo:
                                            *mut netsnmp_handler_registration,
                                        reqinfo:
                                            *mut netsnmp_agent_request_info,
                                        requests: *mut netsnmp_request_info)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_column_info_t {
    pub isRange:  raw::c_char,
    pub list_count:  raw::c_char,
    pub details: netsnmp_column_info_t__bindgen_ty_1,
    pub next: *mut netsnmp_column_info_t,
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_column_info_t__bindgen_ty_1 {
    pub range: __BindgenUnionField<[ raw::c_uint; 2usize]>,
    pub list: __BindgenUnionField<*mut  raw::c_uint>,
    pub bindgen_union_field: u64,
}

impl Clone for netsnmp_column_info_t__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}

impl Clone for netsnmp_column_info_t {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_column_info = netsnmp_column_info_t;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_registration_info_s {
    pub indexes: *mut netsnmp_variable_list,
    pub number_indexes:  raw::c_uint,
    pub min_column:  raw::c_uint,
    pub max_column:  raw::c_uint,
    pub valid_columns: *mut netsnmp_column_info,
}
impl Clone for netsnmp_table_registration_info_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_registration_info = netsnmp_table_registration_info_s;
#[repr(C)]
pub struct netsnmp_table_request_info_s {
    pub colnum:  raw::c_uint,
    pub number_indexes:  raw::c_uint,
    pub indexes: *mut netsnmp_variable_list,
    pub index_oid: [oid; 128usize],
    pub index_oid_len: usize,
    pub reg_info: *mut netsnmp_table_registration_info,
}

pub type netsnmp_table_request_info = netsnmp_table_request_info_s;
extern "C" {
    pub fn netsnmp_get_table_handler(tabreq:
                                         *mut netsnmp_table_registration_info)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_handler_owns_table_info(handler: *mut netsnmp_mib_handler);

    pub fn netsnmp_registration_owns_table_info(reg:
                                                    *mut netsnmp_handler_registration);

    pub fn netsnmp_register_table(reginfo: *mut netsnmp_handler_registration,
                                  tabreq:
                                      *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_unregister_table(reginfo:
                                        *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_table_build_oid(reginfo: *mut netsnmp_handler_registration,
                                   reqinfo: *mut netsnmp_request_info,
                                   table_info:
                                       *mut netsnmp_table_request_info)
     ->  raw::c_int;

    pub fn netsnmp_table_build_oid_from_index(reginfo:
                                                  *mut netsnmp_handler_registration,
                                              reqinfo:
                                                  *mut netsnmp_request_info,
                                              table_info:
                                                  *mut netsnmp_table_request_info)
     ->  raw::c_int;

    pub fn netsnmp_table_build_result(reginfo:
                                          *mut netsnmp_handler_registration,
                                      reqinfo: *mut netsnmp_request_info,
                                      table_info:
                                          *mut netsnmp_table_request_info,
                                      type_: raw::c_uchar, result: *mut raw::c_uchar,
                                      result_len: usize)
     ->  raw::c_int;

    pub fn netsnmp_update_variable_list_from_index(arg1:
                                                       *mut netsnmp_table_request_info)
     ->  raw::c_int;

    pub fn netsnmp_update_indexes_from_variable_list(tri:
                                                         *mut netsnmp_table_request_info)
     ->  raw::c_int;

    pub fn netsnmp_find_table_registration_info(reginfo:
                                                    *mut netsnmp_handler_registration)
     -> *mut netsnmp_table_registration_info;

    pub fn netsnmp_table_registration_info_clone(tri:
                                                     *mut netsnmp_table_registration_info)
     -> *mut netsnmp_table_registration_info;

    pub fn netsnmp_table_registration_info_free(arg1:
                                                    *mut netsnmp_table_registration_info);

    pub fn netsnmp_table_index_find_next_row(c: *mut netsnmp_container,
                                             tblreq:
                                                 *mut netsnmp_table_request_info)
     -> *mut netsnmp_index;

    pub fn netsnmp_closest_column(current:  raw::c_uint,
                                  valid_columns: *mut netsnmp_column_info)
     ->  raw::c_uint;

    pub fn table_helper_handler(handler: *mut netsnmp_mib_handler,
                                reginfo: *mut netsnmp_handler_registration,
                                reqinfo: *mut netsnmp_agent_request_info,
                                requests: *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_table_helper_add_indexes(tinfo:
                                                *mut netsnmp_table_registration_info, ...);

    pub fn netsnmp_check_getnext_reply(request: *mut netsnmp_request_info,
                                       prefix: *mut oid, prefix_len: usize,
                                       newvar: *mut netsnmp_variable_list,
                                       outvar:
                                           *mut *mut netsnmp_variable_list)
     ->  raw::c_int;

    pub fn netsnmp_extract_table_info(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_table_request_info;

    pub fn netsnmp_table_get_or_create_row_stash(reqinfo:
                                                     *mut netsnmp_agent_request_info,
                                                 storage_name: *const raw::c_uchar)
     -> *mut *mut netsnmp_oid_stash_node;

    pub fn netsnmp_table_next_column(table_info:
                                         *mut netsnmp_table_request_info)
     ->  raw::c_uint;

    pub fn netsnmp_sparse_table_register(reginfo:
                                             *mut netsnmp_handler_registration,
                                         tabreq:
                                             *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_sparse_table_handler_get() -> *mut netsnmp_mib_handler;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_row_s {
    pub indexes: *mut netsnmp_variable_list,
    pub index_oid: *mut oid,
    pub index_oid_len: usize,
    pub data: *mut  raw::c_void,
    pub next: *mut netsnmp_table_row_s,
    pub prev: *mut netsnmp_table_row_s,
}

impl Clone for netsnmp_table_row_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_row = netsnmp_table_row_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_data_s {
    pub indexes_template: *mut netsnmp_variable_list,
    pub name: *mut  raw::c_char,
    pub flags:  raw::c_int,
    pub store_indexes:  raw::c_int,
    pub first_row: *mut netsnmp_table_row,
    pub last_row: *mut netsnmp_table_row,
}

impl Clone for netsnmp_table_data_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_data = netsnmp_table_data_s;
extern "C" {
    pub fn netsnmp_table_data_generate_index_oid(row: *mut netsnmp_table_row);

    pub fn netsnmp_create_table_data(name: *const  raw::c_char)
     -> *mut netsnmp_table_data;

    pub fn netsnmp_create_table_data_row() -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_clone_row(row: *mut netsnmp_table_row)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_delete_row(row: *mut netsnmp_table_row)
     -> *mut  raw::c_void;

    pub fn netsnmp_table_data_add_row(table: *mut netsnmp_table_data,
                                      row: *mut netsnmp_table_row)
     ->  raw::c_int;

    pub fn netsnmp_table_data_replace_row(table: *mut netsnmp_table_data,
                                          origrow: *mut netsnmp_table_row,
                                          newrow: *mut netsnmp_table_row);

    pub fn netsnmp_table_data_remove_row(table: *mut netsnmp_table_data,
                                         row: *mut netsnmp_table_row)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_remove_and_delete_row(table:
                                                        *mut netsnmp_table_data,
                                                    row:
                                                        *mut netsnmp_table_row)
     -> *mut  raw::c_void;

    pub fn netsnmp_table_data_delete_table(table: *mut netsnmp_table_data);

    pub fn netsnmp_get_table_data_handler(table: *mut netsnmp_table_data)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_table_data(reginfo:
                                           *mut netsnmp_handler_registration,
                                       table: *mut netsnmp_table_data,
                                       table_info:
                                           *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_register_read_only_table_data(reginfo:
                                                     *mut netsnmp_handler_registration,
                                                 table:
                                                     *mut netsnmp_table_data,
                                                 table_info:
                                                     *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_table_data_helper_handler(handler:
                                                 *mut netsnmp_mib_handler,
                                             reginfo:
                                                 *mut netsnmp_handler_registration,
                                             reqinfo:
                                                 *mut netsnmp_agent_request_info,
                                             requests:
                                                 *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_extract_table(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_table_data;

    pub fn netsnmp_extract_table_row(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_extract_table_row_data(arg1: *mut netsnmp_request_info)
     -> *mut  raw::c_void;

    pub fn netsnmp_insert_table_row(arg1: *mut netsnmp_request_info,
                                    arg2: *mut netsnmp_table_row);

    pub fn netsnmp_table_data_build_result(reginfo:
                                               *mut netsnmp_handler_registration,
                                           reqinfo:
                                               *mut netsnmp_agent_request_info,
                                           request: *mut netsnmp_request_info,
                                           row: *mut netsnmp_table_row,
                                           column:  raw::c_int,
                                           type_: raw::c_uchar,
                                           result_data: *mut raw::c_uchar,
                                           result_data_len: usize)
     ->  raw::c_int;

    pub fn netsnmp_table_data_get_first_row(table: *mut netsnmp_table_data)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_get_next_row(table: *mut netsnmp_table_data,
                                           row: *mut netsnmp_table_row)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_get(table: *mut netsnmp_table_data,
                                  indexes: *mut netsnmp_variable_list)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_get_from_oid(table: *mut netsnmp_table_data,
                                           searchfor: *mut oid,
                                           searchfor_len: usize)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_num_rows(table: *mut netsnmp_table_data)
     ->  raw::c_int;

    pub fn netsnmp_init_table_dataset();
}
pub type Netsnmp_Value_Change_Ok =
    ::std::option::Option<unsafe extern "C" fn(old_value:
                                                   *mut  raw::c_char,
                                               old_value_len: usize,
                                               new_value:
                                                   *mut  raw::c_char,
                                               new_value_len: usize,
                                               mydata:
                                                   *mut  raw::c_void)
                              ->  raw::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_data_set_storage_s {
    pub column:  raw::c_uint,
    pub writable:  raw::c_char,
    pub change_ok_fn: Netsnmp_Value_Change_Ok,
    pub my_change_data: *mut  raw::c_void,
    pub type_: raw::c_uchar,
    pub data: netsnmp_table_data_set_storage_s__bindgen_ty_1,
    pub data_len: raw::c_ulong,
    pub next: *mut netsnmp_table_data_set_storage_s,
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_data_set_storage_s__bindgen_ty_1 {
    pub voidp: __BindgenUnionField<*mut  raw::c_void>,
    pub integer: __BindgenUnionField<*mut  raw::c_long>,
    pub string: __BindgenUnionField<*mut raw::c_uchar>,
    pub objid: __BindgenUnionField<*mut oid>,
    pub bitstring: __BindgenUnionField<*mut raw::c_uchar>,
    pub counter64: __BindgenUnionField<Struct_counter64>,
    pub floatVal: __BindgenUnionField<*mut f32>,
    pub doubleVal: __BindgenUnionField<*mut f64>,
    pub bindgen_union_field: u64,
}

impl Clone for netsnmp_table_data_set_storage_s__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}

impl Clone for netsnmp_table_data_set_storage_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_data_set_storage = netsnmp_table_data_set_storage_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_data_set_s {
    pub table: *mut netsnmp_table_data,
    pub default_row: *mut netsnmp_table_data_set_storage,
    pub allow_creation:  raw::c_int,
    pub rowstatus_column:  raw::c_uint,
}

impl Clone for netsnmp_table_data_set_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_data_set = netsnmp_table_data_set_s;
extern "C" {
    pub fn netsnmp_create_table_data_set(arg1: *const  raw::c_char)
     -> *mut netsnmp_table_data_set;

    pub fn netsnmp_table_data_set_clone_row(row: *mut netsnmp_table_row)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_dataset_delete_all_data(data:
                                                     *mut netsnmp_table_data_set_storage);

    pub fn netsnmp_table_dataset_delete_row(row: *mut netsnmp_table_row);

    pub fn netsnmp_table_dataset_add_row(table: *mut netsnmp_table_data_set,
                                         row: *mut netsnmp_table_row);

    pub fn netsnmp_table_dataset_replace_row(table:
                                                 *mut netsnmp_table_data_set,
                                             origrow: *mut netsnmp_table_row,
                                             newrow: *mut netsnmp_table_row);

    pub fn netsnmp_table_dataset_remove_row(table:
                                                *mut netsnmp_table_data_set,
                                            row: *mut netsnmp_table_row);

    pub fn netsnmp_table_dataset_remove_and_delete_row(table:
                                                           *mut netsnmp_table_data_set,
                                                       row:
                                                           *mut netsnmp_table_row);

    pub fn netsnmp_delete_table_data_set(table_set:
                                             *mut netsnmp_table_data_set);

    pub fn netsnmp_table_set_add_default_row(arg1:
                                                 *mut netsnmp_table_data_set,
                                             arg2:  raw::c_uint,
                                             arg3:  raw::c_int,
                                             arg4:  raw::c_int,
                                             default_value:
                                                 *mut  raw::c_void,
                                             default_value_len: usize)
     ->  raw::c_int;

    pub fn netsnmp_table_set_multi_add_default_row(arg1:
                                                       *mut netsnmp_table_data_set, ...);

    pub fn netsnmp_get_table_data_set_handler(arg1:
                                                  *mut netsnmp_table_data_set)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_table_data_set_helper_handler(handler:
                                                     *mut netsnmp_mib_handler,
                                                 reginfo:
                                                     *mut netsnmp_handler_registration,
                                                 reqinfo:
                                                     *mut netsnmp_agent_request_info,
                                                 requests:
                                                     *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_register_table_data_set(arg1:
                                               *mut netsnmp_handler_registration,
                                           arg2: *mut netsnmp_table_data_set,
                                           arg3:
                                               *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_extract_table_data_set(request: *mut netsnmp_request_info)
     -> *mut netsnmp_table_data_set;

    pub fn netsnmp_extract_table_data_set_column(arg1:
                                                     *mut netsnmp_request_info,
                                                 arg2:  raw::c_uint)
     -> *mut netsnmp_table_data_set_storage;

    pub fn netsnmp_table_dataset_get_or_create_stash(ari:
                                                         *mut netsnmp_agent_request_info,
                                                     tds:
                                                         *mut netsnmp_table_data_set,
                                                     tri:
                                                         *mut netsnmp_table_request_info)
     -> *mut *mut netsnmp_oid_stash_node;

    pub fn netsnmp_table_dataset_get_newrow(request:
                                                *mut netsnmp_request_info,
                                            reqinfo:
                                                *mut netsnmp_agent_request_info,
                                            rootoid_len:
                                                 raw::c_int,
                                            datatable:
                                                *mut netsnmp_table_data_set,
                                            table_info:
                                                *mut netsnmp_table_request_info)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_register_auto_data_table(table_set:
                                                *mut netsnmp_table_data_set,
                                            registration_name:
                                                *mut  raw::c_char);

    pub fn netsnmp_unregister_auto_data_table(table_set:
                                                  *mut netsnmp_table_data_set,
                                              registration_name:
                                                  *mut  raw::c_char);

    pub fn netsnmp_config_parse_table_set(token:
                                              *const  raw::c_char,
                                          line: *mut  raw::c_char);

    pub fn netsnmp_config_parse_add_row(token: *const  raw::c_char,
                                        line: *mut  raw::c_char);

    pub fn netsnmp_table_data_set_get_first_row(table:
                                                    *mut netsnmp_table_data_set)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_data_set_get_next_row(table:
                                                   *mut netsnmp_table_data_set,
                                               row: *mut netsnmp_table_row)
     -> *mut netsnmp_table_row;

    pub fn netsnmp_table_set_num_rows(table: *mut netsnmp_table_data_set)
     ->  raw::c_int;

    pub fn netsnmp_table_data_set_find_column(arg1:
                                                  *mut netsnmp_table_data_set_storage,
                                              arg2:  raw::c_uint)
     -> *mut netsnmp_table_data_set_storage;

    pub fn netsnmp_mark_row_column_writable(row: *mut netsnmp_table_row,
                                            column:  raw::c_int,
                                            writable:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_set_row_column(arg1: *mut netsnmp_table_row,
                                  arg2:  raw::c_uint,
                                  arg3:  raw::c_int,
                                  arg4: *const  raw::c_void,
                                  arg5: usize) ->  raw::c_int;

    pub fn netsnmp_table_dataset_add_index(table: *mut netsnmp_table_data_set,
                                           type_: raw::c_uchar);

    pub fn netsnmp_table_set_add_indexes(tset:
                                             *mut netsnmp_table_data_set, ...);
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_tdata_row_s {
    pub oid_index: netsnmp_index,
    pub indexes: *mut netsnmp_variable_list,
    pub data: *mut  raw::c_void,
}

impl Clone for netsnmp_tdata_row_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_tdata_row = netsnmp_tdata_row_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_tdata_s {
    pub indexes_template: *mut netsnmp_variable_list,
    pub name: *mut  raw::c_char,
    pub flags:  raw::c_int,
    pub container: *mut netsnmp_container,
}

impl Clone for netsnmp_tdata_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_tdata = netsnmp_tdata_s;
pub type netsnmp_table_data2row = netsnmp_tdata_row_s;
pub type netsnmp_table_data2 = netsnmp_tdata_s;
extern "C" {
    pub fn netsnmp_tdata_create_table(name: *const  raw::c_char,
                                      flags:  raw::c_long)
     -> *mut netsnmp_tdata;

    pub fn netsnmp_tdata_delete_table(table: *mut netsnmp_tdata);

    pub fn netsnmp_tdata_create_row() -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_clone_row(row: *mut netsnmp_tdata_row)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_copy_row(dst_row: *mut netsnmp_tdata_row,
                                  src_row: *mut netsnmp_tdata_row)
     ->  raw::c_int;

    pub fn netsnmp_tdata_delete_row(row: *mut netsnmp_tdata_row)
     -> *mut  raw::c_void;

    pub fn netsnmp_tdata_add_row(table: *mut netsnmp_tdata,
                                 row: *mut netsnmp_tdata_row)
     ->  raw::c_int;

    pub fn netsnmp_tdata_replace_row(table: *mut netsnmp_tdata,
                                     origrow: *mut netsnmp_tdata_row,
                                     newrow: *mut netsnmp_tdata_row);

    pub fn netsnmp_tdata_remove_row(table: *mut netsnmp_tdata,
                                    row: *mut netsnmp_tdata_row)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_remove_and_delete_row(table: *mut netsnmp_tdata,
                                               row: *mut netsnmp_tdata_row)
     -> *mut  raw::c_void;

    pub fn netsnmp_get_tdata_handler(table: *mut netsnmp_tdata)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_tdata_register(reginfo: *mut netsnmp_handler_registration,
                                  table: *mut netsnmp_tdata,
                                  table_info:
                                      *mut netsnmp_table_registration_info)
     ->  raw::c_int;

    pub fn netsnmp_tdata_unregister(reginfo:
                                        *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_tdata_extract_table(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_tdata;

    pub fn netsnmp_tdata_extract_container(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_container;

    pub fn netsnmp_tdata_extract_row(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_extract_entry(arg1: *mut netsnmp_request_info)
     -> *mut  raw::c_void;

    pub fn netsnmp_insert_tdata_row(arg1: *mut netsnmp_request_info,
                                    arg2: *mut netsnmp_tdata_row);

    pub fn netsnmp_remove_tdata_row(arg1: *mut netsnmp_request_info,
                                    arg2: *mut netsnmp_tdata_row);

    pub fn netsnmp_tdata_row_entry(row: *mut netsnmp_tdata_row)
     -> *mut  raw::c_void;

    pub fn netsnmp_tdata_row_first(table: *mut netsnmp_tdata)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_get(table: *mut netsnmp_tdata,
                                 row: *mut netsnmp_tdata_row)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_next(table: *mut netsnmp_tdata,
                                  row: *mut netsnmp_tdata_row)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_get_byidx(table: *mut netsnmp_tdata,
                                       indexes: *mut netsnmp_variable_list)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_get_byoid(table: *mut netsnmp_tdata,
                                       searchfor: *mut oid,
                                       searchfor_len: usize)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_next_byidx(table: *mut netsnmp_tdata,
                                        indexes: *mut netsnmp_variable_list)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_next_byoid(table: *mut netsnmp_tdata,
                                        searchfor: *mut oid,
                                        searchfor_len: usize)
     -> *mut netsnmp_tdata_row;

    pub fn netsnmp_tdata_row_count(table: *mut netsnmp_tdata)
     ->  raw::c_int;

    pub fn netsnmp_tdata_compare_idx(row: *mut netsnmp_tdata_row,
                                     indexes: *mut netsnmp_variable_list)
     ->  raw::c_int;

    pub fn netsnmp_tdata_compare_oid(row: *mut netsnmp_tdata_row,
                                     compareto: *mut oid,
                                     compareto_len: usize)
     ->  raw::c_int;

    pub fn netsnmp_tdata_compare_subtree_idx(row: *mut netsnmp_tdata_row,
                                             indexes:
                                                 *mut netsnmp_variable_list)
     ->  raw::c_int;

    pub fn netsnmp_tdata_compare_subtree_oid(row: *mut netsnmp_tdata_row,
                                             compareto: *mut oid,
                                             compareto_len: usize)
     ->  raw::c_int;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_iterator_info_s {
    pub refcnt:  raw::c_int,
    pub get_first_data_point: Netsnmp_First_Data_Point,
    pub get_next_data_point: Netsnmp_Next_Data_Point,
    pub make_data_context: Netsnmp_Make_Data_Context,
    pub free_loop_context: Netsnmp_Free_Loop_Context,
    pub free_data_context: Netsnmp_Free_Data_Context,
    pub free_loop_context_at_end: Netsnmp_Free_Loop_Context,
    pub myvoid: *mut  raw::c_void,
    pub flags:  raw::c_int,
    pub table_reginfo: *mut netsnmp_table_registration_info,
    pub get_row_indexes: Netsnmp_First_Data_Point,
    pub indexes: *mut netsnmp_variable_list,
}

impl Clone for netsnmp_iterator_info_s {
    fn clone(&self) -> Self { *self }
}
pub type Netsnmp_First_Data_Point =
    ::std::option::Option<unsafe extern "C" fn(loop_context:
                                                   *mut *mut  raw::c_void,
                                               data_context:
                                                   *mut *mut  raw::c_void,
                                               arg1:
                                                   *mut netsnmp_variable_list,
                                               arg2:
                                                   *mut netsnmp_iterator_info_s)
                              -> *mut netsnmp_variable_list>;
pub type Netsnmp_Next_Data_Point =
    ::std::option::Option<unsafe extern "C" fn(loop_context:
                                                   *mut *mut  raw::c_void,
                                               data_context:
                                                   *mut *mut  raw::c_void,
                                               arg1:
                                                   *mut netsnmp_variable_list,
                                               arg2:
                                                   *mut netsnmp_iterator_info_s)
                              -> *mut netsnmp_variable_list>;
pub type Netsnmp_Make_Data_Context =
    ::std::option::Option<unsafe extern "C" fn(loop_context:
                                                   *mut  raw::c_void,
                                               arg1:
                                                   *mut netsnmp_iterator_info_s)
                              -> *mut  raw::c_void>;
pub type Netsnmp_Free_Loop_Context =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut  raw::c_void,
                                               arg2:
                                                   *mut netsnmp_iterator_info_s)>;
pub type Netsnmp_Free_Data_Context =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut  raw::c_void,
                                               arg2:
                                                   *mut netsnmp_iterator_info_s)>;
pub type netsnmp_iterator_info = netsnmp_iterator_info_s;
extern "C" {
    pub fn netsnmp_handler_owns_iterator_info(h: *mut netsnmp_mib_handler);

    pub fn netsnmp_get_table_iterator_handler(iinfo:
                                                  *mut netsnmp_iterator_info)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_register_table_iterator(reginfo:
                                               *mut netsnmp_handler_registration,
                                           iinfo: *mut netsnmp_iterator_info)
     ->  raw::c_int;

    pub fn netsnmp_iterator_delete_table(iinfo: *mut netsnmp_iterator_info);

    pub fn netsnmp_extract_iterator_context(arg1: *mut netsnmp_request_info)
     -> *mut  raw::c_void;

    pub fn netsnmp_insert_iterator_context(arg1: *mut netsnmp_request_info,
                                           arg2: *mut  raw::c_void);

    pub fn netsnmp_table_iterator_helper_handler(handler:
                                                     *mut netsnmp_mib_handler,
                                                 reginfo:
                                                     *mut netsnmp_handler_registration,
                                                 reqinfo:
                                                     *mut netsnmp_agent_request_info,
                                                 requests:
                                                     *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_iterator_row_first(arg1: *mut netsnmp_iterator_info)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_get(arg1: *mut netsnmp_iterator_info,
                                    arg2: *mut  raw::c_void)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_next(arg1: *mut netsnmp_iterator_info,
                                     arg2: *mut  raw::c_void)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_get_byidx(arg1: *mut netsnmp_iterator_info,
                                          arg2: *mut netsnmp_variable_list)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_next_byidx(arg1: *mut netsnmp_iterator_info,
                                           arg2: *mut netsnmp_variable_list)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_get_byoid(arg1: *mut netsnmp_iterator_info,
                                          arg2: *mut oid, arg3: usize)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_next_byoid(arg1: *mut netsnmp_iterator_info,
                                           arg2: *mut oid, arg3: usize)
     -> *mut  raw::c_void;

    pub fn netsnmp_iterator_row_count(arg1: *mut netsnmp_iterator_info)
     ->  raw::c_int;

    pub fn netsnmp_container_table_handler_get(tabreq:
                                                   *mut netsnmp_table_registration_info,
                                               container:
                                                   *mut netsnmp_container,
                                               key_type:
                                                    raw::c_char)
     -> *mut netsnmp_mib_handler;

    pub fn netsnmp_container_table_register(reginfo:
                                                *mut netsnmp_handler_registration,
                                            tabreq:
                                                *mut netsnmp_table_registration_info,
                                            container: *mut netsnmp_container,
                                            key_type:  raw::c_char)
     ->  raw::c_int;

    pub fn netsnmp_container_table_unregister(reginfo:
                                                  *mut netsnmp_handler_registration)
     ->  raw::c_int;

    pub fn netsnmp_container_table_container_extract(request:
                                                         *mut netsnmp_request_info)
     -> *mut netsnmp_container;

    pub fn netsnmp_container_table_row_insert(request:
                                                  *mut netsnmp_request_info,
                                              row: *mut netsnmp_index);

    pub fn netsnmp_container_table_row_remove(request:
                                                  *mut netsnmp_request_info,
                                              row: *mut netsnmp_index);

    pub fn netsnmp_container_table_find_next_row(request:
                                                     *mut netsnmp_request_info,
                                                 tblreq:
                                                     *mut netsnmp_table_request_info,
                                                 container:
                                                     *mut netsnmp_container,
                                                 key_type:
                                                      raw::c_char)
     -> *mut  raw::c_void;
}
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_request_group_item_s {
    pub ri: *mut netsnmp_request_info,
    pub tri: *mut netsnmp_table_request_info,
    pub next: *mut netsnmp_request_group_item_s,
}

impl Clone for netsnmp_request_group_item_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_request_group_item = netsnmp_request_group_item_s;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_request_group_s {
    pub index: netsnmp_index,
    pub table: *mut netsnmp_container,
    pub existing_row: *mut netsnmp_index,
    pub undo_info: *mut netsnmp_index,
    pub row_created:  raw::c_char,
    pub row_deleted:  raw::c_char,
    pub fill1:  raw::c_char,
    pub fill2:  raw::c_char,
    pub list: *mut netsnmp_request_group_item,
    pub status:  raw::c_int,
    pub rg_void: *mut  raw::c_void,
}

impl Clone for netsnmp_request_group_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_request_group = netsnmp_request_group_s;
pub type Netsnmp_User_Row_Operation_c =
    ::std::option::Option<unsafe extern "C" fn(lhs:
                                                   *const  raw::c_void,
                                               rhs:
                                                   *const  raw::c_void)
                              ->  raw::c_int>;
pub type Netsnmp_User_Row_Operation =
    ::std::option::Option<unsafe extern "C" fn(lhs:
                                                   *mut  raw::c_void,
                                               rhs:
                                                   *mut  raw::c_void)
                              ->  raw::c_int>;
pub type Netsnmp_User_Get_Processor =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut netsnmp_request_info,
                                               arg2: *mut netsnmp_index,
                                               arg3:
                                                   *mut netsnmp_table_request_info)
                              ->  raw::c_int>;
pub type UserRowMethod =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut netsnmp_index)
                              -> *mut netsnmp_index>;
pub type Netsnmp_User_Row_Action =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut netsnmp_index,
                                               arg2: *mut netsnmp_index,
                                               arg3:
                                                   *mut netsnmp_request_group)
                              ->  raw::c_int>;
pub type Netsnmp_User_Group_Method =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut netsnmp_request_group)>;
#[repr(C)]
#[derive(Copy)]
pub struct netsnmp_table_array_callbacks_s {
    pub row_copy: Netsnmp_User_Row_Operation,
    pub row_compare: Netsnmp_User_Row_Operation_c,
    pub get_value: Netsnmp_User_Get_Processor,
    pub can_activate: Netsnmp_User_Row_Action,
    pub activated: Netsnmp_User_Row_Action,
    pub can_deactivate: Netsnmp_User_Row_Action,
    pub deactivated: Netsnmp_User_Row_Action,
    pub can_delete: Netsnmp_User_Row_Action,
    pub create_row: UserRowMethod,
    pub duplicate_row: UserRowMethod,
    pub delete_row: UserRowMethod,
    pub set_reserve1: Netsnmp_User_Group_Method,
    pub set_reserve2: Netsnmp_User_Group_Method,
    pub set_action: Netsnmp_User_Group_Method,
    pub set_commit: Netsnmp_User_Group_Method,
    pub set_free: Netsnmp_User_Group_Method,
    pub set_undo: Netsnmp_User_Group_Method,
    pub container: *mut netsnmp_container,
    pub can_set:  raw::c_char,
}

impl Clone for netsnmp_table_array_callbacks_s {
    fn clone(&self) -> Self { *self }
}
pub type netsnmp_table_array_callbacks = netsnmp_table_array_callbacks_s;
extern "C" {
    pub fn netsnmp_table_container_register(reginfo:
                                                *mut netsnmp_handler_registration,
                                            tabreq:
                                                *mut netsnmp_table_registration_info,
                                            cb:
                                                *mut netsnmp_table_array_callbacks,
                                            container: *mut netsnmp_container,
                                            group_rows:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_table_array_register(reginfo:
                                            *mut netsnmp_handler_registration,
                                        tabreq:
                                            *mut netsnmp_table_registration_info,
                                        cb:
                                            *mut netsnmp_table_array_callbacks,
                                        container: *mut netsnmp_container,
                                        group_rows:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_extract_array_context(arg1: *mut netsnmp_request_info)
     -> *mut netsnmp_container;

    pub fn netsnmp_table_array_helper_handler(handler:
                                                  *mut netsnmp_mib_handler,
                                              reginfo:
                                                  *mut netsnmp_handler_registration,
                                              reqinfo:
                                                  *mut netsnmp_agent_request_info,
                                              requests:
                                                  *mut netsnmp_request_info)
     ->  raw::c_int;

    pub fn netsnmp_table_array_check_row_status(cb:
                                                    *mut netsnmp_table_array_callbacks,
                                                ag:
                                                    *mut netsnmp_request_group,
                                                rs_new:
                                                    *mut  raw::c_long,
                                                rs_old:
                                                    *mut  raw::c_long)
     ->  raw::c_int;

    pub fn netsnmp_register_statistic_handler(reginfo:
                                                  *mut netsnmp_handler_registration,
                                              start: oid,
                                              begin:  raw::c_int,
                                              end:  raw::c_int)
     ->  raw::c_int;

    pub fn netsnmp_init_helpers();
}
#[repr(C)]
#[derive(Copy)]
pub struct __locale_data {
    pub _address: u8,
}
impl Clone for __locale_data {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct __va_list_tag {
    pub gp_offset:  raw::c_uint,
    pub fp_offset:  raw::c_uint,
    pub overflow_arg_area: *mut  raw::c_void,
    pub reg_save_area: *mut  raw::c_void,
}

impl Clone for __va_list_tag {
    fn clone(&self) -> Self { *self }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_at {
    pub _address: u8,
}
impl Clone for sockaddr_at {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_ax25 {
    pub _address: u8,
}
impl Clone for sockaddr_ax25 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_dl {
    pub _address: u8,
}
impl Clone for sockaddr_dl {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_eon {
    pub _address: u8,
}
impl Clone for sockaddr_eon {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_inarp {
    pub _address: u8,
}
impl Clone for sockaddr_inarp {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_ipx {
    pub _address: u8,
}
impl Clone for sockaddr_ipx {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_iso {
    pub _address: u8,
}
impl Clone for sockaddr_iso {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_ns {
    pub _address: u8,
}
impl Clone for sockaddr_ns {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_x25 {
    pub _address: u8,
}
impl Clone for sockaddr_x25 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct snmp_internal_session {
    pub _address: u8,
}
impl Clone for snmp_internal_session {
    fn clone(&self) -> Self { *self }
}
