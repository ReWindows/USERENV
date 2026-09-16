//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const type_info::`vftable'", rva: 0x1F058, first_slot: 0, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const std::exception::`vftable'", rva: 0x1F090, first_slot: 1, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const wil::ResultException::`vftable'", rva: 0x1F0A0, first_slot: 3, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const ProfileAPILogging::`vftable'", rva: 0x1F0B0, first_slot: 5, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const UserenvTelemetry::WINAPIDeleteProfile::`vftable'", rva: 0x1F0D0, first_slot: 9, slot_count: 3, confidence: "likely" },
    VTableInfo { name: "const wil::ActivityBase<class ProfileAPILogging, 1, 0, 5, 16777216, struct _TlgReflectorTag_Param0IsProviderType>::`vftable'", rva: 0x1F0E8, first_slot: 12, slot_count: 3, confidence: "likely" },
    VTableInfo { name: "const wil::TraceLoggingProvider::`vftable'", rva: 0x1F100, first_slot: 15, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const UserenvTelemetry::WINAPICreateProfile::`vftable'", rva: 0x1F120, first_slot: 19, slot_count: 3, confidence: "likely" },
    VTableInfo { name: "const UserenvTelemetry::WINAPIRemapProfile::`vftable'", rva: 0x1F138, first_slot: 22, slot_count: 3, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::`vftable'", rva: 0x1F150, first_slot: 25, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__base<bool __cdecl (void *, uint64_t, void *, uint64_t, unsigned int)>::`vftable'", rva: 0x1F178, first_slot: 30, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const UserenvTelemetry::DeleteUserProfileDirectory::`vftable'", rva: 0x1F1A0, first_slot: 35, slot_count: 3, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0x1F058, slot: 0, byte_offset: 0, target_rva: 0xE600, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F090, slot: 0, byte_offset: 0, target_rva: 0x10040, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F090, slot: 1, byte_offset: 8, target_rva: 0x10CC0, target_id: Some("?what@exception@std@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl std::exception::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0A0, slot: 0, byte_offset: 0, target_rva: 0x10000, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0A0, slot: 1, byte_offset: 8, target_rva: 0x10B60, target_id: Some("?what@ResultException@wil@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl wil::ResultException::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0B0, slot: 0, byte_offset: 0, target_rva: 0x12D50, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0B0, slot: 1, byte_offset: 8, target_rva: 0x12BC0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0B0, slot: 2, byte_offset: 16, target_rva: 0x9100, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0B0, slot: 3, byte_offset: 24, target_rva: 0x12210, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0D0, slot: 0, byte_offset: 0, target_rva: 0x99D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0D0, slot: 1, byte_offset: 8, target_rva: 0x13910, target_id: Some("?StopActivity@WINAPIDeleteProfile@UserenvTelemetry@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl UserenvTelemetry::WINAPIDeleteProfile::StopActivity(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0D0, slot: 2, byte_offset: 16, target_rva: 0x13FE0, target_id: Some("?WasAlreadyReportedToTelemetry@WINAPIDeleteProfile@UserenvTelemetry@@MEAA_NJ@Z"), target_name: Some("protected: virtual bool __cdecl UserenvTelemetry::WINAPIDeleteProfile::WasAlreadyReportedToTelemetry(long)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E8, slot: 0, byte_offset: 0, target_rva: 0x99D0, target_id: Some("?NotifyFailure@?$ActivityBase@VProfileAPILogging@@$00$0EAAAAAAAAAAA@$04$0BAAAAAA@U_TlgReflectorTag_Param0IsProviderType@@@wil@@UEAA_NAEBUFailureInfo@2@@Z"), target_name: Some("public: virtual bool __cdecl wil::ActivityBase<class ProfileAPILogging, 1, 0, 5, 16777216, struct _TlgReflectorTag_Param0IsProviderType>::NotifyFailure(struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E8, slot: 1, byte_offset: 8, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E8, slot: 2, byte_offset: 16, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F100, slot: 0, byte_offset: 0, target_rva: 0x12D50, target_id: Some("?NotifyFailure@TraceLoggingProvider@wil@@EEAA_NAEBUFailureInfo@2@@Z"), target_name: Some("private: virtual bool __cdecl wil::TraceLoggingProvider::NotifyFailure(struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F100, slot: 1, byte_offset: 8, target_rva: 0x12BC0, target_id: Some("?Initialize@TraceLoggingProvider@wil@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::Initialize(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F100, slot: 2, byte_offset: 16, target_rva: 0x9100, target_id: Some("?OnErrorReported@TraceLoggingProvider@wil@@MEAAX_NAEBUFailureInfo@2@@Z"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::OnErrorReported(bool, struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F100, slot: 3, byte_offset: 24, target_rva: 0x12250, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F120, slot: 0, byte_offset: 0, target_rva: 0x99D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F120, slot: 1, byte_offset: 8, target_rva: 0x136E0, target_id: Some("?StopActivity@WINAPICreateProfile@UserenvTelemetry@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl UserenvTelemetry::WINAPICreateProfile::StopActivity(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F120, slot: 2, byte_offset: 16, target_rva: 0x13FE0, target_id: Some("?WasAlreadyReportedToTelemetry@WINAPICreateProfile@UserenvTelemetry@@MEAA_NJ@Z"), target_name: Some("protected: virtual bool __cdecl UserenvTelemetry::WINAPICreateProfile::WasAlreadyReportedToTelemetry(long)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F138, slot: 0, byte_offset: 0, target_rva: 0x99D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F138, slot: 1, byte_offset: 8, target_rva: 0x13B40, target_id: Some("?StopActivity@WINAPIRemapProfile@UserenvTelemetry@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl UserenvTelemetry::WINAPIRemapProfile::StopActivity(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F138, slot: 2, byte_offset: 16, target_rva: 0x13FE0, target_id: Some("?WasAlreadyReportedToTelemetry@WINAPIRemapProfile@UserenvTelemetry@@MEAA_NJ@Z"), target_name: Some("protected: virtual bool __cdecl UserenvTelemetry::WINAPIRemapProfile::WasAlreadyReportedToTelemetry(long)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F150, slot: 0, byte_offset: 0, target_rva: 0x156A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F150, slot: 1, byte_offset: 8, target_rva: 0x190A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F150, slot: 2, byte_offset: 16, target_rva: 0x190A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F150, slot: 3, byte_offset: 24, target_rva: 0x12BC0, target_id: Some("?destroy@?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAAXXZ"), target_name: Some("public: virtual void __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::destroy(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F150, slot: 4, byte_offset: 32, target_rva: 0x155F0, target_id: Some("??R?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAA_N$$QEAPEAX$$QEA_K01$$QEAI@Z"), target_name: Some("public: virtual bool __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::operator()(void * &&, uint64_t &&, void * &&, uint64_t &&, unsigned int &&)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F178, slot: 0, byte_offset: 0, target_rva: 0x15660, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F178, slot: 1, byte_offset: 8, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F178, slot: 2, byte_offset: 16, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F178, slot: 3, byte_offset: 24, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F178, slot: 4, byte_offset: 32, target_rva: 0xF190, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F1A0, slot: 0, byte_offset: 0, target_rva: 0x99D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F1A0, slot: 1, byte_offset: 8, target_rva: 0x1ACE0, target_id: Some("?StopActivity@DeleteUserProfileDirectory@UserenvTelemetry@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl UserenvTelemetry::DeleteUserProfileDirectory::StopActivity(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F1A0, slot: 2, byte_offset: 16, target_rva: 0x13FE0, target_id: Some("?WasAlreadyReportedToTelemetry@DeleteUserProfileDirectory@UserenvTelemetry@@MEAA_NJ@Z"), target_name: Some("protected: virtual bool __cdecl UserenvTelemetry::DeleteUserProfileDirectory::WasAlreadyReportedToTelemetry(long)"), ambiguous: false, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
