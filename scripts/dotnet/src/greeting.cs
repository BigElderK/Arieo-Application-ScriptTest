using System.Runtime.CompilerServices;
using System.Diagnostics;



namespace Arieo.Interface.Sample
{
    class ISample
    {
        public const ulong INTERFACE_ID = 9421138728380728032UL;
        public const ulong INTERFACE_CHECKSUM = 8375695449026144995UL;
        
        private readonly ulong m_handle;

        public ISample(ulong handle)
        {
            m_handle = handle;
        }

        public Int32 doSomthing_1(Int32 i1, Int32 i2)
        {
            return ApplicationWorld.wit.imports.arieo.sample.ISampleInterop.DoSomthing1(m_handle, i1, i2);
        }

        public Int32 doSomthing_2(Int32 i1, Int32 i2)
        {
            return ApplicationWorld.wit.imports.arieo.sample.ISampleInterop.DoSomthing2(m_handle, i1, i2);
        }

    }
}

namespace ApplicationWorld.wit.exports.arieo.application
{
    public class GuestImpl : IGuest
    {
        public static string Greeting()
        {
            // Call to a noinline method should force a real function call
            var result = GreetingCore();
            return result;
        }
        
        private static string GreetingCore()
        {
            // You can call the host's log function if needed
            ApplicationWorld.wit.imports.arieo.application.HostInterop.Log("C# guest Greeting() called.");
            
            // Call module manager create-instance to get ISample instance handle
            // Use the ISample wrapper class
            Arieo.Interface.Sample.ISample sample = new Arieo.Interface.Sample.ISample(ApplicationWorld.wit.imports.arieo.module.ModuleManagerInterop.GetInterface(
                unchecked((ulong)Arieo.Interface.Sample.ISample.INTERFACE_ID),
                unchecked((ulong)Arieo.Interface.Sample.ISample.INTERFACE_CHECKSUM),
                "")
            );

            Int32 result1 = sample.doSomthing_1(10, 20);
            ApplicationWorld.wit.imports.arieo.application.HostInterop.Log($"do-somthing1(10, 20) = {result1}");
            
            Int32 result2 = sample.doSomthing_2(5, 15);
            ApplicationWorld.wit.imports.arieo.application.HostInterop.Log($"do-somthing2(5, 15) = {result2}");
            
            return $"Hello from .NET! Results: {result1}, {result2}";
        }
    }
}
