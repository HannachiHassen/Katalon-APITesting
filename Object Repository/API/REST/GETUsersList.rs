<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>get data of users</description>
   <name>GETUsersList</name>
   <tag></tag>
   <elementGuidId>99d5c112-b75b-4c70-aa03-febd85e5c66f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.6</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>6a909f8d-b520-4909-aef8-20d244ddcaf7</id>
      <name>ValidateResponse</name>
      <type>AUTO_DETECT</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>UsersJsonResponseSchema.txt</data>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[4].first_name', &quot;George&quot;)

GlobalVariable.FIRST_NAME = WS.getElementPropertyValue(response, 'data[4].first_name')

println 'Global Variable First_Name is : ' +GlobalVariable.FIRST_NAME</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
