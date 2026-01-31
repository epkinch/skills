using Microsoft.VisualStudio.TestTools.UnitTesting;
using OpenQA.Selenium;
using OpenQA.Selenium.Chrome;
using OpenQA.Selenium.Support.UI;
using System;
using System.Collections.Generic;
using System.Threading;
using System.IO;


[TestClass]
public class AutomatedUITests 
{
    private IWebDriver driver = null!;

    [TestInitialize]
    public void TestInitialize()
    {
        string filePath = Path.Combine(Directory.GetCurrentDirectory(), "index.html");
        string fileUrl = "file:///" + filePath.Replace("\\", "/");
        driver = new ChromeDriver();
        driver.Navigate().GoToUrl(fileUrl);
    }

    [TestCleanup]
    public void TestCleanup()
    {
        driver.Quit();
    }

    [TestMethod]
    public void Test1() 
    {
        IWebElement test1 = driver.FindElement(By.Id("test-1-div"));
        string inputEmailId = "inputEmail";
        Assert.IsTrue(canElementBeFound(test1, By.Id(inputEmailId)), "email input cannot be found");
        string inputPasswordId = "inputPassword";
        Assert.IsTrue(canElementBeFound(test1, By.Id(inputPasswordId)), "password input cannot be found");
        string signInButtonClassName = "btn-primary";
        Assert.IsTrue(canElementBeFound(test1, By.ClassName(signInButtonClassName)), "sign in button cannot be found");

        IWebElement emailInput = test1.FindElement(By.Id(inputEmailId));
        emailInput.Click();
        emailInput.SendKeys("test@ualberta.ca");

        IWebElement passwordInput = test1.FindElement(By.Id(inputPasswordId));
        passwordInput.Click();
        passwordInput.SendKeys("1234");

        Thread.Sleep(1000);
    }

    [TestMethod]
    public void Test2()
    {
        IWebElement test2 = driver.FindElement(By.Id("test-2-div"));
        IWebElement listGroup = test2.FindElement(By.ClassName("list-group"));
        IReadOnlyList<IWebElement> listItems = listGroup.FindElements(By.TagName("li"));

        Assert.AreEqual(listItems.Count, 3, "an unexpected number of list items found");

        List<string> seperatedText = seperateText(listItems[1]);
        Assert.AreEqual(seperatedText[0], "List Item 2", "list item 2 does not contain expected text");
        Assert.AreEqual(seperatedText[1], "6", "badge 2 does not contain expected number");
    }

    [TestMethod]
    public void Test3()
    {
        IWebElement test3 = driver.FindElement(By.Id("test-3-div"));
        IWebElement dropdownToggle = test3.FindElement(By.ClassName("dropdown-toggle"));

        Assert.AreEqual(dropdownToggle.Text, "Option 1", "Option 1 is not initially selected");

        dropdownToggle.Click();
        IWebElement option3 = test3.FindElements(By.ClassName("dropdown-item"))[2];
        option3.Click();

        Thread.Sleep(1000);
    }

    [TestMethod]
    public void Test4()
    {
        IWebElement test4 = driver.FindElement(By.Id("test-4-div"));

        IWebElement firstButton = test4.FindElement(By.ClassName("btn-primary"));
        Assert.IsTrue(firstButton.Enabled, "first button disabled");

        IWebElement secondButton = test4.FindElement(By.ClassName("btn-secondary"));
        Assert.IsFalse(secondButton.Enabled, "second button enabled");

    }

    [TestMethod]
    public void Test5()
    {
        IWebElement test5 = driver.FindElement(By.Id("test-5-div"));

        WebDriverWait wait = new WebDriverWait(driver, TimeSpan.FromSeconds(20));
        IWebElement button = wait.Until(d => 
        {
            IWebElement button = d.FindElement(By.Id("test5-button"));
            if (button.Displayed && button.Enabled)
            {
                return button;
            }
            return null;
        });
        button.Click();

        IWebElement success = driver.FindElement(By.Id("test5-alert"));
        Assert.AreEqual(success.Text, "You clicked a button!", "success message not displayed");

        Assert.IsFalse(button.Enabled, "button remains enabled after clicking");
        Thread.Sleep(1000);
    }

    [TestMethod]
    public void Test6()
    {
        IWebElement test6 = driver.FindElement(By.Id("test-6-div"));
        IWebElement grid = test6.FindElement(By.TagName("tbody"));

        string bottomRight = findCellValue(grid, 2, 2);
        Assert.AreEqual(bottomRight, "Ventosanzap");
    }

    #region Helper Functions

    public bool canElementBeFound(IWebElement parent, By selector)
    {
        try
        {
            parent.FindElement(selector);
        }
        catch
        {
            return false;
        }
        return true;
    }

    // not recursive, and not foolproof
    public List<String> seperateText(IWebElement parent)
    {
        int original_length = parent.Text.Length;
        string toBeRemoved = "";
        List<String> seperatedText = [""];
        IList<IWebElement> children = parent.FindElements(By.XPath("./*"));
        for (int i = 0; i < children.Count; i++)
        {
            toBeRemoved += children[i].Text;
            seperatedText.Add(children[i].Text);
        }

        seperatedText[0] = parent.Text.Replace(toBeRemoved, "").Trim();

        if (original_length - seperatedText[0].Length - 1 > toBeRemoved.Length) // Trimming the space reduces length by 1
        {
            throw new InvalidOperationException("the parent's text mirrors the children's text");
        }

        return seperatedText;
    }

    public string findCellValue(IWebElement grid, int x, int y)
    {
        IReadOnlyList<IWebElement> cells = grid.FindElements(By.TagName("td"));
        return cells[x + 3 * y].Text;
    }

    #endregion

}