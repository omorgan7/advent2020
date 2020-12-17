#include <vector>
#include <string>
#include <cmath>
#include <cstdio>

enum State
{
    Active,
    Inactive
};

struct Grid3D
{
    int size;
    int lower;
    int upper;

    Grid3D(int sz)
    {
        size = sz;
        lower = -sz / 2;
        if (size % 2 == 0)
        {
            upper = sz / 2 - 1;
        }
        else
        {
            upper = sz / 2;
        }
        s = std::vector<State>(size * size * size, Inactive);
    }

    State& operator()(int x, int y, int z)
    {
        int index =
            size * size * (z - lower) +
            size * (y - lower) +
            x - lower;

        if (index < 0 || index >= s.size()) {
            printf("OUT OF BOUNDS ACCESS FOR %d %d %d, %d", x, y, z, index);
            std::abort();
        }
        return s[index];
    }

    void grow()
    {
        int newSize = size + 2;
        Grid3D newGrid3D = Grid3D(newSize);

        for (int z = lower; z <= upper; ++z)
        {
            for (int y = lower; y <= upper; ++y)
            {
                for (int x = lower; x <= upper; ++x)
                {
                    newGrid3D(x, y, z) = (*this)(x, y, z);
                }
            }
        }

        *this = newGrid3D;
    }

    void update()
    {
        Grid3D newGrid3D = *this;
        for (int z = lower + 1; z <= upper - 1; ++z)
        {
            for (int y = lower + 1; y <= upper - 1; ++y)
            {
                for (int x = lower + 1; x <= upper - 1; ++x)
                {
                    State s = (*this)(x, y, z);
                    int active_count = 0;
                    for (int z_offset = -1; z_offset <= 1; ++z_offset)
                    {
                        for (int y_offset = -1; y_offset <= 1; ++y_offset)
                        {
                            for (int x_offset = -1; x_offset <= 1; ++x_offset)
                            {
                                if (x_offset == 0 && y_offset == 0 && z_offset == 0)
                                {
                                    continue;
                                }
                                active_count += (*this)(x + x_offset, y + y_offset, z + z_offset) == Active;
                            }
                        }
                    }

                    if (s == Active)
                    {
                        if (active_count == 2 || active_count == 3)
                        {}
                        else
                        {
                            newGrid3D(x, y, z) = Inactive;
                        }
                    }
                    else {
                        if (active_count == 3)
                        {
                            newGrid3D(x, y, z) = Active;
                        }
                    }
                }
            }
        }

        *this = newGrid3D;
    }

    int active()
    {
        int c = 0;
        for (const auto state : s)
        {
            c += state == Active;
        }

        return c;
    }

    void render()
    {
        for (int z = lower; z <= upper; ++z)
        {
            printf("z=%d\n", z);
            for (int y = lower; y <= upper; ++y)
            {
                for (int x = lower; x <= upper; ++x)
                {
                   printf("%c", (*this)(x, y, z) == Active ? '#' : '.');
                }
                printf("\n");
            }
            printf("\n");
        }
    }

    std::vector<State> s;
};

struct Grid4D
{
    int size;
    int lower;
    int upper;

    Grid4D(int sz)
    {
        size = sz;
        lower = -sz / 2;
        if (size % 2 == 0)
        {
            upper = sz / 2 - 1;
        }
        else
        {
            upper = sz / 2;
        }
        s = std::vector<State>(size * size * size * size, Inactive);
    }

    State& operator()(int x, int y, int z, int w)
    {
        int index =
            size * size * size * (w - lower) +
            size * size * (z - lower) +
            size * (y - lower) +
            x - lower;

        if (index < 0 || index >= s.size()) {
            printf("OUT OF BOUNDS ACCESS FOR %d %d %d %d, %d", x, y, z, w, index);
            std::abort();
        }
        return s[index];
    }

    void grow()
    {
        int newSize = size + 2;
        Grid4D newGrid4D = Grid4D(newSize);

        for (int w = lower; w <= upper; ++w)
        {
            for (int z = lower; z <= upper; ++z)
            {
                for (int y = lower; y <= upper; ++y)
                {
                    for (int x = lower; x <= upper; ++x)
                    {
                        newGrid4D(x, y, z, w) = (*this)(x, y, z, w);
                    }
                }
            }
        }


        *this = newGrid4D;
    }

