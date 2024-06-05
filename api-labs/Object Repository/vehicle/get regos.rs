<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get regos</name>
   <tag></tag>
   <elementGuidId>a2fcc6c0-5bc8-49b9-88a0-f408c2889b19</elementGuidId>
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
   <restUrl>${GlobalVariable.baseVehicleUrl}/data/regos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

assertThat(response.getStatusCode()).isEqualTo(200)

String jsonPass = &quot;&quot;&quot;
	{
		&quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
		&quot;data&quot;: {
			&quot;type&quot;:&quot;array&quot;,
			&quot;items&quot;: {
				&quot;type&quot;:&quot;string&quot;
			}
		}	
	}
	&quot;&quot;&quot;
	
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
assertThat(successful).isEqualTo(true);
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
