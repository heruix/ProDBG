#pragma once

#include "core/math.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum MouseButton
{
	MouseButton_Left,
	MouseButton_Middle,
	MouseButton_Right,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct InputState
{
	Vec2 mousePos;// position within the window
	Vec2 mouseScreenPos; // position on the screen

	bool mouseDown[16]; // mouse button states
    bool keysDown[512]; // Keyboard keys that are pressed
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

bool Input_isKeyDown(int key);
