#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include <string.h>

#include <math.h>
#include <time.h>

// Include the Weld API.
#include "../../../c/weld.h"

struct weld_vector {
    float *data;
    int64_t length;
};

struct args {
    struct weld_vector vector;
};

const char *program = "|x:vec[f32]| result(for(x, appender[f32], |b,i,e| merge(b, sqrt(e))))";

int main() {
    // Compile Weld module.
    weld_error_t e = weld_error_new();
    weld_conf_t conf = weld_conf_new();
    weld_module_t m = weld_module_compile(program, conf, e);
    weld_conf_free(conf);

    if (weld_error_code(e)) {
        const char *err = weld_error_message(e);
        printf("Error message: %s\n", err);
        exit(1);
    }

    weld_vector v;
    const uint64_t length = 100000000;
    float *data = (float *)malloc(sizeof(float) * (length));
    for (int i = 0; i < length; i++) {
        data[i] = 4;
    }

    v.data = data;
    v.length = length;

    struct args a;
    a.vector = v;

    weld_value_t arg = weld_value_new(&a);

    // Run the module and get the result.
    conf = weld_conf_new();

    clock_t start = clock();
    weld_value_t result = weld_module_run(m, conf, arg, e);
    clock_t end = clock();

    printf("Weld Runtime: %f\n", ((double)(end - start)) / CLOCKS_PER_SEC);

    if (weld_error_code(e)) {
        const char *err = weld_error_message(e);
        printf("Error message: %s\n", err);
        exit(1);
    }

    void *result_data = weld_value_data(result);
    weld_vector *res = (weld_vector *)result_data;

    printf("Answer: %f\n", res->data[0]);

    start = clock();
    float *cresult = (float *)malloc(sizeof(float) * (length));
    for (int i = 0; i < length; i++) {
        cresult[i] = sqrtf(data[i]);
    }
    end = clock();

    printf("C Runtime: %f\n", ((double)(end - start)) / CLOCKS_PER_SEC);
    printf("Answer: %f\n", cresult[0]);

    free(data);

    // Free the values.
    weld_value_free(result);
    weld_value_free(arg);
    weld_conf_free(conf);

    weld_error_free(e);
    weld_module_free(m);
    return 0;
}
