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

class ios {
	public:
	inline opl<bool> print(auto text, auto end = "\n") {
		if (cout << text << end) {return true;}
		return null;
	}

	inline opl<string> input(auto text = "") {
		string in_text;
		if (!print(text,"")) {return null;} 
		if (!cin >> in_text) {return null;} 
		return in_text;
	}
}

extern "C" {
	
}

inline opl<size_t> len(auto object) {
	size_t cnt = object.len();
	if (cnt >= 0) {return cnt;}
	return null
}

// ****************************************
// --- end of code ---
// ****************************************
#endif