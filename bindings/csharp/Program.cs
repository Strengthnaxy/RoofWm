namespace RoofWm;

public readonly record struct Rect(int X, int Y, uint W, uint H);

public static class Layout
{
    public static List<Rect> Dwindle(uint width, uint height, int count)
    {
        var rects = new List<Rect>(count);
        int x = 0, y = 0;
        uint w = width, h = height;
        for (int i = 0; i < count; i++)
        {
            if (w >= h)
            {
                uint half = w / 2;
                rects.Add(new Rect(x, y, half, h));
                x += (int)half;
                w -= half;
            }
            else
            {
                uint half = h / 2;
                rects.Add(new Rect(x, y, w, half));
                y += (int)half;
                h -= half;
            }
        }
        return rects;
    }
}

public static class Program
{
    public static void Main()
    {
        var rects = Layout.Dwindle(1920, 1080, 4);
        Console.WriteLine($"roofwm csharp binding: {rects.Count} rects");
        foreach (var r in rects)
            Console.WriteLine($"  {r.X},{r.Y} {r.W}x{r.H}");
    }
}
