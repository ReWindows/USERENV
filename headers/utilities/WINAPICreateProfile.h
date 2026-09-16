#pragma once
#include "../windissect_forwards.h"

// Reconstructed from USERENV.dll by Windissect. 4 member(s).
namespace UserenvTelemetry {
class WINAPICreateProfile {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StartActivity@WINAPICreateProfile@UserenvTelemetry@@QEAAXPEBG0@Z
    void StartActivity(unsigned short const *, unsigned short const *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1WINAPICreateProfile@UserenvTelemetry@@QEAA@XZ
    ~WINAPICreateProfile();
protected:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StopActivity@WINAPICreateProfile@UserenvTelemetry@@MEAAXXZ
    virtual void StopActivity();
    // Category: Method | Source: PDB Internal
    // Symbol: ?WasAlreadyReportedToTelemetry@WINAPICreateProfile@UserenvTelemetry@@MEAA_NJ@Z
    virtual bool WasAlreadyReportedToTelemetry(long);
};
} // namespace UserenvTelemetry
