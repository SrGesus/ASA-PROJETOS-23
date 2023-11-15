
class foo {
public:
    template<typename T>
    int f(T banana) {
        // ...
    }

    virtual int fun(int ) {
        // ...
    }
};

class bar : public foo {
public:
    template<typename T>
    int f(T banana) {

    }
    int fun(int i) override {

    }
};

int main() {
    foo var = bar();
    var.f(1);
    return 0;
}

