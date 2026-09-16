#pragma once
#include "../windissect_forwards.h"

// Reconstructed from USERENV.dll by Windissect. 4 member(s).
namespace UserenvTelemetry {
class WINAPIRemapProfile {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StartActivity@WINAPIRemapProfile@UserenvTelemetry@@QEAAXPEBG0@Z
    void StartActivity(unsigned short const *, unsigned short const *);
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1WINAPIRemapProfile@UserenvTelemetry@@QEAA@XZ
    ~WINAPIRemapProfile();
protected:
    // Category: Method | Source: PDB Internal
    // Symbol: ?StopActivity@WINAPIRemapProfile@UserenvTelemetry@@MEAAXXZ
    virtual void StopActivity();
    // Category: Method | Source: PDB Internal
    // Symbol: ?WasAlreadyReportedToTelemetry@WINAPIRemapProfile@UserenvTelemetry@@MEAA_NJ@Z
    virtual bool WasAlreadyReportedToTelemetry(long);
};
} // namespace UserenvTelemetry
