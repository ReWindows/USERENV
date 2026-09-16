#pragma once
#include "../windissect_forwards.h"

// Reconstructed from USERENV.dll by Windissect. 8 member(s).
class CThreadContext {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?EnablePrivileges@CThreadContext@@QEAAJPEAKK@Z
    long EnablePrivileges(unsigned long *, unsigned long);
    // Category: Method | Source: PDB Internal
    // Symbol: ?ImpersonateSelf@CThreadContext@@QEAAJXZ
    long ImpersonateSelf();
    // Category: Method | Source: PDB Internal
    // Symbol: ?ImpersonateUser@CThreadContext@@QEAAJPEAX@Z
    long ImpersonateUser(void *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?PrivilegeEnabled@CThreadContext@@QEAAHK@Z
    int PrivilegeEnabled(unsigned long);
    // Category: Method | Source: PDB Internal
    // Symbol: ?RevertPrivileges@CThreadContext@@QEAAJXZ
    long RevertPrivileges();
    // Category: Method | Source: PDB Internal
    // Symbol: ?RevertToPreviousToken@CThreadContext@@QEAAXXZ
    void RevertToPreviousToken();
    // Category: Method | Source: PDB Internal
    // Symbol: ?RevertToSelf@CThreadContext@@QEAAXXZ
    void RevertToSelf();
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1CThreadContext@@QEAA@XZ
    ~CThreadContext();
};
