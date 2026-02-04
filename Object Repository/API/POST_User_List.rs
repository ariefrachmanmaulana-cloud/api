<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_User_List</name>
   <tag></tag>
   <elementGuidId>deda21c1-881d-4d6e-8f8c-e579413c9660</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;API Katalon\&quot;,\n  \&quot;body\&quot;: \&quot;Ini adalah postingan pertama Arief Rachman Maulana\&quot;,\n  \&quot;userId\&quot;: 1\n}&quot;,
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
      <webElementGuid>36c5208c-ccd6-4ae9-a6a4-1d480d645c96</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>User-Agent</name>
      <type>Main</type>
      <value>Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36</value>
      <webElementGuid>405fd709-7f1b-4d1b-bcea-f02acef67a4a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.0.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// 1. Cek status code (Untuk POST sukses biasanya 201 Created)
WS.verifyResponseStatusCode(response, 201)

// 2. Verifikasi data yang baru dibuat muncul di respon
WS.verifyElementPropertyValue(response, 'title', &quot;API Katalon&quot;)

// 3. Pastikan server memberikan ID baru (biasanya 101 untuk JSONPlaceholder)
WS.verifyElementPropertyValue(response, 'id', 101)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
