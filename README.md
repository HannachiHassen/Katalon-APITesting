# API Testing with Katalon Studio


## API Request-Response chaining

 . How to extract data from XML and JSON response
 . How to store extracted values in variables 
 . How to refer variables in other API requests
 . Hands-on demo with Groovy code and examples

### Extract a value from the response of one API
### Refer the extracted value in any subsequent API

  Step 1. Create a global variable where the extracted value will get stored
  Step 2. In the API request from where response value needs to be extracted - Goto Verifications tab Add the code to extract value and update in Global Variable
  
  GlobalVariable.FIRST_NAME = WS.getElementPropertyValue(response, 'data[4].first_name')
  
  The path of the value can be copied by hovering over the value in the response or click on the value in response and press Ctrl+K, the path will be added in the verification snippet.
  
  Step 3. In the 2nd API request, Parameterize the value where extracted data needs to be referred
          Create a variable in variables section with value coming from Global Variable crested in S Refer the value in the request
          
          {
           "name": "${name}",
           "job": "Teacher"
          }
          
  Step 4: Add both API requests in a test casse using keywords Send Request and Verify
  Step 5: Save and Run
  
  