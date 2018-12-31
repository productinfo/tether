#include <stdlib.h>
#include <tether.h>

// SLAVE

struct slave {
    tether window;
};

void slave_message(void *data, const char *message) {
    (void)message;
    struct slave *slave = (struct slave *)data;
    tether_close(slave->window);
}

void slave_closed(void *data) {
    free(data);
}

void slave_new(void) {
    struct slave *data = malloc(sizeof *data);

    tether slave = data->window = tether_new((tether_options) {
        .initial_width = 200,
        .initial_height = 100,
        .minimum_width = 200,
        .minimum_height = 100,
        .borderless = false,
        .debug = false,
        .data = data,
        .message = slave_message,
        .closed = slave_closed
    });

    tether_title(slave, "Slave");
    tether_load(slave, "<script>setTimeout(() => window.tether(''), 1000);</script>");
}

// MASTER

void master_message(void *data, const char *message) {
    (void)data;
    if (*message) {
        tether_exit();
    } else {
        slave_new();
    }
}

void master_closed(void *data) { (void)data; }

void master_new(void) {
    tether window = tether_new((tether_options) {
        .initial_width = 800,
        .initial_height = 600,
        .minimum_width = 0,
        .minimum_height = 0,
        .borderless = false,
        .debug = true,
        .data = NULL,
        .message = master_message,
        .closed = master_closed
    });

    tether_title(window, "Master");
    tether_load(window, "<script>\
        let slaves = 0;\
        (function go() {\
            window.tether('');\
            slaves += 1;\
            if (slaves < 500) setTimeout(go, 50);\
            else window.tether('_');\
        })();\
    </script>");
}

// MAIN

void start(void) {
    master_new();
}

int main(void) {
    tether_start(start);
    return 0;
}
