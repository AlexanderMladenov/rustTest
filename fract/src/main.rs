extern crate image;
extern crate rand;

use rand::Rng;

static fract_seq: [usize; 5] = [1, 1, 0, 0, 1];

fn r(n: usize, x: f32, y: f32) -> f32 {

    let fract_r = n % fract_seq.len();
    let choice = fract_seq[fract_r];
    if choice == 1 { x } else { y }
}

fn compute_lyapunov_exp(x: f32, y: f32) -> f32 {
    
    let iterations = 1..500;
    
    let mut iter_seq = [0.0; 500];
    iter_seq[0] = 0.5;

    for i in &mut iter_seq[..] {
        i = 
    }

    0.0
}

fn main() {

    /*
float computeLyapunovExponent(const vec2& point)
{
    auto& p = point;

    float iter[ITERATIONS];
    iter[0] = 0.5f;

    for (int i = 1; i < ITERATIONS; i++)
    {
        iter[i] = r(i - 1, p) * iter[i - 1] * (1 - iter[i - 1]);
    }

    float result = 0.f;
    float ri[ITERATIONS];
    for (auto i = 1; i < ITERATIONS; i++)
    {
        ri[i] = abs(r(i, p));
    }

    float oneMinus2iterations[ITERATIONS];
    for (auto i = 1; i < ITERATIONS; i++) // vectorized
    {
        oneMinus2iterations[i] = abs((1 - (2 * iter[i])));
    }

    for (int i = 1; i < ITERATIONS; i++) // vectorized
    {
        result += logf(ri[i] * oneMinus2iterations[i]);
    }

    return (result / (float) ITERATIONS);
}

void renderFractalPixel(int x, int y)
{
    float a = (float) x / (float) (FRAME_RES);
    a = 2.f + a * 2.f;

    float b = (float) y / (float) (FRAME_RES);
    b = 2.f + b * 2.f;
    
    auto lyapunovExp = computeLyapunovExponent(vec2(a, b));
    if (lyapunovExp < 0)
    {
        frameR[x][y] = abs(lyapunovExp);
        frameG[x][y] = abs(lyapunovExp);
        frameB[x][y] = 0.f;
    }
    else if (lyapunovExp == 0.f)
    {
        frameR[x][y] = 1;
        frameG[x][y] = 1;
        frameB[x][y] = 0.f;
    }
    else
    {
        frameR[x][y] = 0;
        frameG[x][y] = lyapunovExp * 0.5f;
        frameB[x][y] = lyapunovExp;
    }
}

void renderFractalRegion(int xFrom, int xTo)
{
    for (auto x = xFrom; x < xTo; x++)
    {
        for (auto y = 0; y < FRAME_RES; y++)
        {
            renderFractalPixel(x, y);
        }
    }
    return;
}

std::vector<std::thread> threads;
void DoFractalThreaded()
{
    unsigned int numThreads = std::thread::hardware_concurrency();
    int delta = FRAME_RES / numThreads;
    for (int i = 0; i < numThreads; i++)
    {
        auto fun = std::bind(&renderFractalRegion, delta * i, delta * i + delta);
        threads.emplace_back(fun);
    }
}

typedef  std::chrono::time_point<std::chrono::high_resolution_clock> TimePoint;
template <typename T = std::chrono::seconds>
inline long long timePast(const TimePoint& start, const TimePoint& end)
{
    return std::chrono::duration_cast<T>(end - start).count();
}

#undef main
int main(int argc, char* argv[])
{
    if (argc == 2)
    {
        fractString = argv[1];
    }

    std::cout << "Lyapunov Fractal Renderer. Version: " << VERSION << std::endl;
    auto begin = std::chrono::high_resolution_clock::now();
    if (SDL_Init(SDL_INIT_EVERYTHING != 0))
    {
        std::cout << "SDL_init failed: " << SDL_GetError() << std::endl;
        waitForUserExit();
        SDL_DestroyWindow(m_Window);
        SDL_Quit();
        return -1;
    }

    m_Window = SDL_CreateWindow("LyapunovFract", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                FRAME_RES, FRAME_RES, SDL_WINDOW_OPENGL);

    if (!m_Window)
    {
        std::cout << "SDL_CreateWindow failed: " << SDL_GetError() << std::endl;
        waitForUserExit();
        SDL_DestroyWindow(m_Window);
        SDL_Quit();
        return -2;
    }

    m_Surface = SDL_GetWindowSurface(m_Window);

    if (!m_Surface)
    {
        std::cout << "SDL_GetWindowSurface failed: " << SDL_GetError() << std::endl;
        waitForUserExit();
        SDL_DestroyWindow(m_Window);
        SDL_Quit();
        return -3;
    }
    redShift = m_Surface->format->Rshift;
    greenShift = m_Surface->format->Gshift;
    blueShift = m_Surface->format->Bshift;

    SwapBuffers();
    auto end = std::chrono::high_resolution_clock::now();
    
    auto timeSec = timePast(begin, end);
    auto timeMili = timePast<std::chrono::milliseconds>(begin, end);
    std::cout << "Seting up SDL took: " << timeSec << " sec " << timeMili - (timeSec * 1000) << " milisec" << std::endl;

    std::cout << "Starting rendering with: " << std::thread::hardware_concurrency() << " threads." << std::endl;
    begin = std::chrono::high_resolution_clock::now();

    DoFractalThreaded();

    for (auto&& th : threads)
    {
        th.join();
    }
    end = std::chrono::high_resolution_clock::now();
    std::cout << "Rendering finished!" << std::endl;
    timeSec = timePast(begin, end);
    timeMili = timePast<std::chrono::milliseconds>(begin, end);

    std::stringstream ss;
    ss << "Time taken to render Lyapunov Fractal: " << timeSec << " sec " << timeMili - (timeSec * 1000) << " milisec";
    std::cout << "Time taken to render Lyapunov Fractal: " << timeSec << " sec " << timeMili - (timeSec * 1000) << " milisec" << std::endl;
    std::cout << "Fractal string used to generate image: " << fractString << std::endl;
    std::cout << ITERATIONS << " iterations per pixel" << std:: endl;

    SDL_SetWindowTitle(m_Window, ss.str().c_str());
    SwapBuffers();
    waitForUserExit();

    SDL_DestroyWindow(m_Window);
    SDL_Quit();
    return 0;
}
*/
}