using System;
using System.Windows;
using System.Threading.Tasks;

namespace MyWpfApp
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private async void ButtonExit_Click(object sender, RoutedEventArgs e)
        {
           Application.Current.Shutdown();
           //await CallWindowsRuntimeApisAsync();
        }

#region Windows Runtime API's
        // private async Task CallWindowsRuntimeApisAsync()
        // {
        //     var locator = new Windows.Devices.Geolocation.Geolocator();
        //     var location = await locator.GetGeopositionAsync();
        //     var position = location.Coordinate.Point.Position;
        //     var latlong = string.Format("lat:{0}, long:{1}", position.Latitude, position.Longitude);
        //     var result = MessageBox.Show(latlong);
        // }
    }
#endregion
}
