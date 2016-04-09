#pragma once

#ifdef __cplusplus
extern "C" {
#endif

struct PDGRect;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void Settings_getWindowRect(struct PDGRect* rect);
void Settings_setWindowRect(struct PDGRect* rect);
void Settings_load();
void Settings_save();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef __cplusplus
}
#endif
