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
          
  Step 4: Add both API requests in a test case using keywords Send Request and Verify
  Step 5: Save and Run
  
 ## Chaining API requests for a JSON response using Groovy scripts
 
   Step 1. Add the first API in Test case 
   Step2. Parameterize the value in 2nd API where extracted data needs to be referred
         Create a variable in variables section
         Refer the value in the request
          
          {
           "name": "${name}",
           "job": "Teacher"
          }
          
  Step 3. Refer the value of the local variable from a Global Variable
  Step 4. Add the 2nd API request in the test case
  Step 5. In Script section add groovy script to extract value from 1st API Response & store in Global Variable
   
   
 ## Chaining API requests for a XML response using Groovy scripts
 
   Step 1. Add the first API in Test case 
   Step2. Parameterize the value where extracted data needs to be referred
         Create a variable in variables section
         Refer the value in the request
          
          <tem:Arg1>${num1}</tem:Arg1>
          <tem:Arg2>10</tem:Arg2>
          
  Step 3. Refer the value of the local variable from a Global Variable
  Step 4. Add the API request in the test case
  Step 5. In Script section add groovy script to extract value from 1st API Response and store in Global Variable
  Step 6. Run and validate
    