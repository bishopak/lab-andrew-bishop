<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get vehicle data</name>
   <tag></tag>
   <elementGuidId>b5e3e89c-eb43-4c97-8885-6824048eb185</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseVehicleUrl}/data/${registration}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'WATERBUFFALO670'</defaultValue>
      <description></description>
      <id>43d6428d-573e-4a35-8f1b-152c92e0467c</id>
      <masked>false</masked>
      <name>registration</name>
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


int statusCode = response.getStatusCode();
boolean verified = false;

if  (statusCode == 200) {
	System.out.println(response.getResponseText());
	String jsonPass =
		&quot;&quot;&quot;
		{
			&quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
			&quot;year&quot;: {
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;make&quot;: {
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;model&quot;: {
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;transmission&quot;:{
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;odometer&quot;:{
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;rego&quot;:{
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;state&quot;:{
				&quot;type&quot;:&quot;string&quot;
			},
			&quot;addressModel&quot;: {
				&quot;type&quot;:&quot;array&quot;,
				&quot;items&quot;: {
					&quot;required&quot;: [
						&quot;addressType&quot;,
						&quot;address1&quot;,
						&quot;address2&quot;,
						&quot;state&quot;,
						&quot;postcode&quot;,
						&quot;country&quot;
					],
					&quot;properties&quot;:  {
						&quot;addressType&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;address1&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;address2&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;state&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;postcode&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;country&quot;: {
							&quot;type&quot;:&quot;string&quot;
						}
					}
				}
			},
			&quot;owner&quot;: {
				&quot;type&quot;:&quot;array&quot;,
				&quot;items&quot;: {
					&quot;required&quot;: [
						&quot;fullName&quot;,
						&quot;dob&quot;,
						&quot;isCurrentOwner&quot;,
						&quot;license&quot;
					],
					&quot;properties&quot;:  {
						&quot;fullName&quot;: {
							&quot;type&quot;:&quot;string&quot;
						},
						&quot;dob&quot;: {
							&quot;type&quot;:&quot;string&quot;,
							&quot;pattern&quot;:&quot;^([0-2][0-9]|3[0-1])\\/(0[1-9]|1[0-2])\\/[0-9]{4}\$&quot;
						},
						&quot;isCurrentOwner&quot;: {
							&quot;type&quot;:&quot;boolean&quot;
						},
						&quot;license&quot;: {
							&quot;type&quot;:&quot;string&quot;
						}
					}
				}
			}
		}
		&quot;&quot;&quot; 
		
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	verified = true;

} else if (statusCode == 404) {
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
	
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	assertThat(successful).isEqualTo(true);
	
	assertThat(response.getResponseText()).isEqualTo('{&quot;message&quot;:&quot;No vehicle found!&quot;}');
	verified = true;
}

assertThat(statusCode).isIn([200,404])
assertThat(verified).isEqualTo(true);
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
