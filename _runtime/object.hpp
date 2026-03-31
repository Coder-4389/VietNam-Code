#ifdef Object_Hpp
#define Object_Hpp

// ****************************************
// --- start of code ---
// ****************************************
#include "global.hpp"
#include "kernel.hpp"

enum vntype {
    vni, i8, i16, i32, i64,
    vnf, f32, f64,
    vns, str8, str16, str32,
    vnchar, 
};

class vnobj {
	public: 
    vntype objtype;
	void del() {
        delete this;
    }
};

// ****************************************
// --- end of code ---
// ****************************************

#endif