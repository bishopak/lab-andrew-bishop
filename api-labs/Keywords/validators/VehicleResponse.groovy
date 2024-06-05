package validators
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import java.text.SimpleDateFormat

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import groovy.console.ui.SystemOutputInterceptor
import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import groovy.json.JsonSlurper
class VehicleResponse {
	/**
	 * Send request and verify status code
	 * @param request request object, must be an instance of RequestObject
	 * @param expectedStatusCode
	 * @return a boolean to indicate whether the response status code equals the expected one
	 */
	@Keyword
	def compare(String vehicleDataResponse, String vehicleDetailsResponse) {
		def data = new JsonSlurper().parseText(vehicleDataResponse)
		def details = new JsonSlurper().parseText(vehicleDetailsResponse)



		//easy matches
		validate(data.data.vehicle.year, details.year)
		validate(data.data.vehicle.make, details.make)
		validate(data.data.vehicle.model, details.model)
		validate(data.data.vehicle.transmission, details.transmission)
		validate(data.data.vehicle.odometer, details.odometer)
		validate(data.data.registration.registrationNumber, details.rego)
		validate(data.data.registration.state, details.state)

		data.data.registration.address.each {address ->
			registrationInList(address, details.addressModel)
		}
		
		details.addressModel.each {address ->
			registrationInList(address, data.data.registration.address)
		}
		
		data.data.owner.each {owner ->
			ownerInList(owner, details.owner)
		}
		
		details.owner.each{owner ->
			ownerInList(owner, data.data.owner)
		}
	}
	
	
	def registrationInList(Object record, ArrayList list) {
		def found = false
		
		list.each{l -> 
			def matched = (l.addressType == record.addressType
				&& l.state == record.state
				&& l.country == record.country)
			  
			if (record.containsKey("addressLine1")) {
				matched = (matched && l.addressLine1 == record.address1 && l.addressLine2 == record.address2) 
			} else {
				matched = (matched && record.addressLine1 == l.address1 && record.addressLine2 == l.address2)
			}
			
			if (matched) {
				if (found) {
					KeywordUtil.markFailed("expected: " + list.toString() + " to contain " + record.toString() + " only a single instace of this record")
				}
				found = true
			}
		}
		
		if (!found) {
			KeywordUtil.markFailed("expected: " + list.toString() + " to contain " + record.toString())
		}
	}
	
	def ownerInList(Object record, ArrayList list) {
		def found = false
		def dateFormat = new SimpleDateFormat("dd MMMMM yyyy")
		
		list.each{l ->
			def matched = (l.fullName == record.fullName && l.isCurrentOwner == record.isCurrentOwner)
			if (record.containsKey("driverLicense")) {
				matched = (matched && l.license == record.driverLicense && record.dateOfBirth ==  dateFormat.parse(l.dob).format("dd/MM/yyyy"))
			} else {
				matched = (matched && record.license == l.driverLicense && l.dateOfBirth ==  dateFormat.parse(record.dob).format("dd/MM/yyyy"))
			}
			
			if (matched) {
				if (found) {
					KeywordUtil.markFailed("expected: " + list.toString() + " to contain " + record.toString() + " only a single instace of this record")
				}
				found = true
			}
		}
		
		if (!found) {
			KeywordUtil.markFailed("expected: " + list.toString() + " to contain " + record.toString())
		}
	}
	
	def contains(ArrayList list, Object obj) {
		if (!list.contains(obj)) {
			KeywordUtil.markFailed("expected: " + list.toString() + " to contain " + obj.toString())
		}
	}

	//fudge: doing this to save some typing
	def validate(String a, String b) {
		if (a != b) {
			KeywordUtil.markFailed( a + " != " + b + ", expected match")
		}
	}
}