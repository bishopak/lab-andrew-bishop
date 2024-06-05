import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import java.util.Date
import groovy.json.JsonOutput

def payload = [:];

if (firstName != "") {
	payload["firstName"] = firstName;
}

if (lastName != "") {
	payload["lastName"] = lastName;
}

if (email != "") {
	def s = new Date().getTime().toString();
	System.out.println("lll -> " + email)
	payload["email"] = email.replace("#", s);
}

if (nationality != "") {
	payload["nationality"] = nationality;
}

if (dateOfBirth != "") {
	payload["dateOfBirth"] = dateOfBirth;
}

if (phoneNumber != "") {
	payload["mobileNumber"] = phoneNumber;
}

createStudentResponse = WS.sendRequestAndVerify(findTestObject('students/Create student', [('clientId') : GlobalVariable.userName
            , ('authToken') : "notvalid", ('payload') : JsonOutput.toJson(payload)]))

WS.verifyResponseStatusCode(createStudentResponse, expectedStatusCode.toInteger())