// Windissect exhaustive candidate class surface for USERENV.dll.
// Experimental: declarations may contain unresolved PDB language ambiguities.
#pragma once

#include "windissect_abi.h"
#include "windissect_forwards.h"
#include "windissect_vcalls.h"
#include "windissect_calls.h"
#include "utilities/CThreadContext.h"
#include "utilities/UserenvTelemetry.h"
#include "utilities/DeleteUserProfileDirectory.h"
#include "utilities/WINAPICreateProfile.h"
#include "utilities/WINAPIDeleteProfile.h"
#include "utilities/WINAPIRemapProfile.h"
#include "utilities/ProfileAPILogging.h"
#include "utilities/CAutoImpersonate.h"
#include "utilities/CCopyError.h"
#include "utilities/proflib.h"
#include "utilities/ResourceString.h"
