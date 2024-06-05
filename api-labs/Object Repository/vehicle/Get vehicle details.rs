<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get vehicle details</name>
   <tag></tag>
   <elementGuidId>3441d970-f0f2-4f5c-8e4d-660468852c2c</elementGuidId>
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
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>50848056-9b22-4ce1-901d-7a97a6cf620b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseVehicleUrl}/vehicle/${registration}/details</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'CRICKET558'</defaultValue>
      <description></description>
      <id>921c15c0-cd8c-47b2-8d60-76a31b33a57d</id>
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
			&quot;message&quot;: &quot;ok&quot;,
			&quot;data&quot;: {
				&quot;type&quot;:&quot;object&quot;,
				&quot;vehicle&quot; : {
					&quot;type&quot;:&quot;object&quot;,
					&quot;year&quot;:{&quot;type&quot;:&quot;string&quot;},
				    &quot;make&quot;:{&quot;type&quot;:&quot;string&quot;},
				    &quot;model&quot;:{&quot;type&quot;:&quot;string&quot;},
				    &quot;transmission&quot;:{&quot;type&quot;:&quot;string&quot;},
				    &quot;odometer&quot;:{&quot;type&quot;:&quot;string&quot;}
				},
				&quot;registration&quot;: {
					&quot;type&quot;:&quot;object&quot;,
					&quot;registrationNumber&quot;:{&quot;type&quot;:&quot;string&quot;},
					&quot;state&quot;:{&quot;type&quot;:&quot;string&quot;},
					&quot;address&quot;:{
						&quot;type&quot;: &quot;array&quot;,
						&quot;items&quot;: {
							&quot;required&quot;:[&quot;addressType&quot;,&quot;addressLine1&quot;,&quot;addressLine2&quot;,&quot;postcode&quot;,&quot;state&quot;,&quot;country&quot;],
							&quot;properties&quot;:{
								&quot;addressType&quot;:{&quot;type&quot;:&quot;string&quot;},
					         	&quot;addressLine1&quot;:{&quot;type&quot;:&quot;string&quot;},
					          	&quot;addressLine2&quot;:{&quot;type&quot;:&quot;string&quot;},
					          	&quot;postcode&quot;:{&quot;type&quot;:&quot;string&quot;},
					          	&quot;state&quot;:{&quot;type&quot;:&quot;string&quot;},
					          	&quot;country&quot;:{&quot;type&quot;:&quot;string&quot;}
							}
						}
					}
				},
				&quot;owner&quot;: {
					&quot;type&quot;:&quot;array&quot;,
					&quot;items&quot;: {
						&quot;required&quot;:[&quot;fullName&quot;,&quot;dateOfBirth&quot;,&quot;driverLicense&quot;,&quot;isCurrentOwner&quot;],
						&quot;properties&quot;:{
					        &quot;fullName&quot;:{&quot;type&quot;:&quot;string&quot;},
					        &quot;dateOfBirth&quot;:{
								&quot;type&quot;:&quot;string&quot;,
								&quot;pattern&quot;:&quot;^([0-2][0-9]|3[0-1])\\/(0[1-9]|1[0-2])\\/[0-9]{4}\$&quot;
							},
					        &quot;driverLicense&quot;:{&quot;type&quot;:&quot;string&quot;},
					        &quot;isCurrentOwner&quot;:{&quot;type&quot;:&quot;boolean&quot;}							
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
