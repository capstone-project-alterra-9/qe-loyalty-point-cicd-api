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

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_01'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_02'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_03'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_04'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_05'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_06'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_07'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_08'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_09'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

response = WS.sendRequest(findTestObject('API/Product/Product_Admin/TC_Add_Product_10'))

WS.verifyResponseStatusCode(response, GlobalVariable.status_code2)

