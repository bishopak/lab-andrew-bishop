<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>View students</name>
   <tag></tag>
   <elementGuidId>2d5fc9eb-cd7f-4b2b-bfef-85df4ad1fb7b</elementGuidId>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${authToken}</value>
      <webElementGuid>abf48925-60a6-4c35-8263-8c38f918e181</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Client-Id</name>
      <type>Main</type>
      <value>${clientId}</value>
      <webElementGuid>bf8f2b9c-3a1f-4514-9528-f792e605f18e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseStudentUrl}/students</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.authToken</defaultValue>
      <description></description>
      <id>be0603fb-d935-45a3-a9da-a35171959a5b</id>
      <masked>false</masked>
      <name>authToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userName</defaultValue>
      <description></description>
      <id>ad7bc339-c0d6-4d01-afc3-75b0daefd469</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

int statusCode = response.getStatusCode()
boolean validated = false

if (statusCode == 200) {
	System.out.println(response.getResponseText());
	String jsonPass = &quot;&quot;&quot;
	{
		&quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
		&quot;students&quot;: {
			&quot;type&quot;:&quot;array&quot;,
			&quot;items&quot;: {
				&quot;required&quot;: [
					&quot;id&quot;,
					&quot;firstName&quot;,
					&quot;lastName&quot;,
					&quot;nationality&quot;,
					&quot;dateOfBirth&quot;,
					&quot;email&quot;,
					&quot;mobileNumber&quot;
				],
				&quot;properties&quot;: {
					&quot;id&quot;: {
						&quot;type&quot;: &quot;string&quot;,
						&quot;format&quot;: &quot;uuid&quot;
					},
					&quot;firstName&quot;: {
						&quot;type&quot;: &quot;string&quot;
					},
					&quot;lastName&quot;: {
						&quot;type&quot;: &quot;string&quot;
					},
					&quot;nationality&quot;: {
						&quot;type&quot;: &quot;string&quot;
					},
					&quot;dateOfBirth&quot;: {
						&quot;type&quot;: &quot;string&quot;,
						&quot;pattern&quot;: &quot;^([0-2][0-9]|3[0-1])\\/(0[1-9]|1[0-2])\\/[0-9]{4}\$&quot;
					},
					&quot;email&quot;: {
						&quot;type&quot;: &quot;string&quot;,
						&quot;format&quot;: &quot;email&quot;
					},
					&quot;mobileNumber&quot;: {
						&quot;type&quot;: &quot;string&quot;,
						&quot;pattern&quot;:&quot;^[+61|0]{1}[4-5]{1}[0-9]{8}\$&quot;
					}
				}
			}
		}	
	}
	&quot;&quot;&quot;
		
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	validated = true;

	
	//this mmakes me sad, need this variable set by default by the looks of things
	//could there be a better way to do this?
	GlobalVariable.authToken = WS.getElementPropertyValue(response, &quot;token&quot;);
	
	
} else if (statusCode == 401) {
	String jsonPass =
	&quot;&quot;&quot;
	{
	  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
	  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
	  &quot;message&quot;: {
	   		&quot;type&quot;: &quot;string&quot;,
	  		&quot;description&quot;: &quot;error string&quot;
	   }
	}
	&quot;&quot;&quot;
	//do I need to both?
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	
	
	String expectedPayload = '{&quot;message&quot;:&quot;Unauthorized request.&quot;}'
	assertThat(response.getResponseText()).isEqualTo(expectedPayload);
	validated = true;
	
}

assertThat(validated).isEqualTo(true)


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
