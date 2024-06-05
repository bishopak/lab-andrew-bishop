<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get shoppingcart items</name>
   <tag></tag>
   <elementGuidId>177fd7c4-fd2a-49d6-a4bc-017e40dd5424</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Client-Id</name>
      <type>Main</type>
      <value>${clientId}</value>
      <webElementGuid>4ec4d77a-6b19-4560-8a29-9256a96135c4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseCartUrl}/shoppingcart/items?Type=${type}&amp;Discount=${discount}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9f7f22c0-47d5-4165-b9fc-c8a361c7dabf</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1a94677c-458b-45b2-a5fc-94b550e92818</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c6aa2afb-0441-47a7-8565-08dd99b2fa84</id>
      <masked>false</masked>
      <name>discount</name>
   </variables>
   <variables>
      <defaultValue>401</defaultValue>
      <description></description>
      <id>3a558dbc-19ec-4079-9891-d349df0e8fe1</id>
      <masked>false</masked>
      <name>expectedStatusCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
def variables = request.getVariables()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



int statusCode = response.getStatusCode();
String expectedBody = &quot;&quot;;

boolean expectedStatusCode = false;

if (statusCode == 200) {
	expectedStatusCode = true;
	expectedBody = '{&quot;basket&quot;:[{&quot;name&quot;:&quot;Milk&quot;,&quot;quantity&quot;:&quot;2&quot;,&quot;price&quot;:&quot;$4&quot;},{&quot;name&quot;:&quot;Bread&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;price&quot;:&quot;$5&quot;},{&quot;name&quot;:&quot;Eggs&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;price&quot;:&quot;$6.5&quot;}]}'	
}
else if (statusCode == 401) {
	expectedStatusCode = true;
	expectedBody = '{&quot;message&quot;:&quot;Unauthorized request! Client-Id is missing or invalid.&quot;}';  
}
else if (statusCode == 400) {
	expectedStatusCode = true;
	expectedBody = '{&quot;message&quot;:&quot;Invalid request! Parameters are missing or invalid.&quot;}';
}
else if (statusCode == 502) {
	expectedStatusCode = true;
}

//not going verify the response body for 502
if (statusCode != 502) {
	assertThat(response.getResponseText()).isEqualTo(expectedBody);
}

assertThat(expectedStatusCode).isEqualTo(true);

assertThat(response.getStatusCode()).isEqualTo(variables.get('expectedStatusCode'))



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
