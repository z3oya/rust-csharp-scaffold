using Scaffold.Bindings;

namespace Scaffold.WinForms;

internal sealed class MainForm : Form
{
    readonly TextBox _aBox = new() { Width = 60, Text = "2" };
    readonly TextBox _bBox = new() { Width = 60, Text = "3" };
    readonly Label _addResult = new() { AutoSize = true };
    readonly TextBox _nameBox = new() { Width = 140, Text = "Ada" };
    readonly Label _greetResult = new() { AutoSize = true };
    readonly TextBox _factorBox = new() { Width = 60, Text = "3" };
    readonly TextBox _valueBox = new() { Width = 80, Text = "5" };
    readonly Label _engineResult = new() { AutoSize = true };
    readonly CalcEngine _engine = new();

    public MainForm()
    {
        Text = "rust-csharp-scaffold (WinForms)";
        FormBorderStyle = FormBorderStyle.FixedSingle;
        MaximizeBox = false;
        StartPosition = FormStartPosition.CenterScreen;

        Button add = new() { Text = "Add", AutoSize = true };
        add.Click += (_, _) =>
            _addResult.Text = CalcApi.Add(int.Parse(_aBox.Text), int.Parse(_bBox.Text)).ToString();

        Button greet = new() { Text = "Greet", AutoSize = true };
        greet.Click += (_, _) => _greetResult.Text = TextApi.Greet(_nameBox.Text);

        Button apply = new() { Text = "Apply", AutoSize = true };
        apply.Click += (_, _) =>
        {
            _engine.SetFactor(int.Parse(_factorBox.Text));
            _engineResult.Text = _engine.Apply(long.Parse(_valueBox.Text)).ToString();
        };

        FlowLayoutPanel addRow = new() { AutoSize = true };
        addRow.Controls.AddRange(
            [_aBox, Label("+"), _bBox, add, _addResult]
        );

        FlowLayoutPanel greetRow = new() { AutoSize = true };
        greetRow.Controls.AddRange([_nameBox, greet, _greetResult]);

        FlowLayoutPanel engineRow = new() { AutoSize = true };
        engineRow.Controls.AddRange(
            [Label("factor"), _factorBox, Label("value"), _valueBox, apply, _engineResult]
        );

        FlowLayoutPanel layout =
            new()
            {
                Dock = DockStyle.Fill,
                FlowDirection = FlowDirection.TopDown,
                Padding = new Padding(12),
            };
        layout.Controls.AddRange([addRow, greetRow, engineRow]);
        Controls.Add(layout);
    }

    static Label Label(string text) => new() { Text = text, AutoSize = true };

    protected override void OnFormClosed(FormClosedEventArgs e)
    {
        _engine.Dispose();
        base.OnFormClosed(e);
    }
}
