#include <string>
#include <vector>
#include <map>

using namespace std;

template<typename t1, typename t2 = string>
t1 print(t1 text, t2 end = "\n") {
    cout << text << end;
    return text;
}

template<typename t1>
t1 input(t1 text = "") {
    string input;
	cout << text; cin >> input;
    return input;
}
