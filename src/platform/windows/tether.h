#include <codecvt>
#include <cstdint>
#include <string>
#include <wrl.h>
#include <collection.h>
#include <Windows.UI.Core.h>
#include <Windows.UI.ViewManagement.h>
#include <Windows.UI.Xaml.h>
#include <Windows.ApplicationModel.Activation.h>
#include <Windows.ApplicationModel.Core.h>

extern "C" {
    typedef struct _tether_string {
        uintptr_t len;
        const unsigned char* ptr;
    } tether_string;

    void tether_start(
        tether_string html,
        uintptr_t width,
        uintptr_t height,
        uintptr_t min_width,
        uintptr_t min_height,
        int fullscreen,

        void* han_data,
        void (*han_message) (void*, tether_string),
        void (*han_suspend) (void*)
    );

    void tether_load(tether_string html);

    void tether_eval(tether_string js);

    void tether_dispatch(
        void* data,
        void (*exec) (void*)
    );
}
