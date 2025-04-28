# Open Weather Map Test Steps
This document contains instructions on how to run the Katalon Studio Test Suite for the Open Weather Map open API.

The API is available at `https://openweathermap.org/api`

This Test Suite contains 2 API to test:
1. 5 days forecast data
2. Current air pollution

For more detailed content, you can check the object repositories at: https://github.com/hardiantop/ADLTest-openweathermap/tree/master/Object%20Repository.

## Dependencies
To run the tests, you have to prepare some components below on your Katalon Studio machine:
1. Download and save both the JSON Schemas located in https://github.com/hardiantop/ADLTest-openweathermap/tree/master to your local machine into this path: C:\Schema
   ![001](https://github.com/user-attachments/assets/7190341b-e028-46e4-a344-e8a2566addbc)
2. Get the Open Weather Map API Key and change the appid on each repository with your private API key. This is optional because the test suite already has an API key, but if you don't use your private key, the limit of 1000 free API calls per day will be shared.

## Instructions
This instruction assumes you have experience using Katalon Studio.
1. Pull request the test suite.
2. Open the Test Suite (Get Weather Info)
3. Open the **Run** option, and select **Web Service**.
![002](https://github.com/user-attachments/assets/9a1ac1fc-bd6f-4121-9f59-dfb00f15734b)
4. Wait until the test finishes and generate a report.
![Screenshot 2025-04-28 155957](https://github.com/user-attachments/assets/b74db447-d007-46ec-a1ce-8aaf24fbc3b3)

## Getting Test Reports
You can get the test result on your local report folder.
1. Open the report tab.
2. Right-click on the report.
3. Select **Open containing folder**. The report is available in PDF, HTML, and CSV.

![Screenshot 2025-04-28 1550101](https://github.com/user-attachments/assets/eb273628-ac28-47a8-972e-0f171bded15e)

-End of document-
