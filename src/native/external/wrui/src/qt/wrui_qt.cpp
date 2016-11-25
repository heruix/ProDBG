#include "../../include/wrui.h"
#include "signal_wrappers.h"
#include "widget_private.h"
#include <QApplication>
#include <QPushButton>
#include <QMainWindow>

extern struct GUDockWidget* dock_widget_create();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int connect(void* sender, const char* id, void* reciver, void* func) {
	GUObject* object = (GUObject*)sender;
	QObject* q_obj = (QObject*)object->p;

	QSlotWrapperNoArgs* wrap = new QSlotWrapperNoArgs(reciver, (SignalNoArgs)func);

	QObject::connect(q_obj, id, wrap, SLOT(method()));
	/*
		return 1;
	} else {
		printf("wrui: unable to create connection between (%p - %s) -> (%p -> %p)\n");
		return 0;
	}
	*/
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct GUObjectFuncs s_objFuncs = {
	connect,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void widget_set_size(struct GUWidget* widget, int width, int height) {
	QObject* q_obj = (QObject*) widget->object->p;
	QWidget* q_widget = static_cast<QWidget*>(q_obj);
	q_widget->resize(width, height);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct GUWidgetFuncs s_widgetFuncs = {
	widget_set_size,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int app_run(GUApplication* app) {
	QApplication* qt_app = (QApplication*)app->p;
	return qt_app->exec();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static struct GUApplicationFuncs s_appFuncs = {
	app_run,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static GUApplication* application_create() {
	int argc = 0;
	GUApplication* app = new GUApplication;
	app->p = (void*) new QApplication(argc, 0);

	return app;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static GUPushButton* push_button_create() {
	QPushButton* qt_button = new QPushButton(0, 0);
	qt_button->show();

	// TODO: Smarter allocator than just using new all the time

	GUPushButton* button = new GUPushButton;

	button->base = new GUWidget;

	widget_setup(button->base, (void*) static_cast<QObject*>(qt_button));

	return button;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static GUMainWindow* main_window_create() {
	QMainWindow* qt_win = new QMainWindow();
	qt_win->setWindowTitle("none");

	// TODO: Smarter allocator than just using new all the time

	GUMainWindow* win = new GUMainWindow;
	win->base = new GUWidget;

	widget_setup(win->base, (void*) static_cast<QObject*>(qt_win));

	return win;
}

extern GUDockWidgetFuncs g_dockWidgetFuncs;
extern GUMainWindowFuncs g_mainWindowFuncs;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static Wrui s_wrui = {
	WRUI_VERSION(0, 0, 1),

	// user facing

	application_create,
	0,
	push_button_create,
	main_window_create,
	dock_widget_create,

	// funcs

	&s_objFuncs,
	&s_widgetFuncs,
	0,
	&g_mainWindowFuncs,
	0,
	&s_appFuncs,
	0
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" Wrui* wrui_get() {
	return &s_wrui;
}
