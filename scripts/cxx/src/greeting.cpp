// greeter_impl.cpp
#include "application_cpp.h"

namespace Arieo::Interface::Sample
{
    class ISample
    {
    protected:
        std::uint64_t m_handle;

    public:
        static constexpr std::uint64_t m_interface_id = 9421138728380728032ULL;
        static constexpr std::uint64_t m_interface_checksum = 8375695449026144995ULL;

        ISample(std::uint64_t handle) : m_handle(handle) {}

        std::int32_t doSomthing_1(std::int32_t i1, std::int32_t i2)
        {
            return ::arieo::sample::i_sample::DoSomthing1(m_handle, i1, i2);
        }

        std::int32_t doSomthing_2(std::int32_t i1, std::int32_t i2)
        {
            return ::arieo::sample::i_sample::DoSomthing2(m_handle, i1, i2);
        }

    };

}

wit::string exports::arieo::application::guest::Greeting()
{
    // Test local variables for debugging
    int count = 42;
    const char* name = "Arieo";
    double pi = 3.14159;
    bool is_debug = true;
    int numbers[3] = {1, 2, 3};

    // Use wit::string instead of std::string (no WASI dependency)
    wit::string a = wit::string::from_view("Test");
    wit::string b = wit::string::from_view("Copy");
    wit::string c = wit::string::from_view("Move");

    // You can call the host's log function if needed
    ::arieo::application::host::Log("C++ guest Greeting() called.");
    ::arieo::application::host::Log(name);
    
    Arieo::Interface::Sample::ISample sample = ::arieo::module::module_manager::GetInterface(
        Arieo::Interface::Sample::ISample::m_interface_id, 
        Arieo::Interface::Sample::ISample::m_interface_checksum, 
        ""
    );
    sample.doSomthing_1(5, 7);
    sample.doSomthing_2(5, 7);
    
    // ::arieo::sample::i_sample::do-somthing-1(10, 20); // Example call to host module function
    

    if (is_debug)
    {
        ::arieo::application::host::Log("Debug mode enabled");
    }
    
    return wit::string::from_view("Hello from C++ guest implementation!");
}
