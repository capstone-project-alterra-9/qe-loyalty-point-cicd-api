<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TC_Update_Transactions_01</name>
   <tag></tag>
   <elementGuidId>8534a90a-6255-4ac4-bd01-4b7a6f7272de</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;paymentMethod\&quot; : \&quot;redeem\&quot;,\n    \&quot;userID\&quot; : \&quot;93615ef0-d109-4fee-870d-2d79bdc516b2\&quot;,\n    \&quot;productID\&quot; : \&quot;4ac4afdb-5c97-474f-a8d8-0184491b89ee\&quot;,\n    \&quot;identifierNum\&quot; : \&quot;081234567890\&quot;,\n    \&quot;status\&quot; : \&quot;success\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>12049ef3-e359-4ad6-bf68-4b8bd9b2c10b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6ImFkbWlud2ViMTIzQGdtYWlsLmNvbSIsImV4cCI6MTY3MTg2MDUyNywidXNlcm5hbWUiOiJhZG1pbndlYmJhcnUifQ.a7VZgDn_NYw7y8BfuL3cxaItHiKyPClTRAdsUFVTJR4</value>
      <webElementGuid>f53f8805-c776-4fee-91dd-bcf461bee4f8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/api/auth/transactions/bcc806e1-8e61-44fa-939d-7ee54cbdbeb1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
