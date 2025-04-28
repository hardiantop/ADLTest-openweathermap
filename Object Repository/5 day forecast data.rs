<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5 day forecast data</name>
   <tag></tag>
   <elementGuidId>58587ce8-b8ce-4e71-8049-ee8c6e8c1985</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://api.openweathermap.org/data/2.5/forecast?lat=-6.2474&amp;lon=106.8061&amp;appid=f60bf788045c8eb9166604c6f6796f27</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>8fb48a9e-d89c-4a82-8445-2d4201c026e7</id>
      <name>cod</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\albra\Katalon Studio\ADLTest-openweathermap\OpenWeatherAPI5DaysForecastSchema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>e9a7ac07-3f3b-40bb-8baa-3c9be8c2940f</id>
      <name>temp</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\albra\Katalon Studio\ADLTest-openweathermap\OpenWeatherAPI5DaysForecastSchema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>dad17764-ffc3-491a-ba1c-97bd39ea74a6</id>
      <name>weather</name>
      <type>JSON_SCHEMA</type>
      <dataType>URL</dataType>
      <target>RESPONSE</target>
      <data>https://github.com/hardiantop/ADLTest-openweathermap/blob/5af13242422da7f81de1577f7a303f15fca6cde3/OpenWeatherAPI5DaysForecastSchema</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
