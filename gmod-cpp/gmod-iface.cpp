#define GMMODULE

#include "Interface.h"
#include <stdio.h>

using namespace GarrysMod::Lua;

extern "C" {
	const char* glua_get_string(lua_State* state, int stackPos) {
		return LUA->GetString(stackPos);
	}
	void glua_push_global(lua_State* state) {
		LUA->PushSpecial(GarrysMod::Lua::SPECIAL_GLOB);
	}
	void glua_push_string(lua_State* state, char* string) {
		LUA->PushString(string);
	}
	void glua_push_cfunc(lua_State* state, int (*f)(lua_State*)) {
		LUA->PushCFunction(f);
	}
	void glua_set_table(lua_State* state, int stackPos) {
		LUA->SetTable(stackPos);
	}
}