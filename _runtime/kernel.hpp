#ifndef Kernel_Hpp
#define Kernel_Hpp

// ****************************************
// --- start of code ---
// ****************************************

#include "global.hpp"
#include "untils.hpp"

class vnios {
	public:
	inline opl<bool> print(auto text, auto end = "\n") {
		if (cout << text << end) {return true;}
		return null;
	}

	inline opl<string> input(auto text = "") {
		print(text,""); string in_text;
		if (cin >> in_text) {return in_text;} 
		return in_text;
	}
};

class vnmet {
	public:
	opl<size_t> len(auto obj) {
		size_t cnt;
		if (cnt = obj.len()) {return cnt;}
		return null;
	} 
};

struct vnstd {
	vnios ios;
	vnmet met;
};

vnstd vst;

extern "C" {
	opl<bool> ios_init() {
		if (&vst.ios != nullptr) {return true;}
		return null;
	}
	
	opl<bool> met_init() {
		if (&vst.met != nullptr) {return true;}
		return null;
	}
}

// ****************************************
// --- end of code ---
// ****************************************

#endif