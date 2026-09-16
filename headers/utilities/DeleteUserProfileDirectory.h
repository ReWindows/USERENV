#pragma once
#include "../windissect_forwards.h"

// Reconstructed from USERENV.dll by Windissect. 4 member(s).
namespace UserenvTelemetry {
class DeleteUserProfileDirectory {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StartActivity@DeleteUserProfileDirectory@UserenvTelemetry@@QEAAXXZ
    void StartActivity();
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1DeleteUserProfileDirectory@UserenvTelemetry@@QEAA@XZ
    ~DeleteUserProfileDirectory();
protected:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StopActivity@DeleteUserProfileDirectory@UserenvTelemetry@@MEAAXXZ
    virtual void StopActivity();
    // Category: Method | Source: PDB Internal
    // Symbol: ?WasAlreadyReportedToTelemetry@DeleteUserProfileDirectory@UserenvTelemetry@@MEAA_NJ@Z
    virtual bool WasAlreadyReportedToTelemetry(long);
};
} // namespace UserenvTelemetry
