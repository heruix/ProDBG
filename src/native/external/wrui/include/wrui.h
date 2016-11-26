#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#include "dock_widget.h"

struct GUWidget;
struct GUTabWidget;
struct GUPushButton;
struct GUWindow;
struct GUMainWindow;
struct GUDockWidget;
struct GUApplication;

#define GU_EVENT_RELEASED "2released()"

#define GU_INTERNAL_WIDGET_TYPE(type) \
typedef struct type { \
	GUWidget* base; \
} type

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef enum GUDockingArea {
    GUDockingArea_Left = 0x1,
    GUDockingArea_Right = 0x2,
    GUDockingArea_Top = 0x4,
    GUDockingArea_Bottom = 0x8,
    GUDockingArea_All = 0xf,
    GUDockingArea_None = 0,
} GUDockingArea;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUObjectFuncs {
	int (*connect)(void* sender, const char* id, void* reciver, void* func);
} GUObjectFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUWidgetFuncs {
	void (*set_size)(struct GUWidget* widget, int width, int height);
	void (*show)(struct GUWidget* widget);
} GUWidgetFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUWindowFuncs {
	void (*set_title)(struct GUWindow* win, const char* name);
} GUWindowFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUMainWindowFuncs {
	void (*set_central_widget)(struct GUMainWindow* win, struct GUWidget* widget);
	void (*add_dock_widget)(struct GUMainWindow* win, uint32_t area, struct GUDockWidget* widget);
	void (*set_window_title)(struct GUMainWindow* win, const char* title);
} GUMainWindowFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUTabWidgetFuncs {
	void (*clear)(struct GUTabWidget* tab);
} GUTabWidgetFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUPushButtonFuncs {
	void (*set_title)(struct GUPushButton* button, const char* text); 
} GUPushButtonFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUApplicationFuncs {
	int (*run)(struct GUApplication* p);
} GUApplicationFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUObject {
	void* p;
	GUObjectFuncs* object_funcs;
} GUObject;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUWidget {
	GUObject* object;
} GUWidget;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUDockWidget {
	GUWidget* base;
	void* privd;
} GUDockWidget;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

GU_INTERNAL_WIDGET_TYPE(GUWindow);
GU_INTERNAL_WIDGET_TYPE(GUTabWidget);
GU_INTERNAL_WIDGET_TYPE(GUPushButton);
GU_INTERNAL_WIDGET_TYPE(GUMainWindow);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct GUApplication {
	void* p; // private data
} GUApplication;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct Wrui {
	uint64_t api_version;

	GUApplication* (*application_create)();
	GUWindow* (*window_create)();
	GUPushButton* (*push_button_create)();
	GUMainWindow* (*main_window_create)();
	GUDockWidget* (*dock_widget_create)();
	GUTabWidget* (*tab_widget_create)();

	GUWidgetFuncs* widget_funcs;
	GUWindowFuncs* window_funcs;
	GUMainWindowFuncs* main_window_funcs;
	GUPushButtonFuncs* push_button_funcs;
	GUApplicationFuncs* application_funcs;
	GUDockWidgetFuncs* dock_widget_funcs; 
	GUTabWidgetFuncs* tab_widget_funcs; 

} Wrui;

#define WRUI_VERSION(major, minor, sub) ((((uint64_t)major) << 32) | (minor << 16) | (sub))

// Should be the only exported symbol

extern Wrui* wrui_get();

#ifdef __cplusplus
}
#endif

