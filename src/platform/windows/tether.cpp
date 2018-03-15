#include "tether.h"

//TODO: I'm pretty sure the destructor will never run.

using namespace Platform::Collections;
using namespace Windows::ApplicationModel;
using namespace Windows::ApplicationModel::Activation;
using namespace Windows::ApplicationModel::Core;
using namespace Windows::Foundation;
using namespace Windows::UI::Core;
using namespace Windows::UI::ViewManagement;
using namespace Windows::UI::Xaml;
using namespace Windows::UI::Xaml::Controls;

Platform::String^ convert_string(tether_string in) {
    const char* ptr = (const char*) in.ptr;
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    std::wstring wstr = converter.from_bytes(ptr, ptr + in.len);
    return ref new Platform::String(wstr.c_str());
}

std::string trevnoc_string(Platform::String^ in) {
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    return converter.to_bytes(in->Data());
}

class Handler {
public:

    Handler() {
        this->data = NULL;
        this->rmessage = [](void*, tether_string){};
        this->rsuspend = [](void*){};
    }

    void message(tether_string msg) {
        rmessage(data, msg);
    }

    void suspend() {
        rsuspend(data);
    }

    void* data;
    void (*rmessage) (void*, tether_string);
    void (*rsuspend) (void*);
};

ref class Program sealed : Application {
public:

    Program() {
        Suspending += ref new SuspendingEventHandler(this, &Program::OnSuspending);
    }

internal:

    Platform::String^ html;
    Size size, min_size;
    bool fullscreen;
    Handler handler;

    void Message(Platform::Object^ sender, NotifyEventArgs^ e) {
        std::string s = trevnoc_string(e->Value);
        tether_string rs;
        rs.len = s.size();
        rs.ptr = (const unsigned char*) s.data();
        handler.message(rs);
    }

    void AttachScripts(WebView^ sender, WebViewDOMContentLoadedEventArgs^ p) {
        Vector<Platform::String^>^ args = ref new Vector<Platform::String^>();
        args->Append("window.tether = function (s) { window.external.notify(s); };");
        sender->InvokeScriptAsync("eval", args);
    }

    void OnSuspending(Object^ sender, SuspendingEventArgs^ e) {
        handler.suspend();
    }

protected:

    virtual void OnLaunched(LaunchActivatedEventArgs^ e) override {
        if (Window::Current->Content) return;

        ApplicationView::PreferredLaunchViewSize = size;
        ApplicationView::GetForCurrentView()->SetPreferredMinSize(min_size);
        ApplicationView::PreferredLaunchWindowingMode = fullscreen
            ? ApplicationViewWindowingMode::FullScreen
            : ApplicationViewWindowingMode::PreferredLaunchViewSize;

        // Make and attach the web view.

        WebView^ webview = ref new WebView();
        Window::Current->Content = webview;

        // Setup the window title.

        webview->RegisterPropertyChangedCallback(
            WebView::DocumentTitleProperty,
            ref new DependencyPropertyChangedCallback([] (DependencyObject^ sender, DependencyProperty^ dp) {
                ApplicationView::GetForCurrentView()->Title = (Platform::String^) sender->GetValue(dp);
            })
        );

        // Register the message event.
        
        webview->ScriptNotify += ref new NotifyEventHandler(this, &Program::Message);
        webview->DOMContentLoaded += ref new TypedEventHandler<WebView^, WebViewDOMContentLoadedEventArgs^>(this, &Program::AttachScripts);

        // Set the HTML.

        webview->NavigateToString(html);

        // Open the window.

        Window::Current->Activate();
    }
};

extern "C" {
    void tether_start(
        tether_string html,
        uintptr_t width,
        uintptr_t height,
        uintptr_t min_width,
        uintptr_t min_height,
        int fullscreen,

        void* hdata,
        void (*hmessage) (void*, tether_string),
        void (*hsuspend) (void*)
    ) {
        RoInitialize(RO_INIT_MULTITHREADED);

        Application::Start(ref new ApplicationInitializationCallback([=] (ApplicationInitializationCallbackParams^ p) {
            Program^ program = ref new Program();
            program->html = convert_string(html);
            program->size = Size((float) width, (float) height);
            program->min_size = Size((float) min_width, (float) min_height);
            program->fullscreen = fullscreen;
            program->handler.data = hdata;
            program->handler.rmessage = hmessage;
            program->handler.rsuspend = hsuspend;
        }));
    }

    void tether_load(tether_string html) {
        WebView^ webview = (WebView^) Window::Current->Content;

        if (webview) {
            webview->NavigateToString(convert_string(html));
        }
    }

    void tether_eval(tether_string js) {
        WebView^ webview = (WebView^) Window::Current->Content;

        if (webview) {
            Vector<Platform::String^>^ args = ref new Vector<Platform::String^>();
            args->Append(convert_string(js));
            webview->InvokeScriptAsync("eval", args);
        }
    }

    void tether_dispatch(void* data, void (*exec) (void*)) {
        CoreApplication::MainView->CoreWindow->Dispatcher->RunAsync(
            CoreDispatcherPriority::Normal,
            ref new DispatchedHandler([=] () {
                exec(data);
            })
        );
    }
}
