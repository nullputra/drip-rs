// Run:
// $ g++ -Wall -o tests\data\z-algorithm tests\data\z-algorithm.cpp && tests\data\z-algorithm
#include <bits/stdc++.h>

using namespace std;

vector<int> z_algorithm(string &s, int x = 0) {
    int n = s.size() - x;
    vector<int> z(n);
    z[0] = n;
    for (int i = 1, len = 0; i < n;) {
        while (i + len < n && s[x + len] == s[x + i + len]) len++;
        z[i] = len;
        if (len == 0) {
            i++;
            continue;
        }
        int reused_len = 1;
        while (reused_len + z[reused_len] < len) {
            z[i + reused_len] = z[reused_len], reused_len++;
        }
        i += reused_len, len -= reused_len;
    }
    return z;
}
vector<int> z_search(const string &s, const string &ptn) {
    string t = ptn + "$" + s;
    int nt = s.size(), np = ptn.size();
    vector<int> z_idx, z = z_algorithm(t);
    for (int i = np + 1; i < nt + 2; i++) {
        if (z[i] == np) z_idx.push_back(i - np - 1);
    }
    return z_idx;
}

void z_algorithm_works() {
    string s, ptn;
    vector<int> model_ans;

    s = "atcoder";
    model_ans = {7, 0, 0, 0, 0, 0, 0};
    assert(z_algorithm(s) == model_ans);

    s = "ababa";
    model_ans = {5, 0, 3, 0, 1};
    assert(z_algorithm(s) == model_ans);

    s = "ababa";
    ptn = "aba";
    model_ans = {0, 2};
    assert(z_search(s, ptn) == model_ans);

    cout << "z-algorithm works" << endl;
}
int main() {
    z_algorithm_works();
    return 0;
}
