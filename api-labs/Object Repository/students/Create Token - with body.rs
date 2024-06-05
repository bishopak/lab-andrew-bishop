<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Token - with body</name>
   <tag></tag>
   <elementGuidId>5b1b5e4f-ca87-4e10-b3fc-f18f16ef2c2e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${payload}&quot;,
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
      <webElementGuid>c2eddc55-eb66-4487-94c1-c4c7d39eba0e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Client-Id</name>
      <type>Main</type>
      <value>${clientId}</value>
      <webElementGuid>440a8172-183a-44b2-9ebb-0fbe431965f1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseStudentUrl}/token</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.userName</defaultValue>
      <description></description>
      <id>cf7f9a16-5b40-457e-ad0f-c2f7f7f31e9f</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <variables>
      <defaultValue>'quality-engineering'</defaultValue>
      <description>login key? need better name</description>
      <id>8828e91a-a48c-47a9-af96-4c81e911458e</id>
      <masked>false</masked>
      <name>Key</name>
   </variables>
   <variables>
      <defaultValue>'{&quot;key&quot;:&quot;${Key}&quot;}'</defaultValue>
      <description></description>
      <id>d1a0fa78-f74d-4c2d-ba70-e7c862ff039f</id>
      <masked>false</masked>
      <name>payload</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

int statusCode = response.getStatusCode()
boolean validated = false

if (statusCode == 200) {
	String jsonPass =
	&quot;&quot;&quot;
	{
	  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
	  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
	  &quot;token&quot;: {
	   		&quot;type&quot;: &quot;string&quot;,
	  		&quot;description&quot;: &quot;token that is created&quot;
	   }
	}
	&quot;&quot;&quot;
	//do I need to both?
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	validated = true;

	
	//this mmakes me sad, need this variable set by default by the looks of things
	//could there be a better way to do this?
	System.out.println(&quot;token before: &quot; + GlobalVariable.authToken);
	GlobalVariable.authToken = WS.getElementPropertyValue(response, &quot;token&quot;);
	System.out.println(&quot;token after: &quot; + GlobalVariable.authToken);
	
} else if (statusCode == 400) {
	String jsonPass =
	&quot;&quot;&quot;
	{
	  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
	  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
	  &quot;error&quot;: {
	   		&quot;type&quot;: &quot;string&quot;,
	  		&quot;description&quot;: &quot;error string&quot;
	   }
	}
	&quot;&quot;&quot;
	//do I need to both?
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	
	
	String expectedPayload = '{&quot;error&quot;:&quot;Invalid key!&quot;}'
	assertThat(response.getResponseText()).isEqualTo(expectedPayload);
	validated = true;

} else if (statusCode == 502) {
	validate = true;
}


assertThat(validated).isEqualTo(true);</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
