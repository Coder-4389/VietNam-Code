#ifndef KERNEL_HPP
#define KERNEL_HPP

// ****************************************
// --- start of code ---
// ****************************************

// --- object lib ---
#include <string>
#include <vector>
#include <map>
#include <optional>

// --- int lib ---
#include <cstdint>

// --- exec lib ---
#include <iostream>

using namespace std;

template <typename t1>
using opl = optional<t1>;

constexpr auto null = nullopt;

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

struct vstd {
	vnios ios;
	vnmet met;
};

extern "C" {
	opl<bool> ios_init() {
		if (&vstd::ios != nullptr) {return true;}
		return null;
	}
	
	opl<bool> met_init() {
		if (&vstd::met != nullptr) {return true;}
		return null;
	}
}

// ****************************************
// --- end of code ---
// ****************************************
#endif