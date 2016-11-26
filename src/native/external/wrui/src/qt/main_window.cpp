
#include "../../include/wrui.h"
#include <QMainWindow>

extern GUMainWindowFuncs g_mainWindowFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void add_dock_widget(GUMainWindow* win, uint32_t area, struct GUDockWidget* w) {
	QMainWindow* main_win = (QMainWindow*)win->base->object->p;
	QDockWidget* widget = (QDockWidget*)w->base->object->p;

	int flags = 0;

	if (area & GUDockingArea_Left)
	    flags |= Qt::LeftDockWidgetArea;

	if (area & GUDockingArea_Right)
	    flags |= Qt::RightDockWidgetArea;

	if (area & GUDockingArea_Top)
	    flags |= Qt::TopDockWidgetArea;

	if (area & GUDockingArea_Bottom)
	    flags |= Qt::BottomDockWidgetArea;

    return main_win->addDockWidget((Qt::DockWidgetArea)flags, widget);;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_central_widget(GUMainWindow* win, struct GUWidget* widget) {
	QMainWindow* qt_main_win = (QMainWindow*)win->base->object->p;
	QWidget* qt_widget = (QWidget*)widget->object->p;
	qt_main_win->setCentralWidget(qt_widget);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void set_window_title(GUMainWindow* win, const char* title) {
	QMainWindow* qt_main_win = (QMainWindow*)win->base->object->p;
	qt_main_win->setWindowTitle(QString(title));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct GUMainWindowFuncs g_mainWindowFuncs = {
	set_central_widget,
	add_dock_widget,
	set_window_title
};

