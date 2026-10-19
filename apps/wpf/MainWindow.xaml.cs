using System.Windows;
using Scaffold.Bindings;

namespace Scaffold.Wpf;

public partial class MainWindow : Window
{
    readonly CalcEngine _engine = new();

    public MainWindow()
    {
        InitializeComponent();
    }

    void OnAdd(object sender, RoutedEventArgs e)
    {
        AddResult.Text = CalcApi.Add(int.Parse(ABox.Text), int.Parse(BBox.Text)).ToString();
    }

    void OnGreet(object sender, RoutedEventArgs e)
    {
        GreetResult.Text = TextApi.Greet(NameBox.Text);
    }

    void OnApply(object sender, RoutedEventArgs e)
    {
        _engine.SetFactor(int.Parse(FactorBox.Text));
        EngineResult.Text = _engine.Apply(long.Parse(ValueBox.Text)).ToString();
    }

    protected override void OnClosed(EventArgs e)
    {
        _engine.Dispose();
        base.OnClosed(e);
    }
}
