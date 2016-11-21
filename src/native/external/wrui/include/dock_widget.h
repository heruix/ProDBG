#pragma once

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// http://doc.qt.io/qt-5/qdockwidget.html

struct GUWidget;
struct GUDockWidget;

typedef struct GUDockWidgetFuncs {
	bool (*is_floating)(struct GUDockWidget* dock_widget);
	void (*set_floating)(struct GUDockWidget* dock_widget, bool floating);
	void (*set_widget)(struct GUDockWidget* dock_widget, struct GUWidget* widget);
	struct GUWidget* (*widget)(struct GUDockWidget* dock_widget);
} GUDockWidgetFuncs;

#ifdef __cplusplus
}
#endif


