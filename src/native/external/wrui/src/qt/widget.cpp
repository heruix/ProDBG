#include "../../include/wrui.h"
#include "signal_wrappers.h"

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

void object_setup(GUObject* object, void* data) {
	object->p = data;
	object->object_funcs = &s_objFuncs;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void widget_setup(GUWidget* widget, void* data) {
	widget->object = new GUObject;
	object_setup(widget->object, data);
}

