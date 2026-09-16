#pragma once
#include "../windissect_forwards.h"

// Reconstructed from USERENV.dll by Windissect. 4 member(s).
namespace UserenvTelemetry {
class WINAPIDeleteProfile {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StartActivity@WINAPIDeleteProfile@UserenvTelemetry@@QEAAXPEBG0@Z
    void StartActivity(unsigned short const *, unsigned short const *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1WINAPIDeleteProfile@UserenvTelemetry@@QEAA@XZ
    ~WINAPIDeleteProfile();
protected:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StopActivity@WINAPIDeleteProfile@UserenvTelemetry@@MEAAXXZ
    virtual void StopActivity();
    // Category: Method | Source: PDB Internal
    // Symbol: ?WasAlreadyReportedToTelemetry@WINAPIDeleteProfile@UserenvTelemetry@@MEAA_NJ@Z
    virtual bool WasAlreadyReportedToTelemetry(long);
};
} // namespace UserenvTelemetry