    void update()
    {
        Grid4D newGrid4D = *this;
        for (int w = lower + 1; w <= upper - 1; ++w)
        {
            for (int z = lower + 1; z <= upper - 1; ++z)
            {
                for (int y = lower + 1; y <= upper - 1; ++y)
                {
                    for (int x = lower + 1; x <= upper - 1; ++x)
                    {
                        State s = (*this)(x, y, z, w);
                        int active_count = 0;
                        
                        for (int w_offset = -1; w_offset <= 1; ++w_offset)
                        {
                            for (int z_offset = -1; z_offset <= 1; ++z_offset)
                            {
                                for (int y_offset = -1; y_offset <= 1; ++y_offset)
                                {
                                    for (int x_offset = -1; x_offset <= 1; ++x_offset)
                                    {
                                        if (x_offset == 0 && y_offset == 0 && z_offset == 0 && w_offset == 0)
                                        {
                                            continue;
                                        }
                                        active_count += (*this)(x + x_offset, y + y_offset, z + z_offset, w + w_offset) == Active;
                                    }
                                }
                            }
                        }

                        if (s == Active)
                        {
                            if (active_count == 2 || active_count == 3)
                            {}
                            else
                            {
                                newGrid4D(x, y, z, w) = Inactive;
                            }
                        }
                        else {
                            if (active_count == 3)
                            {
                                newGrid4D(x, y, z, w) = Active;
                            }
                        }
                    }
                }
            }
        }

        *this = newGrid4D;
    }

    int active()
    {
        int c = 0;
        for (const auto state : s)
        {
            c += state == Active;
        }

        return c;
    }

    void render()
    {
        for (int w = lower; w <= upper; ++w)
        {
            printf("w=%d\n", w);
            for (int z = lower; z <= upper; ++z)
            {
                printf("z=%d\n", z);
                for (int y = lower; y <= upper; ++y)
                {
                    for (int x = lower; x <= upper; ++x)
                    {
                    printf("%c", (*this)(x, y, z, w) == Active ? '#' : '.');
                    }
                    printf("\n");
                }
                printf("\n");
            }
        }
    }

    std::vector<State> s;
};

std::vector<State> getInput()
{
    std::string input = "##....#.\n#.#..#..\n...#....\n...#.#..\n###....#\n#.#....#\n.#....##\n.#.###.#";
    std::vector<State> state;

    for (const auto c : input)
    {
        switch (c)
        {
            case '#': {
                state.push_back(Active);
            } break;
            case '.': {
                state.push_back(Inactive);
            } break;
        }
    }

    return state;
}

void part1()
{
    const std::vector<State> z0 = getInput();
    const int size = std::sqrt(static_cast<double>(z0.size()));

    
    Grid3D grid(size);
    int old_lower = grid.lower;
    int old_upper = grid.upper;
    grid.grow();

    auto zBegin = z0.begin();
    for (int y = old_lower; y <= old_upper; ++y)
    {
        for (int x = old_lower; x <= old_upper; ++x)
        {
            grid(x, y, 0) = *zBegin++;
        }
    }

    for (int i = 0; i < 6; ++i)
    {
        grid.grow();
        grid.update();
    }

    printf("%d\n", grid.active());
}


void part2()
{
    const std::vector<State> z0 = getInput();
    const int size = std::sqrt(static_cast<double>(z0.size()));
    
    Grid4D grid(size);
    int old_lower = grid.lower;
    int old_upper = grid.upper;
    grid.grow();

    auto zBegin = z0.begin();
    for (int y = old_lower; y <= old_upper; ++y)
    {
        for (int x = old_lower; x <= old_upper; ++x)
        {
            grid(x, y, 0, 0) = *zBegin++;
        }
    }

    for (int i = 0; i < 6; ++i)
    {
        grid.grow();
        grid.update();
    }

    printf("%d\n", grid.active());
}

int main()
{
    part1();
    part2();
}