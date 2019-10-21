#pragma once

#include "MainPage.g.h"



namespace winrt::ReactNativeUWP::implementation
{
    struct MainPage : MainPageT<MainPage>
    {
        MainPage();

    private:
      void LoadReact();
    };
}

namespace winrt::ReactNativeUWP::factory_implementation
{
    struct MainPage : MainPageT<MainPage, implementation::MainPage>
    {
    };
}


