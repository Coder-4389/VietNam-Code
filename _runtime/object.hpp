#ifndef Object
#define Object

// ****************************************
// --- start of code ---
// ****************************************

#include "global.hpp"
#include "kernel.hpp"

class vnobj {
	public:
	void del() {
        delete this;
    }
};

// ****************************************
// --- end of code ---
// ****************************************

#endif