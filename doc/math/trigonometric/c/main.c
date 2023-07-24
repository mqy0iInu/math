#include <stdio.h>
#include <stdint.h>
#include <math.h>

int main(int argc, char **argv) {
    char type;
    double value;

    printf("sin, cos, or tan?\nEnter -> 's' ... sin\nEnter -> 'c' ... cos\nEnter -> 't' ... tan\n");
    printf(">");
    scanf("%c", &type);

    printf("Enter value(rad)\n");
    printf(">");
    scanf("%lf", &value);

    switch (type) {
        case 's':
        case 'S':
            printf("sin(%.2lf) = %.2lf\n", value, sin(value));
            break;
        case 'c':
        case 'C':
            printf("cos(%.2lf) = %.2lf\n", value, cos(value));
            break;
        case 't':
        case 'T':
            printf("tan(%.2lf) = %.2lf\n", value, tan(value));
            break;
        default:
            printf("Invalid choice\n");
            break;
    }

    return 0;
}
