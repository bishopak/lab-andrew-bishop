<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create student</name>
   <tag></tag>
   <elementGuidId>55d9076b-db5d-443e-a1af-d9949562dde7</elementGuidId>
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
      <webElementGuid>cb9a5c16-3119-4d65-93b8-bf587a694da3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Client-Id</name>
      <type>Main</type>
      <value>${clientId}</value>
      <webElementGuid>51444829-b7d0-4d8e-8ef7-96335608a40d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${authToken}</value>
      <webElementGuid>dce0c5b2-91b6-4fcf-a356-bace64296eef</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseStudentUrl}/student/create</restUrl>
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
      <id>2d066417-5c83-4eae-866a-47af0b548cbe</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authToken</defaultValue>
      <description></description>
      <id>5e0d3aee-765f-4bff-9699-b8a736271f10</id>
      <masked>false</masked>
      <name>authToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ee1596df-814d-4704-b978-8f662fe21830</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8149f042-be5a-4b28-9888-13f2590f8159</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>15b49044-d69b-4d8a-904a-77c6ac6672ff</id>
      <masked>false</masked>
      <name>nationality</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9835067b-9b5d-4ae0-b675-2a40df2f9648</id>
      <masked>false</masked>
      <name>dateOfBirth</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ba91df0f-b6cc-461f-9998-949b18ffcade</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>51464830-ccf5-4322-bc05-750e47cd598e</id>
      <masked>false</masked>
      <name>mobileNumber</name>
   </variables>
   <variables>
      <defaultValue>'{&quot;firstName&quot;: &quot;${firstName}&quot;,&quot;lastName&quot;: &quot;${lastName}&quot;,&quot;nationality&quot;: &quot;${nationality}&quot;,&quot;dateOfBirth&quot;: &quot;${dateOfBirth}&quot;,&quot;email&quot;: &quot;${email}&quot;,&quot;mobileNumber&quot;: &quot;${mobileNumber}&quot;}'</defaultValue>
      <description></description>
      <id>52e28e19-5548-414e-935e-ca2b85ca045c</id>
      <masked>false</masked>
      <name>payload</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*&#xd;
&#xd;
import com.kms.katalon.core.testobject.RequestObject&#xd;
import com.kms.katalon.core.testobject.ResponseObject&#xd;
import com.kms.katalon.core.testobject.TestObjectProperty&#xd;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS&#xd;
import com.kms.katalon.core.webservice.verification.WSResponseManager&#xd;
&#xd;
import groovy.json.JsonSlurper&#xd;
import internal.GlobalVariable as GlobalVariable&#xd;
&#xd;
&#xd;
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()&#xd;
&#xd;
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()&#xd;
&#xd;
&#xd;
&#xd;
int statusCode = response.getStatusCode()&#xd;
boolean validated = false&#xd;
&#xd;
&#xd;
if (statusCode == 200 || statusCode == 201) {&#xd;
	String jsonPass =&#xd;
		&quot;&quot;&quot;&#xd;
		{&#xd;
			&quot;\$schema&quot;: &quot;https://example.com/person.schema.json&quot;,&#xd;
		  	&quot;\$id&quot;: {&#xd;
				&quot;type&quot;: &quot;string&quot;,&#xd;
				&quot;format&quot;: &quot;uuid&quot;,&#xd;
	  			&quot;description&quot;: &quot;uniquie id string&quot;&#xd;
			},&#xd;
			&quot;message&quot;: {&#xd;
				&quot;type&quot;: &quot;string&quot;,&#xd;
	  			&quot;description&quot;: &quot;response message&quot;&#xd;
			}&#xd;
		}&#xd;
		&quot;&quot;&quot;&#xd;
		&#xd;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)&#xd;
	assertThat(successful).isEqualTo(true);&#xd;
	validated = true;&#xd;
	WS.verifyElementPropertyValue(response, 'message', &quot;New student was created successfully!&quot;)&#xd;
	&#xd;
} else if (statusCode == 401) {&#xd;
	String jsonPass =&#xd;
	&quot;&quot;&quot;&#xd;
	{&#xd;
	  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,&#xd;
	  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,&#xd;
	  &quot;message&quot;: {&#xd;
	   		&quot;type&quot;: &quot;string&quot;,&#xd;
	  		&quot;description&quot;: &quot;error string&quot;&#xd;
	   }&#xd;
	}&#xd;
	&quot;&quot;&quot;&#xd;
	//do I need to both?&#xd;
	&#xd;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)&#xd;
	assertThat(successful).isEqualTo(true);&#xd;
	&#xd;
	&#xd;
	String expectedPayload = '{&quot;message&quot;:&quot;Unauthorized request.&quot;}'&#xd;
	assertThat(response.getResponseText()).isEqualTo(expectedPayload);&#xd;
	validated = true;	&#xd;
} else if (statusCode == 400) {&#xd;
	String jsonPass =&#xd;
	&quot;&quot;&quot;&#xd;
	{&#xd;
	  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,&#xd;
	  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,&#xd;
	  &quot;message&quot;: {&#xd;
	   		&quot;type&quot;: &quot;string&quot;,&#xd;
	  		&quot;description&quot;: &quot;error string&quot;&#xd;
	   }&#xd;
	}&#xd;
	&quot;&quot;&quot;&#xd;
	//do I need to both?&#xd;
	&#xd;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)&#xd;
	assertThat(successful).isEqualTo(true);&#xd;
	&#xd;
	String expectedPayload = '{&quot;message&quot;:&quot;ERROR! Student exists!&quot;}'&#xd;
	assertThat(response.getResponseText()).isEqualTo(expectedPayload);&#xd;
	validated = true;&#xd;
} else if (statusCode == 502) {&#xd;
	validated = true;&#xd;
}&#xd;
&#xd;
&#xd;
assertThat(validated).isEqualTo(true)&#xd;
&#xd;
&#xd;



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
