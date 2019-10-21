using System;
using System.Windows;

namespace XamlIslands.WPF
{
    public partial class MainWindow : Window
    {
        Windows.UI.Xaml.Controls.Button uwpButton;

        public MainWindow()
        {
            InitializeComponent();
        }

        private void OnChildChanged(object sender, EventArgs e)
        {
            if (uwpButton == null)
            {
                uwpButton = ((Microsoft.Toolkit.Wpf.UI.XamlHost.WindowsXamlHost)sender).Child as Windows.UI.Xaml.Controls.Button;
                uwpButton.HorizontalAlignment = Windows.UI.Xaml.HorizontalAlignment.Stretch;
                uwpButton.VerticalAlignment = Windows.UI.Xaml.VerticalAlignment.Stretch;
                uwpButton.Background = new Windows.UI.Xaml.Media.SolidColorBrush(Windows.UI.Colors.Red);
                uwpButton.Click += delegate { uwpButton.Background = new Windows.UI.Xaml.Media.SolidColorBrush(Windows.UI.Colors.Blue); };
            }
        }
    }
}
