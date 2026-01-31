Welcome to my solution! All code written is my own.

To run: Ensure you have .NET installed on your machine, then navigate to this directory and
run ```dotnet test``` in the command line.

I used C#/.NET and MSTest as the testing framework, simply because I have experience in it.

Note that in MSTest, if a TestElement throws any exception, it is considered to have failed. 
Given that IWebDriver.FindElement... throws an exception if the element is not found, asserting
the existence of elements is not strictly necessary, but I have done so anyway.

I developed this on macOS. It should work on Windows, but I have not tested it.