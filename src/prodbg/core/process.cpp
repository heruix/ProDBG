#include "process.h"

#ifndef _WIN32
#include <spawn.h>
#include <sys/wait.h>
#include <stdint.h>
#else
#include <windows.h>
#endif

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern char** environ;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

ProcessHandle Process_spawn(const char* exe, const char** args)
{
#ifndef _WIN32
	pid_t pid;
	int status = posix_spawn(&pid, exe, 0, 0, (char**)args, environ);
	if (status != 0)
		return 0;
	return (ProcessHandle)(uintptr_t)pid;
#else
	(void)exe;
	(void)args;
	return 0;
#endif

}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

int Process_wait(ProcessHandle handle)
{
#ifndef _WIN32
	int status = 0;
	waitpid((pid_t)(uintptr_t)handle, &status, 0);
#endif
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

int Process_kill(ProcessHandle handle)
{
#ifdef PRODBG_UNIX
	kill((pid_t)(uintptr_t)handle, 9);
#endif
	(void)handle;
	return 0;
}
