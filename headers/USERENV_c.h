// Flat C exports observed in USERENV.dll. Unknown ABIs are intentionally not declared.
// Validated dialects: ISO C17/C23/C26 draft and Windissect C20 (C ABI under C++20).
#pragma once
#ifndef USERENV_C_H
#define USERENV_C_H
#if defined(__cplusplus)
#  if __cplusplus < 202002L
#    define WINDISSECT_C20_PROFILE 0
#  else
#    define WINDISSECT_C20_PROFILE 1
#  endif
extern "C" {
#else
#  if defined(__STDC_VERSION__) && __STDC_VERSION__ < 201710L
#    error "Windissect C output requires C17 or newer"
#  endif
#  define WINDISSECT_C20_PROFILE 0
#  define WINDISSECT_C_STANDARD __STDC_VERSION__
#endif

// Export: LoadProfileExtender (ABI unverified)
// Export: GetUserProfileDirectoryA (ABI unverified)
// Export: GetUserProfileDirectoryW (ABI unverified)
// Export: UnloadUserProfile (ABI unverified)
// Export: LoadUserProfileW (ABI unverified)
// Export: ExpandEnvironmentStringsForUserA (ABI unverified)
// Export: LoadUserProfileA (ABI unverified)
// Export: ExpandEnvironmentStringsForUserW (ABI unverified)
// Export: GetAppContainerFolderPath (ABI unverified)
// Export: CreateEnvironmentBlock (ABI unverified)
// Export: DestroyEnvironmentBlock (ABI unverified)
// Export: GetAppContainerRegistryLocation (ABI unverified)
// Export: DeriveAppContainerSidFromAppContainerName (ABI unverified)
// Export: #ordinal211 (ABI unverified)
// Export: LookupAppContainerDisplayName (ABI unverified)
// Export: GetAllUsersProfileDirectoryW (ABI unverified)
// Export: CreateAppContainerProfile (ABI unverified)
// Export: #ordinal212 (ABI unverified)
// Export: CreateAppContainerProfileInternal (ABI unverified)
// Export: RegisterGPNotification (ABI unverified)
// Export: UnregisterGPNotification (ABI unverified)
// Export: #ordinal210 (ABI unverified)
// Export: UpdateAppContainerProfile (ABI unverified)
// Export: #ordinal218 (ABI unverified)
// Export: IsAppContainerProfilePresentInternal (ABI unverified)
// Export: GetProfilesDirectoryW (ABI unverified)
// Export: GetAppliedGPOListA (ABI unverified)
// Export: #ordinal217 (ABI unverified)
// Export: GetUserProfileDirectoryForUserSidW (ABI unverified)
// Export: ProcessGroupPolicyCompletedEx (ABI unverified)
// Export: #ordinal213 (ABI unverified)
// Export: DeleteAppContainerProfileInternal (ABI unverified)
// Export: EnterCriticalPolicySection (ABI unverified)
// Export: GetDefaultUserProfileDirectoryW (ABI unverified)
// Export: DeleteAppContainerProfile (ABI unverified)
// Export: FreeGPOListA (ABI unverified)
// Export: LeaveCriticalPolicySection (ABI unverified)
// Export: DeleteProfileW (ABI unverified)
// Export: RsopSetPolicySettingStatus (ABI unverified)
// Export: RsopResetPolicySettingStatus (ABI unverified)
// Export: UnloadProfileExtender (ABI unverified)
// Export: GetNextFgPolicyRefreshInfo (ABI unverified)
// Export: GetProfileType (ABI unverified)
// Export: RsopLoggingEnabled (ABI unverified)
// Export: AreThereVisibleLogoffScripts (ABI unverified)
// Export: AreThereVisibleShutdownScripts (ABI unverified)
// Export: ForceSyncFgPolicy (ABI unverified)
// Export: FreeGPOListW (ABI unverified)
// Export: GenerateGPNotification (ABI unverified)
// Export: GetAppliedGPOListW (ABI unverified)
// Export: GetGPOListA (ABI unverified)
// Export: GetGPOListW (ABI unverified)
// Export: GetPreviousFgPolicyRefreshInfo (ABI unverified)
// Export: HasPolicyForegroundProcessingCompleted (ABI unverified)
// Export: RefreshPolicy (ABI unverified)
// Export: RefreshPolicyEx (ABI unverified)
// Export: WaitForMachinePolicyForegroundProcessing (ABI unverified)
// Export: WaitForUserPolicyForegroundProcessing (ABI unverified)
// Export: #ordinal206 (ABI unverified)
// Export: CreateDirectoryJunctionsForSystem (ABI unverified)
// Export: #ordinal207 (ABI unverified)
// Export: CreateDirectoryJunctionsForUserProfile (ABI unverified)
// Export: #ordinal137 (ABI unverified)
// Export: CreateGroupEx (ABI unverified)
// Export: #ordinal139 (ABI unverified)
// Export: CreateLinkFileEx (ABI unverified)
// Export: #ordinal104 (ABI unverified)
// Export: DeleteGroup (ABI unverified)
// Export: #ordinal122 (ABI unverified)
// Export: DeleteLinkFile (ABI unverified)
// Export: DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName (ABI unverified)
// Export: ProcessGroupPolicyCompleted (ABI unverified)
// Export: RsopAccessCheckByType (ABI unverified)
// Export: RsopFileAccessCheck (ABI unverified)
// Export: CreateProfile (ABI unverified)
// Export: #ordinal208 (ABI unverified)
// Export: RemapProfile (ABI unverified)
// Export: DeleteProfileA (ABI unverified)
// Export: GetAllUsersProfileDirectoryA (ABI unverified)
// Export: GetDefaultUserProfileDirectoryA (ABI unverified)
// Export: GetProfilesDirectoryA (ABI unverified)
// Export: #ordinal203 (ABI unverified)
// Export: CheckDirectoryOwnership (ABI unverified)
// Export: #ordinal209 (ABI unverified)
// Export: CopyProfileDirectoryEx2 (ABI unverified)
// Export: #ordinal202 (ABI unverified)
// Export: DeleteProfileDirectory (ABI unverified)
// Export: #ordinal219 (ABI unverified)
// Export: DeleteProfileDirectory2 (ABI unverified)
// Export: DllCanUnloadNow (ABI unverified)
// Export: DllGetClassObject (ABI unverified)
// Export: DllRegisterServer (ABI unverified)
// Export: DllUnregisterServer (ABI unverified)
// Export: #ordinal214 (ABI unverified)
// Export: GetLongProfilePathName (ABI unverified)
// Export: #ordinal175 (ABI unverified)
// Export: CheckXForestLogon (ABI unverified)
// Export: #ordinal135 (ABI unverified)
// Export: PingComputer (ABI unverified)

#ifdef __cplusplus
} // extern "C"
#endif
#endif // USERENV_C_H
