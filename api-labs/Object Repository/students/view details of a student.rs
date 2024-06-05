<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>view details of a student</name>
   <tag></tag>
   <elementGuidId>ae7dc2cc-e7af-4921-88cc-09c2caa6145b</elementGuidId>
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
      <webElementGuid>8adf2fb3-db09-49e4-b937-70390d20da95</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${authToken}</value>
      <webElementGuid>6536c826-a980-4dbe-8654-b96b206e8339</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseStudentUrl}/student/${studentId}/details</restUrl>
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
      <id>926b3c9a-fc7b-4c92-b273-80cf935e9013</id>
      <masked>false</masked>
      <name>authToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3689a206-36b3-4f56-9c62-0c3e5cc05561</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e860dbee-6607-4638-bb5d-294abc3dfbdc</id>
      <masked>false</masked>
      <name>studentId</name>
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
	//todo: imrove regex for pattern for phone as its just for australian 
	//      and only targets landlines
	String jsonPass = 
		&quot;&quot;&quot;
		{
		  	&quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
		  	&quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
			&quot;type&quot;: &quot;object&quot;,
			&quot;properties&quot;: {
				&quot;data&quot; : {
					&quot;type&quot;: &quot;object&quot;,
					&quot;properties&quot;: {
						&quot;id&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;format&quot;: &quot;uuid&quot;,
						  	&quot;description&quot;: &quot;uniquie id string&quot;
						},
						&quot;firstName&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;description&quot;: &quot;students first name&quot;
						},
						&quot;lastName&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;description&quot;: &quot;students last name&quot;
						},
						&quot;nationality&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;description&quot;: &quot;students nationality&quot;
						},
						&quot;dateOfBirth&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;pattern&quot;: &quot;^([0-2][0-9]|3[0-1])\\/(0[1-9]|1[0-2])\\/[0-9]{4}\$&quot;,
							&quot;description&quot;: &quot;students birthday&quot;
						},
						&quot;email&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;format&quot;: &quot;email&quot;,
							&quot;description&quot;: &quot;students email address&quot;
						},
						&quot;mobileNumber&quot;: {
							&quot;type&quot;: &quot;string&quot;,
							&quot;pattern&quot;:&quot;^[+61|0]{1}[4-5]{1}[0-9]{8}&quot;,
							&quot;description&quot;: &quot;students mobile number&quot;
						}
					}
				}
			}
		}
		&quot;&quot;&quot;
		
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	validated = true;
	
} else if (statusCode == 401 || statusCode == 404) {
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
	
	if (statusCode == 401) {
		expectedPayload = '{&quot;message&quot;:&quot;Unauthorized request.&quot;}'
	} else if (statusCode == 404) {
		expectedPayload = '{&quot;message&quot;:&quot;No student found!&quot;}'
	}
	assertThat(response.getResponseText()).isEqualTo(expectedPayload);
	validated = true;
	
} else if (statusCode == 502) {
	validated = true
}


assertThat(validated).isEqualTo(true)


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
