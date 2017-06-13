#include <stdlib.h>
#include <stdio.h>
#include <memory.h>

typedef unsigned char uchar;
typedef struct {
    uchar* array;
    size_t size;
} bigint;


int bigint_expand(bigint* data) {
    uchar* new_array;
    size_t old_size = data->size;
    size_t new_size = data->size * 1.5 + 1;
    new_array = realloc(data->array, new_size);
    if(new_array == NULL) {
        return 0;
    }
    data->array = new_array;
    data->size = new_size;
    memset(data->array + old_size, 0, new_size - old_size);
    return 1;
}

int bigint_init(bigint* value) {
    value->array = (uchar*) malloc(1);
    if( value->array == NULL) {
        value->size = 0;
        return 0;
    }
    value->array[0] = 1;
    value->size = 1;
    return 1;
}

void bigint_free(bigint* value) {
    free(value->array);
    value->array = NULL;
    value->size = 0;
}

int bigint_double(bigint* value) {
    int i;
    int ret = 1;
    uchar carry = 0, product;
    for(i = 0; i < value->size; i++) {
        product =  value->array[i] * 2 + carry;
        carry = product / 10;
        value->array[i] = product % 10;
    }
    if (carry != 0) {
        int old_size = value->size;
        ret = bigint_expand(value);
        if(ret) {
            value->array[old_size] = carry;
        }
    }
    return ret;
}

void bigint_print(bigint* value) {
    int i;
    int only_zeros = 1;
    for(i = value->size - 1; i >= 0; i--) {
        if(value->array[i] == 0 && only_zeros) {
            continue;
        }
        printf("%u", value->array[i]);
        only_zeros = 0;
    }
    printf("\n");
}

#define BIGINT_CHECK(bigint_struct, cmd) do \
{ \
    if(!cmd) { \
        bigint_free(bigint_struct);\
        printf("BIGINT Error @%s:%d\n", __FILE__, __LINE__); \
        return 1; \
    } \
} while(0)

int main(int argc, char ** argv) {
    if(argc == 1) {
        printf("Error, power expected\n");
        exit(1);
    }

    int power, max_power = atol(argv[1]);
    bigint value;
    BIGINT_CHECK(&value, bigint_init(&value));
    for(power = 0; power < max_power; power++) {
        BIGINT_CHECK(&value, bigint_double(&value));
    }
    bigint_print(&value);
    bigint_free(&value);
    return 0;
}

